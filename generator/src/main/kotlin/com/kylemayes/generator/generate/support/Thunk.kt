// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.support

import com.kylemayes.generator.registry.Registry

/** Wraps a Vulkan API registry operation in another function which caches the result. */
fun <T> thunk(operation: Registry.() -> T): Registry.() -> T {
    var result: T? = null
    return { result ?: operation(this).also { r -> result = r } }
}

/** Wraps a Vulkan API registry operation in another function which caches the results. */
fun <T, R> thunk(operation: Registry.(T) -> R): Registry.(T) -> R {
    val results = HashMap<T, R>()
    return { results[it] ?: operation(this, it).also { r -> results[it] = r } }
}
