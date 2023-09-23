package com.kylemayes.generator.support

import org.kohsuke.github.GHCommit
import org.kohsuke.github.GitHub
import java.net.URI
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse.BodyHandlers
import java.nio.file.Files
import java.nio.file.Path

/** A GitHub repository and associated branch. */
data class Repository(val user: String, val name: String, val branch: String) {
    /** Gets the current commit hash for this repository. */
    fun getCurrentCommitHash(directory: Path): String =
        Files.readAllLines(directory.resolve(name))
            .map { it.trim() }
            .first { !it.startsWith('#') }

    /** Sets the current commit hash for this repository. */
    fun setCurrentCommitHash(directory: Path, hash: String): Path =
        Files.writeString(directory.resolve(name), "# https://github.com/$user/$name\n$hash\b")

    /** Gets a commit for this repository from GitHub. */
    fun getCommit(github: GitHub, hash: String): GHCommit =
        github.getRepository("$user/$name")
            .getCommit(hash)

    /** Gets the latest commit for a file in this repository from GitHub. */
    fun getLatestCommit(github: GitHub, path: String): GHCommit =
        github.getRepository("$user/$name")
            .queryCommits()
            .from(branch)
            .path(path)
            .pageSize(1)
            .list()
            .first()

    /** Gets the commits after a commit for a file in this repository from GitHub. */
    fun getTrailingCommits(github: GitHub, path: String, commit: GHCommit): List<GHCommit> =
        github.getRepository("$user/$name")
            .queryCommits()
            .from(branch)
            .path(path)
            .since(commit.commitDate)
            .list()
            .toList()

    /** Gets the contents of a `Vulkan-Docs` file from GitHub. */
    fun getFileContents(hash: String, path: String): String {
        val uri = "https://raw.githubusercontent.com/$user/$name/$hash/$path"
        val request = HttpRequest.newBuilder(URI.create(uri)).GET().build()

        val client = HttpClient.newHttpClient()
        val response = client.send(request, BodyHandlers.ofString())

        if (response.statusCode() == 200) {
            return response.body()
        } else {
            error("Failed to get `$user/$name/$path` from GitHub (${response.statusCode()}).")
        }
    }
}
