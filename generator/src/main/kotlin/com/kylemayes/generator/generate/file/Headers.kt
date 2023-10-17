// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.support.bindgen
import com.kylemayes.generator.support.time
import mu.KotlinLogging
import kotlin.io.path.ExperimentalPathApi
import kotlin.io.path.createTempDirectory
import kotlin.io.path.deleteRecursively
import kotlin.io.path.writeText

private val log = KotlinLogging.logger { /* */ }

/** Generates a Rust module using `bindgen` for a collection of C/C++ headers. */
@OptIn(ExperimentalPathApi::class)
fun generateHeaders(name: String, headers: Map<String, String>, options: List<String> = emptyList()): String {
    if (headers.isEmpty()) {
        return ""
    }

    val tmp = createTempDirectory("vk-headers")
    headers.forEach { tmp.resolve(it.key).writeText(it.value) }

    val input = tmp.resolve("__input.h")
    input.writeText(headers.keys.joinToString("\n") { "#include \"$it\"" })

    val bindings = log.time("Generate Headers ($name)") {
        bindgen(
            "--verbose",
            "--rust-target=1.64",
            "--use-core",
            "--no-layout-tests",
            *options.toTypedArray(),
            input.toString(),
        )
    }

    tmp.deleteRecursively()

    // Hack to ensure consistent enum types on Windows and non-Windows.
    return bindings.replace("(pub ::core::ffi::c_uint);", "(pub ::core::ffi::c_int);")
}
