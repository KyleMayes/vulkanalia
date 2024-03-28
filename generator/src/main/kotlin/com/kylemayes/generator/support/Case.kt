// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.support

/** Converts this `snake_case` identifier to a `PascalCase` identifier. */
fun String.toPascalCase(): String {
    var pascal = ""

    var upper = true
    for (c in this) {
        when {
            upper -> {
                pascal += c.uppercase()
                upper = false
            }
            c == '_' -> upper = true
            else -> pascal += c.lowercase()
        }
    }

    return pascal
}

private val digitPrefixPattern = Regex("^(\\d+)(Bit|D)")
private val weirdSuffixPattern1 = Regex("[A-Z][A-Z]+\\d+$")
private val weirdSuffixPattern2 = Regex("([A-Z]+_)+[A-Z]+$")

/** Converts this `camelCase` or `PascalCase` identifier to a `snake_case` identifier. */
fun String.toSnakeCase(): String {
    var snake = this[0].lowercase()

    // Match identifiers that are suffixed by multiple acronyms separated by
    // underscores (e.g., `textureCompressionASTC_LDR`) or are suffixed by
    // acronyms that contain digits (e.g., `textureCompressionETC2`) because
    // these suffixes need to be specially handled.
    val weirdSuffix = weirdSuffixPattern1.find(this) ?: weirdSuffixPattern2.find(this)
    if (weirdSuffix != null) {
        val prefix = substring(0 until weirdSuffix.range.first).toSnakeCase()
        val suffix = substring(weirdSuffix.range).lowercase()
        return "${prefix}_$suffix"
    }

    // Iterate over windows so we can look ahead at the next character and so
    // we can look ahead for patterns that need to be specially handled.
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
        // (e.g., convert `viewportScissor2D` to `viewport_scissor_2d` rather
        // than `viewport_scissor2_d`).
        val match = digitPrefixPattern.find(window)
        if (match != null) {
            (2..match.value.length).forEach { _ -> iterator.next() }
            snake += "_"
            snake += match.groupValues[1]
            snake += match.groupValues[2].lowercase()
            continue
        }

        if (window[0].isLowerCase() || window[0].isDigit()) {
            snake += window[0]
            upper = false
        } else if (upper && (window.length < 2 || window[1].isUpperCase())) {
            snake += window[0].lowercase()
        } else {
            snake += "_"
            snake += window[0].lowercase()
            upper = true
        }
    }

    return snake
}
