// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator

import com.github.ajalt.clikt.core.CliktCommand
import com.github.ajalt.clikt.core.findOrSetObject
import com.github.ajalt.clikt.core.requireObject
import com.github.ajalt.clikt.core.subcommands
import com.github.ajalt.clikt.parameters.options.flag
import com.github.ajalt.clikt.parameters.options.option
import com.github.ajalt.clikt.parameters.options.required
import com.kylemayes.generator.generate.generateRustFiles
import com.kylemayes.generator.registry.indexEntities
import com.kylemayes.generator.registry.parseRegistry
import com.kylemayes.generator.support.addBindingsUpdates
import com.kylemayes.generator.support.generateMarkdown
import com.kylemayes.generator.support.parseMarkdown
import com.kylemayes.generator.support.time
import mu.KotlinLogging
import org.eclipse.jgit.api.Git
import org.eclipse.jgit.lib.PersonIdent
import org.eclipse.jgit.storage.file.FileRepositoryBuilder
import org.eclipse.jgit.transport.RefSpec
import org.eclipse.jgit.transport.RemoteRefUpdate
import org.eclipse.jgit.transport.UsernamePasswordCredentialsProvider
import org.kohsuke.github.GitHub
import java.io.File
import java.nio.file.Files
import java.nio.file.Path
import java.security.MessageDigest
import java.util.Base64
import kotlin.system.exitProcess

private val log = KotlinLogging.logger { /* */ }

fun main(args: Array<String>) =
    Generator()
        .subcommands(Check(), Index(), Update())
        .main(args)

data class GeneratorContext(
    val directory: Path,
    val github: GitHub,
    val token: String?,
)

class Generator : CliktCommand(help = "Manages generated Vulkan bindings") {
    private val directory by option(help = "Vulkanalia directory").required()
    private val token by option(help = "GitHub personal access token")
    private val tokenFile by option(help = "GitHub personal access token file")

    private val context by findOrSetObject {
        val directory = Path.of(directory).toAbsolutePath().normalize()

        val token =
            if (this.token != null) {
                this.token
            } else if (tokenFile != null) {
                File(tokenFile!!).readText().trim()
            } else {
                null
            }

        if (token != null) {
            GeneratorContext(directory, GitHub.connectUsingOAuth(token), token)
        } else {
            GeneratorContext(directory, GitHub.connectAnonymously(), null)
        }
    }

    override fun run() {
        log.info { "Working in $directory" }
        if (context.github.isAnonymous) {
            log.info { "Acting as an anonymous GitHub user" }
        } else {
            log.info { "Acting with GitHub OAuth token" }
        }
    }
}

// ===============================================
// Check
// ===============================================

class Check : CliktCommand(help = "Checks generated Vulkan bindings") {
    private val context by requireObject<GeneratorContext>()

    override fun run() {
        val inputs = getRepositoryInputs(context)

        // Parse

        val xml = log.time("Fetch Registry") { inputs.registry.local.lazy.value }
        val registry = log.time("Parse Registry") { parseRegistry(xml) }

        // Headers (video)

        val video = log.time("Fetch Video Headers") { inputs.video.local.lazy.value }

        // Generate

        val files = log.time("Generate Files") { generateRustFiles(registry, video) }

        // Check

        if (!log.time("Format Files") { files.all { it.format() } }) {
            log.error { "One or more files could not be formatted." }
            exitProcess(1)
        }

        if (!log.time("Match Files") { files.all { it.matches(context.directory) } }) {
            log.error { "One or more files did not match what is on disk." }
            exitProcess(1)
        }
    }
}

// ===============================================
// Index
// ===============================================

class Index : CliktCommand(help = "Generates an index for generated Vulkan bindings") {
    private val context by requireObject<GeneratorContext>()

    override fun run() {
        val inputs = getRepositoryInputs(context)

        // Parse

        val xml = log.time("Fetch Registry") { inputs.registry.local.lazy.value }
        val registry = log.time("Parse Registry") { parseRegistry(xml) }

        // Index

        log.info { "Generating index..." }
        val index = registry.indexEntities()
        log.info { "Generated index has ${index.count { it == '\n' } + 1} entries." }
        Files.writeString(context.directory.resolve("index.txt"), index)
    }
}

// ===============================================
// Update
// ===============================================

