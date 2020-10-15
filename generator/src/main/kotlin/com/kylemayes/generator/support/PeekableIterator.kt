// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.support

/** A peekable iterator over a list of items. */
class PeekableIterator<T>(private val items: List<T>, private var index: Int = 0) {
    fun isEmpty() = index >= items.size
    fun peek(offset: Int = 0) = items.getOrNull(index + offset)
    fun advance() = items[index++]
    fun takeWhile(f: (T) -> Boolean): List<T> {
        val items = mutableListOf<T>()
        while (peek()?.let(f) == true) items.add(advance())
        return items
    }
}
