// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.support.bindgen
import kotlin.io.path.ExperimentalPathApi
import kotlin.io.path.createTempDirectory
import kotlin.io.path.deleteRecursively
import kotlin.io.path.readText
import kotlin.io.path.writeText

/** Generates a Rust module using `bindgen` for a collection of C/C++ headers. */
@OptIn(ExperimentalPathApi::class)
fun generateHeaders(headers: Map<String, String>, options: List<String> = emptyList()): String {
    if (headers.isEmpty()) {
        return ""
    }

    val tmp = createTempDirectory("vk-headers")
    headers.forEach { tmp.resolve(it.key).writeText(it.value) }

    val input = tmp.resolve("__input.h")
    val output = tmp.resolve("__output.rs")
    input.writeText(headers.keys.joinToString("\n") { "#include \"$it\"" })

    bindgen(
        "--verbose",
        "--rust-target=1.64",
        "--use-core",
        "--no-layout-tests",
        *options.toTypedArray(),
        input.toString(),
        "--output=$output",
    )

    val contents = output.readText()

    tmp.deleteRecursively()

    return contents
}
