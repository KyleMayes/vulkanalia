// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.support

import mu.KLogger
import java.util.concurrent.atomic.AtomicBoolean

/** Times a block of code. */
fun <T> KLogger.time(name: String, block: () -> T): T {
    info { "[BEGIN] $name" }
    val start = System.nanoTime()
    val result = block()
    val elapsed = "%.3f".format((System.nanoTime() - start) / 1_000_000_000.0)
    info { "[ END ] $name (${elapsed}s)" }
    return result
}

/** Prints a message if a block of code is slow. */
fun <T> KLogger.slow(name: String, millis: Long, block: () -> T): T {
    val done = AtomicBoolean(false)

    val thread = Thread {
        try {
            Thread.sleep(millis)
            if (!done.get()) {
                warn { "$name is taking a while (>${millis}ms)..." }
            }
        } catch (_: InterruptedException) {}
    }

    thread.start()

    val result = block()
    done.set(true)
    thread.interrupt()

    return result
}
