// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.support

import mu.KotlinLogging
import java.nio.file.Path
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicReference
import kotlin.time.Duration.Companion.seconds

private val log = KotlinLogging.logger { /* */ }

private val waitTimeout = 30.seconds
private val killTimeout = 5.seconds

/** Executes the `bindgen` command and returns the output. */
fun bindgen(vararg args: String): String = execute("bindgen", arrayOf(*args))

/** Executes the `git` command and prints the output. */
fun git(
    vararg args: String,
    directory: Path? = null,
) {
    println("> git ${args.joinToString(" ")}")
    println(execute("git", arrayOf(*args), directory = directory))
}

/** Executes the `rustfmt` command and returns the output. */
fun rustfmt(rust: String): String = execute("rustfmt", arrayOf("--edition", "2021"), input = rust)

/** Executes a command (with a time limit) and returns the output. */
private fun execute(
    command: String,
    args: Array<String>,
    input: String? = null,
    directory: Path? = null,
): String {
    var builder = ProcessBuilder(command, *args)
    if (directory != null) builder = builder.directory(directory.toFile())
    val process = builder.start()

    val errors = ConcurrentHashMap<String, Throwable>()
    val latch = CountDownLatch(4)
    val stdout = AtomicReference<String?>(null)
    val stderr = AtomicReference<String?>(null)

    fun operation(
        name: String,
        operation: () -> Unit,
    ) = Thread {
        try {
            Thread.currentThread().name = "$command-$name"
            log.slow("`$command`: $name", waitTimeout / 2) { operation() }
        } catch (e: Throwable) {
            errors[name] = e
        } finally {
            latch.countDown()
        }
    }

    val threads =
        listOf(
            operation("write stdin") {
                if (input != null) process.outputStream.write(input.toByteArray())
                process.outputStream.flush()
                process.outputStream.close()
            },
            operation("read stdout") {
                stdout.set(String(process.inputStream.readAllBytes()))
                process.inputStream.close()
            },
            operation("read stderr") {
                stderr.set(String(process.errorStream.readAllBytes()))
                process.errorStream.close()
            },
            operation("wait") {
                process.waitFor()
                if (process.exitValue() != 0) {
                    error("Non-zero exit code (${process.exitValue()}).")
                }
            },
        )

    threads.forEach { it.start() }
    val countdown = latch.await(waitTimeout.inWholeMilliseconds, TimeUnit.MILLISECONDS)

    val stdoutValue = stdout.get()
    val stderrValue = stderr.get()
    val errorsValue = errors.toMap()

    if (process.isAlive) {
        log.slow("`$command`: kill", killTimeout / 2) {
            try {
                process.destroy()
                process.waitFor(killTimeout.inWholeMilliseconds, TimeUnit.MILLISECONDS)
                process.destroyForcibly()
            } catch (e: Error) {
                e.printStackTrace()
            }
        }
    }

    if (stdoutValue == null || errorsValue.isNotEmpty()) {
        val executed = "$command ${args.joinToString(" ")}".trim()
        val message = "Failed to execute command ('$executed'):\n\n[stdout]=\n$stdoutValue\n\n[stderr]=\n$stderrValue"
        val error = RuntimeException(message)
        if (!countdown) error.addSuppressed(RuntimeException("Timed out."))
        errorsValue.forEach { error.addSuppressed(RuntimeException(it.key, it.value)) }
        throw error
    }

    return stdoutValue
}
