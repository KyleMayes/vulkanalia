// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator

import org.kohsuke.github.GHCommit
import java.nio.file.Files

private val registryPath = RepositoryPath("KhronosGroup/Vulkan-Docs", "main", "xml/vk.xml")

/** The generator inputs pulled from GitHub repositories. */
data class RepositoryInputs(
    /** The Vulkan API registry. */
    val registry: RepositoryInput<String>,
) {
    val list = listOf(registry)

    /** Updates the locally tracked commits to match the latest commits. */
    fun updateLocal(context: GeneratorContext) {
        setLocalCommitHashes(context, list)
    }
}

/** Gets the generator inputs pulled from GitHub repositories. */
fun getRepositoryInputs(context: GeneratorContext): RepositoryInputs {
    val locals = getLocalCommitHashes(context)
    return RepositoryInputs(
        registry = getRepositoryInput(context, locals, registryPath, ::getFile),
    )
}

// ===============================================
// Input
// ===============================================

/** A reference to a file or directory in a GitHub repository. */
data class RepositoryPath(
    val name: String,
    val branch: String,
    val path: String,
)

/** A generator input pulled from a GitHub repository. */
data class RepositoryInput<out T>(
    val path: RepositoryPath,
    val local: RepositoryInputVersion<T>,
    val latest: RepositoryInputVersion<T>,
) {
    val stale: Boolean get() = local.commit.shA1 != latest.commit.shA1

    /** Gets the intermediate commits between the locally tracked and latest commits. */
    fun getIntermediateCommits() =
        local.commit.owner
            .queryCommits()
            .from(path.branch)
            .since(local.commit.commitDate)
            .until(latest.commit.commitDate)
            .list()
            .toList()
}

/** A version of a generator input pulled from a GitHub repository. */
data class RepositoryInputVersion<out T>(
    val commit: GHCommit,
    val lazy: Lazy<T>,
)

/** Gets a generator input pulled from a GitHub repository. */
private fun <T> getRepositoryInput(
    context: GeneratorContext,
    locals: Map<RepositoryPath, String>,
    path: RepositoryPath,
    get: (commit: GHCommit, path: RepositoryPath) -> T,
): RepositoryInput<T> {
    val repository = context.github.getRepository(path.name)
    val latest = repository.queryCommits().from(path.branch).path(path.path).pageSize(1).list().first()
    val local = locals[path]?.let { repository.getCommit(it) } ?: latest
    return RepositoryInput(
        path = path,
        local = RepositoryInputVersion(local, lazy { get(local, path) }),
        latest = RepositoryInputVersion(latest, lazy { get(latest, path) }),
    )
}

/** Fetches the contents of a file for a generator input pulled from a GitHub repository. */
private fun getFile(commit: GHCommit, path: RepositoryPath): String {
    val content = commit.owner.getFileContent(path.path, commit.shA1)
    return if (content.size <= 1024 * 1024 /* 1 MiB */) {
        content.read().readAllBytes().decodeToString()
    } else {
        commit.owner.getBlob(content.sha).read().readAllBytes().decodeToString()
    }
}

/** Fetches the contents of the files in a directory for a generator input pulled from a GitHub repository. */
private fun getDirectory(commit: GHCommit, path: RepositoryPath): Map<String, String> {
    return commit
        .owner
        .getDirectoryContent(path.path, commit.shA1)
        .associate { it.name to it.read().readAllBytes().decodeToString() }
}

// ===============================================
// Local
// ===============================================

/** Reads the locally tracked commit hashes for repository inputs. */
private fun getLocalCommitHashes(context: GeneratorContext): Map<RepositoryPath, String> =
    Files.readAllLines(context.directory.resolve(".commits"))
        .map { it.trim() }
        .filter { it.isNotEmpty() }
        .associate {
            val (key, hash) = it.split(" => ")
            val (user, name, branch, path) = key.split("/", limit = 4)
            RepositoryPath("$user/$name", branch, path) to hash
        }

/** Writes the locally tracked commit hashes for repository inputs. */
private fun setLocalCommitHashes(context: GeneratorContext, inputs: List<RepositoryInput<Any>>) =
    Files.writeString(
        context.directory.resolve(".commits"),
        inputs.joinToString {
            val (name, branch, path) = it.path
            "$name/$branch/$path => ${it.latest.commit.shA1}"
        },
    )
