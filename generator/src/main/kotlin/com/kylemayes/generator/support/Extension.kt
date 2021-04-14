package com.kylemayes.generator.support

import com.kylemayes.generator.registry.Extension

private val skip = setOf<String>()

/** Gets whether a Vulkan extension is supported by `vulkanalia`. */
fun isSupported(extension: Extension) =
    extension.supported != "disabled" && !skip.contains(extension.name.original)
