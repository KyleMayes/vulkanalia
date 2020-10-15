// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.support

/** Converts this `snake_case` string to a `PascalCase` string. */
fun String.toPascalCase(): String {
    var pascal = ""

    var upper = true
    for (c in this) {
        when {
            upper -> { pascal += c.toUpperCase(); upper = false }
            c == '_' -> upper = true
            else -> pascal += c.toLowerCase()
        }
    }

    return pascal
}

private val digitPrefixPattern = Regex("^(\\d+)(Bit|D)")
private val weirdSuffixPattern1 = Regex("[A-Z][A-Z]+\\d+$")
private val weirdSuffixPattern2 = Regex("([A-Z]+_)+[A-Z]+$")

/** Converts this `camelCase` or `PascalCase` string to a `snake_case` string. */
fun String.toSnakeCase(): String {
    var snake = this[0].toString().toLowerCase()

    // Handle weird suffixes in cases like `textureCompressionETC2` (pattern 1)
    // or `textureCompressionASTC_LDR` (pattern 2).
    val weird = weirdSuffixPattern1.find(this) ?: weirdSuffixPattern2.find(this)
    if (weird != null) {
        val prefix = substring(0 until weird.range.first).toSnakeCase()
        val suffix = substring(weird.range).toLowerCase()
        return "${prefix}_$suffix"
    }

    var upper = this[0].isUpperCase()
    val iterator = substring(1).windowed(5, partialWindows = true).iterator()
    while (iterator.hasNext()) {
        val window = iterator.next()

        // Handle identifiers that are already `snake_case` (e.g., `visual_id`).
        if (window[0] == '_') {
            return this
        }

        // The default case conversion logic groups digits with any preceding
        // lowercase characters (e.g., `shaderBufferInt64Atomics` would be
        // converted to `shader_buffer_int64_atomics`). However, there are cases
        // where it is preferable to group digits with the following characters
        // (e.g., convert `residencyStandard3DBlockShape` to
        // `residency_standard_3d_block_shape` rather than
        // `residency_standard3_d_block_shape`).
        val match = digitPrefixPattern.find(window)
        if (match != null) {
            (2..match.value.length).forEach { _ -> iterator.next() }
            snake += "_"
            snake += match.groupValues[1]
            snake += match.groupValues[2].toLowerCase()
            continue
        }

        if (window[0].isLowerCase() || window[0].isDigit()) {
            snake += window[0]
            upper = false
        } else if (upper && (window.length < 2 || window[1].isUpperCase())) {
            snake += window[0].toLowerCase()
        } else {
            snake += "_"
            snake += window[0].toLowerCase()
            upper = true
        }
    }

    return snake
}
