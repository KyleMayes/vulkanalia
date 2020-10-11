// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.support

import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicReference

/** Executes the `git` command and prints the output. */
fun git(vararg args: String) {
    println("> git ${args.joinToString(" ")}")
    println(execute("git", arrayOf(*args)))
}

/** Executes the `rustfmt` command and returns the output. */
fun rustfmt(rust: String) = execute("rustfmt", emptyArray(), rust)

/** Executes a command in a new thread (with a time limit) and returns the output. */
private fun execute(command: String, args: Array<String>, input: String = ""): String {
    val process = ProcessBuilder(command, *args).start()

    val output = AtomicReference<String?>(null)
    val error = AtomicReference<Throwable?>(null)
    val latch = CountDownLatch(1)

    val thread = Thread {
        try {
            Thread.currentThread().name = "$command-thread"

            process.outputStream.write(input.toByteArray())
            process.outputStream.close()

            output.set(String(process.inputStream.readAllBytes()))
            process.inputStream.close()

            process.waitFor()

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
