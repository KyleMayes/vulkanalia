// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.support

import mu.KotlinLogging
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicReference
import kotlin.time.Duration.Companion.seconds

private val log = KotlinLogging.logger { /* */ }

/** Executes the `git` command and prints the output. */
fun git(vararg args: String) {
    println("> git ${args.joinToString(" ")}")
    println(execute("git", arrayOf(*args)))
}

/** Executes the `rustfmt` command and returns the output. */
fun rustfmt(rust: String): String = execute("rustfmt", emptyArray(), rust)

/** Executes a command (with a time limit) and returns the output. */
private fun execute(command: String, args: Array<String>, input: String? = null): String {
    val process = ProcessBuilder(command, *args).start()

    val errors = ConcurrentHashMap<String, Throwable>()
    val latch = CountDownLatch(4)
    val stdout = AtomicReference<String?>(null)
    val stderr = AtomicReference<String?>(null)

    fun operation(name: String, operation: () -> Unit) = Thread {
        try {
            Thread.currentThread().name = "$command-$name"
            log.slow("`$command`: $name", 2.5.seconds) { operation() }
        } catch (e: Throwable) {
            errors[name] = e
        } finally {
            latch.countDown()
        }
    }

    val threads = listOf(
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
        },
    )

    threads.forEach { it.start() }
    val countdown = latch.await(5, TimeUnit.SECONDS)

    val stdoutValue = stdout.get()
    val stderrValue = stderr.get()
    val errorsValue = errors.toMap()

    if (process.isAlive) {
        log.slow("`$command`: kill", 0.5.seconds) {
            try {
                process.destroy()
                process.waitFor(1, TimeUnit.SECONDS)
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
