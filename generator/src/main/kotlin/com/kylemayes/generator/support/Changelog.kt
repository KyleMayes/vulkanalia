package com.kylemayes.generator.support

import com.vladsch.flexmark.ast.BulletList
import com.vladsch.flexmark.ast.Heading
import com.vladsch.flexmark.parser.Parser
import java.lang.StringBuilder

/** A changelog. */
data class Changelog(
    val versions: MutableList<Version>,
)

/** A changelog version (e.g., `## [0.2.1] - 2020-11-04`). */
data class Version(
    val version: String,
    val release: String?,
    val sections: MutableList<Section>
)

/** A changelog version section (e.g., `### Added` or `### Fixed`). */
data class Section(
    val name: String,
    val changes: MutableList<String>,
)

/** Parses a changelog from a Markdown string. */
fun parseMarkdown(markdown: String): Changelog {
    val parser = Parser.builder().build()
    val document = parser.parse(markdown)

    val versions = mutableListOf<Version>()

    // Parse Versions

    val children = document.children.iterator()
    while (children.hasNext()) {
        val versionHeading = children.next() as Heading
        assert(versionHeading.level == 2)

        if (versionHeading.text.contains("[0.1.0]")) {
            break
        }

        // Parse Version

        val versionPieces = versionHeading.text.toString().trim().split(" - ")
        assert(versionPieces.size == 2)

        val version = versionPieces[0].removePrefix("[").removeSuffix("]")
        val release = if (versionPieces[1] == "UNRELEASED") { null } else { versionPieces[1] }

        // Parse Sections

        val sections = mutableListOf<Section>()

        while (children.peek() is Heading && (children.peek() as Heading).level == 3) {
            val sectionHeading = children.next() as Heading
            val sectionList = children.next() as BulletList

            // Parse Section

            val name = sectionHeading.text.toString().trim()
            val changes = sectionList.children
                .map { it.childChars.toString().trim() }
                .toMutableList()

            sections.add(Section(name, changes))
        }

        versions.add(Version(version, release, sections))
    }

    return Changelog(versions)
}

/** Generates a Markdown string for a changelog. */
fun Changelog.generateMarkdown(): String {
    val markdown = StringBuilder()

    for (version in versions) {
        markdown.append("## [${version.version}] - ${version.release ?: "UNRELEASED"}\n\n")

        for (section in version.sections) {
            markdown.append("### ${section.name}\n")

            for (change in section.changes) {
                markdown.append("- $change\n")
            }

            markdown.append("\n")
        }
    }

    markdown.append("## [0.1.0] - 2020-10-19\n- Initial release\n")

    return markdown.toString().replace("\n", System.lineSeparator())
}

/** Adds or extends a `Bindings Updates` section to this changelog. */
fun Changelog.addBindingsUpdates(commit: String, commitMessage: String) {
    // Add an unreleased version if the latest version has been released.

    if (versions[0].release != null) {
        val latest = versions[0].version.split(".").map { it.toInt() }
        val next = "${latest[0]}.${latest[1]}.${latest[2] + 1}"
        versions.add(0, Version(next, null, mutableListOf()))
    }

    // Add a `Bindings Update` section if it does not exist.

    if (versions[0].sections.getOrNull(0)?.name != "Bindings Updates") {
        versions[0].sections.add(0, Section("Bindings Updates", mutableListOf()))
    }

    // Add the change to the `Bindings Updates` section.

    val text = commitMessage.lines().first().removePrefix("Change log for ").removeSuffix(":")
    val change = "[$text](https://github.com/KhronosGroup/Vulkan-Docs/commit/$commit)"
    versions[0].sections[0].changes.add(change)
}
