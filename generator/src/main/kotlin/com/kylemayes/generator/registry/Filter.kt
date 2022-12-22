package com.kylemayes.generator.registry

/** Filters out entities that are not supported by `vulkanalia`. */
fun Registry.filterEntities(): Registry {
    val supportedExtensions = extensions.filter { it.value.isSupported() }

    // Filters out entities that were added by unsupported extensions.
    val unsupportedEntities = extensions.values
        .filter { !supportedExtensions.containsKey(it.name) }
        .flatMap { it.require.commands + it.require.types.map { n -> n.intern() } }
        .toSet()
    fun <T : Entity> Map<Identifier, T>.filterSupportedEntities() =
        filter { !unsupportedEntities.contains(it.key) }

    return copy(
        aliases = aliases.filterSupportedEntities(),
        basetypes = basetypes.filterSupportedEntities(),
        bitmasks = bitmasks.filterSupportedEntities(),
        constants = constants.filterSupportedEntities(),
        commands = commands.filterSupportedEntities(),
        commandAliases = commandAliases,
        enums = enums.filterSupportedEntities(),
        extensions = supportedExtensions,
        functions = functions.filterSupportedEntities(),
        handles = handles.filterSupportedEntities(),
        structs = structs.filterSupportedEntities(),
        unions = unions.filterSupportedEntities(),
        versions = versions,
    )
}

/** Vulkan extensions that are specifically not supported by `vulkanalia`. */
private val unsupportedExtensions = setOf(
    // Video extensions depend on a bunch of types not defined in the Vulkan
    // API registry but instead live in some additional Vulkan headers. Maybe
    // support could be added using `bindgen` to generate Rust types from these
    // additional Vulkan headers?
    "VK_EXT_video_decode_h264",
    "VK_EXT_video_decode_h265",
    "VK_EXT_video_encode_h264",
    "VK_EXT_video_encode_h265",
    "VK_KHR_video_decode_h264",
    "VK_KHR_video_decode_h265",
    "VK_KHR_video_decode_queue",
    "VK_KHR_video_encode_queue",
    "VK_KHR_video_queue",
)

/** Gets whether a Vulkan extension is supported by `vulkanalia`. */
private fun Extension.isSupported() =
    supported != "disabled" && !unsupportedExtensions.contains(name.original)