class Update : CliktCommand(help = "Updates generated Vulkan bindings") {
    private val context by requireObject<GeneratorContext>()

    private val force by option(help = "Force update of generated Vulkan bindings").flag()
    private val repo by option(help = "GitHub repository to publish changes to")
    private val skipUpgrade by option(help = "Skip upgrading commit").flag()

    override fun run() {
        val inputs = getRepositoryInputs(context)

        if (!force && !inputs.list.any { it.stale }) {
            log.info { "Nothing to update." }
            return
        }

        // Parse

        val xmlVersion = if (skipUpgrade) inputs.registry.local else inputs.registry.latest
        val xml = log.time("Fetch Registry") { xmlVersion.lazy.value }
        val registry = log.time("Parse Registry") { parseRegistry(xml) }

        // Headers (video)

        val videoVersion = if (skipUpgrade) inputs.video.local else inputs.video.latest
        val video = log.time("Fetch Video Headers") { videoVersion.lazy.value }

        // Generate

        val files = log.time("Generate Files") { generateRustFiles(registry, video) }

        // Check

        val format = log.time("Format Files") { files.all { it.format() } }

        if (log.time("Match Files") { files.all { it.matches(context.directory) } }) {
            log.info { "All files match what is on disk." }
            return
        }

        // Write (files)

        log.time("Write Files") { files.forEach { it.write(context.directory) } }

        if (!format) {
            log.error { "One or more files could not be formatted." }
            exitProcess(1)
        }

        if (skipUpgrade) {
            return
        }

        // Write (changelog)

        val markdown = context.directory.resolve("CHANGELOG.md")
        val changelog = parseMarkdown(Files.readString(markdown))

        val commits =
            inputs.list
                .flatMap { it.getIntermediateCommits() }
                .toSet()
                .sortedBy { it.commitDate }

        for (commit in commits) {
            log.info { "Intermediate commit hash = ${commit.shA1}" }
            changelog.addBindingsUpdates(commit)
        }

        Files.writeString(markdown, changelog.generateMarkdown())

        // Write (local)

        inputs.updateLocal(context)

        // Publish

        if (repo == null) {
            return
        }

        log.info { "Publishing changes ($repo)..." }

        if (context.github.isAnonymous) {
            log.error { "Cannot publish changes while acting as an anonymous GitHub user." }
            exitProcess(1)
        }

        val digest = MessageDigest.getInstance("SHA-256")
        val id = commits.joinToString(":") { it.shA1 }
        val hash = Base64.getEncoder().encodeToString(digest.digest(id.toByteArray()))

        val repo = context.github.getRepository(repo)
        val head = "vk-$hash"
        val base = "master"

        val existing =
            repo.queryPullRequests()
                .head(head)
                .base(base)
                .list()
                .firstOrNull()
        if (existing != null) {
            log.info { "Pull request already exists (#${existing.number})!" }
            return
        }

        log.info { "Creating branch, committing changes, and pushing branch..." }
        val git = Git(FileRepositoryBuilder.create(context.directory.resolve(".git").toFile()))

        log.time("Git Checkout") {
            git.checkout()
                .setCreateBranch(true)
                .setName(head).call()
        }

        log.time("Git Add") {
            git.add()
                .addFilepattern(".commits")
                .addFilepattern("CHANGELOG.md")
                .addFilepattern("vulkanalia")
                .addFilepattern("vulkanalia-sys")
                .call()
        }

        log.time("Git Commit") {
            git.commit()
                .setAuthor(PersonIdent("KyleMayes", "kyle@mayeses.com"))
                .setMessage("Update generated bindings")
                .call()
        }

        val updates =
            log.time("Git Push") {
                git.push()
                    .setCredentialsProvider(UsernamePasswordCredentialsProvider(context.token, ""))
                    .setForce(true)
                    .setRemote("origin")
                    .setRefSpecs(RefSpec("$head:$head"))
                    .call()
                    .flatMap { it.remoteUpdates }
            }

        if (updates.any { it.status != RemoteRefUpdate.Status.OK }) {
            error("Failed to push branch: $updates")
        }

        log.info { "Creating pull request..." }
        val body = "Update generated bindings (automatically created by update action)."
        val pr = repo.createPullRequest("Update generated bindings", head, base, body)
        log.info { "Created pull request (#${pr.number})!" }
    }
}
