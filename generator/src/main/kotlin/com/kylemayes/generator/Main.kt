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
import com.kylemayes.generator.registry.parseRegistry
import com.kylemayes.generator.support.git
import com.kylemayes.generator.support.time
import mu.KotlinLogging
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
    .subcommands(Check(), Update())
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

        val currentCommit = getCurrentCommit(directory)
        log.info { "Current commit = $currentCommit" }

        val xml = getFileContents(currentCommit, "xml/vk.xml")
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
// Update
// ===============================================

class Update : CliktCommand(help = "Updates generated Vulkan bindings") {
    private val context by requireObject<Triple<Path, GitHub, Boolean>>()

    private val force by option(help = "Force update of generated Vulkan bindings").flag()
    private val repo by option(help = "GitHub repository to publish changes to")

    override fun run() {
        val (directory, github, anonymous) = context

        // Parse

        val currentCommit = getCurrentCommit(directory)
        log.info { "Current commit = $currentCommit" }

        val latestCommit = getLatestCommit(github, "xml/vk.xml")
        log.info { "Latest commit = $latestCommit" }

        if (!force && currentCommit == latestCommit) {
            log.info { "Nothing to update." }
            return
        }

        val xml = getFileContents(latestCommit, "xml/vk.xml")
        val registry = log.time("Parse Registry") { parseRegistry(xml) }

        // Generate

        val files = log.time("Generate Files") { generateRustFiles(registry) }

        // Check

        val format = log.time("Format Files") { files.all { it.format() } }

        if (log.time("Match Files") { files.all { it.matches(directory) } }) {
            log.info { "All files match what is on disk." }
            return
        }

        // Write

        log.time("Write Files") { files.forEach { it.write(directory) } }
        setCurrentCommit(directory, latestCommit)

        if (!format) {
            log.error { "One or more files could not be formatted." }
            exitProcess(1)
        }

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
        val head = "vk-$latestCommit"
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

/** Gets the current `Vulkan-Docs` commit from the `Vulkan-Docs` file. */
fun getCurrentCommit(directory: Path): String =
    Files.readAllLines(directory.resolve("Vulkan-Docs"))
        .map { it.trim() }
        .first { !it.startsWith('#') }

/** Sets the current `Vulkan-Docs` commit to the `Vulkan-Docs` file. */
fun setCurrentCommit(directory: Path, commit: String): Path =
    Files.writeString(
        directory.resolve("Vulkan-Docs"),
        "# https://github.com/KhronosGroup/Vulkan-Docs\n$commit\n"
    )

/** Gets the latest commit for a `Vulkan-Docs` file from GitHub. */
fun getLatestCommit(github: GitHub, path: String): String =
    github.getRepository("KhronosGroup/Vulkan-Docs")
        .queryCommits()
        .from("main")
        .path(path)
        .pageSize(1)
        .list()
        .first()
        .shA1

/** Gets the contents of a `Vulkan-Docs` file from GitHub. */
fun getFileContents(commit: String, path: String): String {
    val uri = "https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/$commit/$path"
    val request = HttpRequest.newBuilder(URI.create(uri)).GET().build()

    val client = HttpClient.newHttpClient()
    val response = client.send(request, BodyHandlers.ofString())

    if (response.statusCode() == 200) {
        return response.body()
    } else {
        error("Failed to get `Vulkan-Docs/$path` from GitHub (${response.statusCode()}).")
    }
}
