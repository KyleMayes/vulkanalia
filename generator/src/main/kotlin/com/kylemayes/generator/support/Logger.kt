// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.support

import mu.KLogger

/** Times a block of code. */
fun <T> KLogger.time(name: String, block: () -> T): T {
    info { "[BEGIN] $name" }
    val start = System.nanoTime()
    val result = block()
    val elapsed = "%.3f".format((System.nanoTime() - start) / 1_000_000_000.0)
    info { "[ END ] $name (${elapsed}s)" }
    return result
}
