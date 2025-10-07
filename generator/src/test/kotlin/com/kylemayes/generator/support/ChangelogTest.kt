package com.kylemayes.generator.support

import org.apache.commons.text.StringEscapeUtils
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource
import org.kohsuke.github.GHCommit
import org.kohsuke.github.GHCommit.ShortInfo
import org.kohsuke.github.GHRepository
import org.mockito.Mockito
import java.nio.file.Files
import java.nio.file.Path

class ChangelogTest {
    private fun load(path: String) = this::class.java.classLoader.getResource("changelog/$path")!!.readText()

    @Test
    fun `Round Trip`() {
        val markdown = Files.readString(Path.of("../CHANGELOG.md"))
        val changelog = parseMarkdown(markdown)
        assertEscapeEquals(markdown, changelog.generateMarkdown())
    }

    @ParameterizedTest
    @CsvSource("released-bindings", "released-no-bindings", "unreleased-bindings", "unreleased-no-bindings")
    fun `Add Bindings Updates`(name: String) {
        val commit = Mockito.mock(GHCommit::class.java)
        Mockito.`when`(commit.shA1).thenReturn("f90136facacd25f016e523064f03713bdfe1b22d")

        val owner = Mockito.mock(GHRepository::class.java)
        Mockito.`when`(commit.owner).thenReturn(owner)
        Mockito.`when`(owner.fullName).thenReturn("KhronosGroup/Vulkan-Docs")

        val commitShortInfo = Mockito.mock(ShortInfo::class.java)
        Mockito.`when`(commit.commitShortInfo).thenReturn(commitShortInfo)
        Mockito.`when`(commitShortInfo.message).thenReturn(load("message.txt"))

        val changelog = parseMarkdown(load("initial/$name.md"))
        changelog.addBindingsUpdates(commit)
        assertEscapeEquals(load("expected/$name.md"), changelog.generateMarkdown())
    }

    private fun assertEscapeEquals(
        expected: String,
        actual: String,
    ) {
        val expectedEscape = StringEscapeUtils.escapeJava(expected.replace(Regex("\r?\n"), "\n"))
        val actualEscape = StringEscapeUtils.escapeJava(actual.replace(Regex("\r?\n"), "\n"))
        assertEquals(expectedEscape, actualEscape)
    }
}
