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
import com.kylemayes.generator.support.git
import com.kylemayes.generator.support.parseMarkdown
import com.kylemayes.generator.support.time
import mu.KotlinLogging
import org.kohsuke.github.GHCommit
import org.kohsuke.github.GitHub
import java.net.URI
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse.BodyHandlers
import java.nio.file.Files
import java.nio.file.Path
import kotlin.system.exitProcess

private val log = KotlinLogging.logger { /* */ }

fun main(args: Array<String>) = Generator()
    .subcommands(Check(), Index(), Update())
    .main(args)

class Generator : CliktCommand(help = "Manages generated Vulkan bindings") {
    private val directory by option(help = "Vulkanalia directory").required()
    private val username by option(help = "GitHub username")
    private val token by option(help = "GitHub personal access token")

    private val context by findOrSetObject {
        val directory = Path.of(directory).toAbsolutePath().normalize()
        if (username != null && token != null) {
            Triple(directory, GitHub.connect(username, token), false)
        } else {
            Triple(directory, GitHub.connectAnonymously(), true)
        }
    }

    override fun run() {
        val (directory, github, anonymous) = context
        log.info { "Working in $directory" }
        if (anonymous) {
            log.info { "Acting as an anonymous GitHub user" }
        } else {
            log.info { "Acting as ${github.myself.login} (${github.myself.email})" }
        }
    }
}

// ===============================================
// Check
// ===============================================

class Check : CliktCommand(help = "Checks generated Vulkan bindings") {
    private val context by requireObject<Triple<Path, GitHub, Boolean>>()

    override fun run() {
        val (directory, _, _) = context

        // Parse

        val currentCommitHash = getCurrentCommitHash(directory)
        log.info { "Current commit hash = $currentCommitHash" }

        val xml = getFileContents(currentCommitHash, "xml/vk.xml")
        val registry = log.time("Parse Registry") { parseRegistry(xml) }

        // Generate

        val files = log.time("Generate Files") { generateRustFiles(registry) }

        // Check

        if (!log.time("Format Files") { files.all { it.format() } }) {
            log.error { "One or more files could not be formatted." }
            exitProcess(1)
        }

        if (!log.time("Match Files") { files.all { it.matches(directory) } }) {
            log.error { "One or more files did not match what is on disk." }
            exitProcess(1)
        }
    }
}

// ===============================================
// Index
// ===============================================

class Index : CliktCommand(help = "Generates an index for generated Vulkan bindings") {
    private val context by requireObject<Triple<Path, GitHub, Boolean>>()

    override fun run() {
        val (directory, _, _) = context

        // Parse

        val currentCommitHash = getCurrentCommitHash(directory)
        log.info { "Current commit hash = $currentCommitHash" }

        val xml = getFileContents(currentCommitHash, "xml/vk.xml")
        val registry = log.time("Parse Registry") { parseRegistry(xml) }

        // Index

        log.info { "Generating index..." }
        val index = registry.indexEntities()
        Files.writeString(directory.resolve("index.txt"), index)
    }
}

// ===============================================
// Update
// ===============================================

class Update : CliktCommand(help = "Updates generated Vulkan bindings") {
    private val context by requireObject<Triple<Path, GitHub, Boolean>>()

    private val force by option(help = "Force update of generated Vulkan bindings").flag()
    private val repo by option(help = "GitHub repository to publish changes to")
    private val skipUpgrade by option(help = "Skip upgrading commit").flag()

