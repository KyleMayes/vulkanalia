// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.support

import com.kylemayes.generator.registry.Entity
import com.kylemayes.generator.registry.Registry

/** The generated URLs for Vulkan manual pages. */
val manualUrls = mutableListOf<Pair<String, String>>()

/** Generates a URL for a Vulkan manual page for a Vulkan entity. */
fun Registry.generateManualUrl(entity: Entity) = generateManualUrl(entity.name.original)

/** Generates a URL for a Vulkan manual page for a Vulkan entity. */
fun Registry.generateManualUrl(name: String): String {
    val url = "https://www.khronos.org/registry/vulkan/specs/latest/man/html/$name.html"
    manualUrls.add(name to url)
    return url
}
