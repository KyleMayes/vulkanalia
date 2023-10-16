// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.support

import mu.KotlinLogging
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

/** Executes a command in a new thread (with a time limit) and returns the output. */
private fun execute(command: String, args: Array<String>, input: String? = null): String {
    val process = ProcessBuilder(command, *args).start()

    val output = AtomicReference<String?>(null)
    val error = AtomicReference<Throwable?>(null)
    val latch = CountDownLatch(1)

    val thread = Thread {
        try {
            Thread.currentThread().name = "$command-thread"

            if (input != null) {
                log.slow("`$command` process stdin", 1.seconds) { process.outputStream.write(input.toByteArray()) }
                process.outputStream.close()
            }

            val outputBytes = log.slow("`$command` process stdin", 1.seconds) { process.inputStream.readAllBytes() }
            output.set(String(outputBytes))
            process.inputStream.close()

            log.slow("`$command` process", 2.5.seconds) { process.waitFor() }

            if (process.exitValue() != 0) {
                error("Non-zero exit code (${process.exitValue()}).")
            }
        } catch (e: Throwable) {
            error.set(e)
        } finally {
            latch.countDown()
        }
    }

    thread.start()
    latch.await(5, TimeUnit.SECONDS)

    val outputValue = output.get()
    val errorValue = error.get()
    if (outputValue == null || errorValue != null) {
        process.destroy()
        process.waitFor(1, TimeUnit.SECONDS)
        process.destroyForcibly()
        val executed = "$command ${args.joinToString(" ")}".trim()
        val message = "Failed to execute command ('$executed')."
        throw RuntimeException(message, errorValue ?: RuntimeException("Timed out."))
    }

    return outputValue
}
