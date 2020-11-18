package com.kylemayes.generator.support

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource
import java.nio.file.Files
import java.nio.file.Path

class ChangelogTest {
    private fun load(path: String) =
        this::class.java.classLoader.getResource("changelog/$path")!!.readText()

    @Test
    fun `Round Trip`() {
        val markdown = Files.readString(Path.of("../CHANGELOG.md"))
        val changelog = parseMarkdown(markdown)
        assertEquals(markdown, changelog.generateMarkdown())
    }

    @ParameterizedTest
    @CsvSource("released-bindings", "released-no-bindings", "unreleased-bindings", "unreleased-no-bindings")
    fun `Add Bindings Updates`(name: String) {
        val changelog = parseMarkdown(load("initial/$name.md"))
        changelog.addBindingsUpdates("f90136facacd25f016e523064f03713bdfe1b22d", load("message.txt"))
        assertEquals(load("expected/$name.md"), changelog.generateMarkdown())
    }
}
