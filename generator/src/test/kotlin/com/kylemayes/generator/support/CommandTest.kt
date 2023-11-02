package com.kylemayes.generator.support

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.io.TempDir
import java.io.File
import java.nio.file.Files

class CommandTest {
    @field:TempDir
    lateinit var temp: File

    @Test
    fun git() {
        val directory = temp.toPath()
        val file = directory.resolve("file.txt")

        git("init", directory = directory)

        git("add", "-A", directory = directory)
        git("status", directory = directory)

        Files.writeString(file, "Text.")
        git("add", "-A", directory = directory)
        git("status", directory = directory)
    }
}
