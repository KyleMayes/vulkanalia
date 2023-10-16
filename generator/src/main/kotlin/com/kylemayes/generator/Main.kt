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
import org.kohsuke.github.GitHub
import java.nio.file.Files
import java.nio.file.Path
import java.security.MessageDigest
import java.util.Base64
import kotlin.system.exitProcess

private val log = KotlinLogging.logger { /* */ }

fun main(args: Array<String>) = Generator()
    .subcommands(Check(), Index(), Update())
    .main(args)

data class GeneratorContext(
    val directory: Path,
    val github: GitHub,
)

class Generator : CliktCommand(help = "Manages generated Vulkan bindings") {
    private val directory by option(help = "Vulkanalia directory").required()
    private val username by option(help = "GitHub username")
    private val token by option(help = "GitHub personal access token")

    private val context by findOrSetObject {
        val directory = Path.of(directory).toAbsolutePath().normalize()
        if (username != null && token != null) {
            GeneratorContext(directory, GitHub.connect(username, token))
        } else {
            GeneratorContext(directory, GitHub.connectAnonymously())
        }
    }

    override fun run() {
        log.info { "Working in $directory" }
        if (context.github.isAnonymous) {
            log.info { "Acting as an anonymous GitHub user" }
        } else {
            log.info { "Acting as ${context.github.myself.login} (${context.github.myself.email})" }
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

        val xml = inputs.registry.local.lazy.value
        val registry = log.time("Parse Registry") { parseRegistry(xml) }

        // Generate

        val files = log.time("Generate Files") { generateRustFiles(registry) }

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

        val xml = inputs.registry.local.lazy.value
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

        val xmlVersion = if (skipUpgrade) { inputs.registry.local } else { inputs.registry.latest }
        val xml = xmlVersion.lazy.value
        val registry = log.time("Parse Registry") { parseRegistry(xml) }

        // Generate

        val files = log.time("Generate Files") { generateRustFiles(registry) }

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

        val commits = inputs.list
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
        git("config", "--local", "user.name", context.github.myself.login)
        git("config", "--local", "user.email", context.github.myself.email)
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