    override fun run() {
        val (directory, github, anonymous) = context

        // Parse

        val currentCommit = getCommit(github, getCurrentCommitHash(directory))
        log.info { "Current commit hash = ${currentCommit.shA1}" }

        val latestCommit = if (!skipUpgrade) {
            val latestCommit = getLatestCommit(github, "xml/vk.xml")
            log.info { "Latest commit hash = ${latestCommit.shA1}" }
            latestCommit
        } else {
            currentCommit
        }

        if (!force && currentCommit.shA1 == latestCommit.shA1) {
            log.info { "Nothing to update." }
            return
        }

        val xml = getFileContents(latestCommit.shA1, "xml/vk.xml")
        val registry = log.time("Parse Registry") { parseRegistry(xml) }

        // Generate

        val files = log.time("Generate Files") { generateRustFiles(registry) }

        // Check

        val format = log.time("Format Files") { files.all { it.format() } }

        if (log.time("Match Files") { files.all { it.matches(directory) } }) {
            log.info { "All files match what is on disk." }
            return
        }

        // Write (files)

        log.time("Write Files") { files.forEach { it.write(directory) } }
        setCurrentCommitHash(directory, latestCommit.shA1)

        if (!format) {
            log.error { "One or more files could not be formatted." }
            exitProcess(1)
        }

        // Write (changelog)

        val markdown = directory.resolve("CHANGELOG.md")
        val changelog = parseMarkdown(Files.readString(markdown))

        for (commit in getTrailingCommits(github, "xml/vk.xml", currentCommit).reversed()) {
            log.info { "Intermediate commit hash = ${commit.shA1}" }
            changelog.addBindingsUpdates(commit.shA1, commit.commitShortInfo.message)
        }

        Files.writeString(markdown, changelog.generateMarkdown())

        // Publish

        if (repo == null) {
            return
        }

        log.info { "Publishing changes ($repo)..." }

        if (anonymous) {
            log.error { "Cannot publish changes while acting as an anonymous GitHub user." }
            exitProcess(1)
        }

        val repo = github.getRepository(repo)
        val head = "vk-${latestCommit.shA1}"
        val base = "master"

        val existing = repo.queryPullRequests()
            .head(head)
            .base(base)
            .list()
            .firstOrNull()
        if (existing != null) {
            log.info { "Pull request already exists (#${existing.number})!" }
            return
        }

        log.info { "Creating branch, committing changes, and pushing branch..." }
        git("config", "--local", "user.name", github.myself.login)
        git("config", "--local", "user.email", github.myself.email)
        git("checkout", "-B", head)
        git("add", "-A")
        git("status")
        git("commit", "-m", "Update generated bindings")
        git("log", "-2")
        git("push", "origin", head, "--force")

        log.info { "Creating pull request..." }
        val body = "Update generated bindings (automatically created by update action)."
        val pr = repo.createPullRequest("Update generated bindings", head, base, body)
        log.info { "Created pull request (#${pr.number})!" }
    }
}

// ===============================================
// Shared
// ===============================================

/** The repository Vulkan files are pulled from. */
private const val repository = "KhronosGroup/Vulkan-Docs"
/** The branch Vulkan files are pulled from. */
private const val branch = "main"
/** The name of the file the current commit hash is stored in. */
private const val file = "Vulkan-Docs"

/** Gets the current `Vulkan-Docs` commit hash from the `Vulkan-Docs` file. */
fun getCurrentCommitHash(directory: Path): String =
    Files.readAllLines(directory.resolve(file))
        .map { it.trim() }
        .first { !it.startsWith('#') }

/** Sets the current `Vulkan-Docs` commit hash to the `Vulkan-Docs` file. */
fun setCurrentCommitHash(directory: Path, hash: String): Path =
    Files.writeString(directory.resolve(file), "# https://github.com/$repository\n$hash\n")

/** Gets a commit for a `Vulkan-Docs` file from GitHub. */
fun getCommit(github: GitHub, hash: String): GHCommit =
    github.getRepository(repository)
        .getCommit(hash)

/** Gets the latest commit for a `Vulkan-Docs` file from GitHub. */
fun getLatestCommit(github: GitHub, path: String): GHCommit =
    github.getRepository(repository)
        .queryCommits()
        .from(branch)
        .path(path)
        .pageSize(1)
        .list()
        .first()

/** Gets the commits after a commit for a `Vulkan-Docs` file from GitHub. */
fun getTrailingCommits(github: GitHub, path: String, commit: GHCommit): List<GHCommit> =
    github.getRepository(repository)
        .queryCommits()
        .from(branch)
        .path(path)
        .since(commit.commitDate)
        .list()
        .toList()

/** Gets the contents of a `Vulkan-Docs` file from GitHub. */
fun getFileContents(hash: String, path: String): String {
    val uri = "https://raw.githubusercontent.com/$repository/$hash/$path"
    val request = HttpRequest.newBuilder(URI.create(uri)).GET().build()

    val client = HttpClient.newHttpClient()
    val response = client.send(request, BodyHandlers.ofString())

    if (response.statusCode() == 200) {
        return response.body()
    } else {
        error("Failed to get `$repository/$path` from GitHub (${response.statusCode()}).")
    }
}
