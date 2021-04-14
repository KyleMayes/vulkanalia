package com.kylemayes.generator.support

import com.kylemayes.generator.registry.Extension

private val skip = setOf(
    // Video extensions depend on a bunch of types not defined in the Vulkan
    // API registry but instead live in some additional Vulkan headers. Maybe
    // support could be added using `bindgen` to generate Rust types from these
    // additional Vulkan headers?
    "VK_EXT_video_decode_h264",
    "VK_EXT_video_decode_h265",
    "VK_EXT_video_encode_h264",
    "VK_EXT_video_encode_h265",
    "VK_KHR_video_decode_queue",
    "VK_KHR_video_encode_queue",
    "VK_KHR_video_queue",
)

/** Gets whether a Vulkan extension is supported by `vulkanalia`. */
fun isSupported(extension: Extension) =
    extension.supported != "disabled" && !skip.contains(extension.name.original)
