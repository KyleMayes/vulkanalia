// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.support

import mu.KLogger
import java.util.concurrent.atomic.AtomicBoolean
import kotlin.time.Duration

/** Times a block of code. */
fun <T> KLogger.time(
    name: String,
    block: () -> T,
): T {
    info { "[BEGIN] $name" }
    val start = System.nanoTime()
    val result = block()
    val elapsed = "%.3f".format((System.nanoTime() - start) / 1_000_000_000.0)
    info { "[ END ] $name (${elapsed}s)" }
    return result
}

/** Times a block of code if it is unexpectedly slow. */
fun <T> KLogger.slow(
    name: String,
    limit: Duration,
    block: () -> T,
): T {
    val done = AtomicBoolean(false)
    val slow = AtomicBoolean(false)

    val thread =
        Thread {
            try {
                Thread.sleep(limit.inWholeMilliseconds)
                if (!done.get()) {
                    slow.set(true)
                    warn { "[UPDATE(SLOW)] $name is taking a while (>${limit}ms)..." }
                }
            } catch (e: InterruptedException) {
                println(e)
            }
        }

    thread.start()

    val start = System.nanoTime()
    val result = block()

    done.set(true)
    thread.interrupt()

    if (slow.get()) {
        val elapsed = "%.3f".format((System.nanoTime() - start) / 1_000_000_000.0)
        warn { "[ END (SLOW) ] $name (${elapsed}s)" }
    }

    return result
}
