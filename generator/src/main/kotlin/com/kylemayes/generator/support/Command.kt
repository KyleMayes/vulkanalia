// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.support

import mu.KotlinLogging
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicReference

private val log = KotlinLogging.logger { /* */ }

/** Executes the `bindgen` command. */
fun bindgen(vararg args: String) {
    execute("bindgen", arrayOf(*args))
}

/** Executes the `git` command and prints the output. */
fun git(vararg args: String) {
    println("> git ${args.joinToString(" ")}")
    println(execute("git", arrayOf(*args)))
}

/** Executes the `rustfmt` command and returns the output. */
fun rustfmt(rust: String) = execute("rustfmt", emptyArray(), rust)

/** Executes a command in a new thread (with a time limit) and returns the output. */
private fun execute(command: String, args: Array<String>, input: String? = null): String {
    val process = ProcessBuilder(command, *args).start()

    val stdout = AtomicReference<String?>(null)
    val stderr = AtomicReference<String?>(null)
    val error = AtomicReference<Throwable?>(null)
    val latch = CountDownLatch(1)

    val thread = Thread {
        try {
            Thread.currentThread().name = "$command-thread"

            if (input != null) {
                log.slow("Writing '$command' stdin", 1000) { process.outputStream.write(input.toByteArray()) }
                process.outputStream.close()
            }

            log.slow("Waiting for '$command'", 5000) { process.waitFor() }

            val stdoutBytes = log.slow("Reading '$command' stdout", 1000) { process.inputStream.readBytes() }
            stdout.set(String(stdoutBytes))
            process.inputStream.close()

            val stderrBytes = log.slow("Reading '$command' stderr", 1000) { process.errorStream.readBytes() }
            stderr.set(String(stderrBytes))
            process.errorStream.close()

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
    latch.await(10, TimeUnit.SECONDS)

    val stdoutValue = stdout.get()
    val stderrValue = stderr.get()
    val errorValue = error.get()
    if (stdoutValue == null || errorValue != null) {
        process.destroy()
        process.waitFor(1, TimeUnit.SECONDS)
        process.destroyForcibly()
        val executed = "$command ${args.joinToString(" ")}".trim()
        val message = "Failed to execute command ('$executed'):\n\n[stdout]=\n$stdoutValue\n\n[stderr]=\n$stderrValue"
        throw RuntimeException(message, errorValue ?: RuntimeException("Timed out."))
    }

    return stdoutValue
}
