package com.kylemayes.generator.registry

/** Filters out entities that are not supported by `vulkanalia`. */
fun Registry.filterEntities(): Registry {
    val unsupportedEntities = getUnsupportedEntities()
    fun <T : Entity> Map<Identifier, T>.filterSupportedEntities() =
        filter { !unsupportedEntities.contains(it.key) && it.value.isVulkanApi() }

    return copy(
        aliases = aliases.filterSupportedEntities(),
        basetypes = basetypes.filterSupportedEntities(),
        bitmasks = bitmasks.filterBitmasks().filterSupportedEntities(),
        constants = constants.filterSupportedEntities(),
        commands = commands.filterCommands().filterSupportedEntities(),
        commandAliases = commandAliases,
        enums = enums.filterEnums().filterSupportedEntities(),
        extensions = extensions.filter { it.value.isSupported() },
        functions = functions.filterSupportedEntities(),
        handles = handles.filterSupportedEntities(),
        structs = structs.filterStructures().filterSupportedEntities(),
        unions = unions.filterStructures().filterSupportedEntities(),
        versions = versions.filter { it.value.isVulkanApi() },
    )
}

// ===============================================
// Filter Extensions
// ===============================================

/** Vulkan extensions that are specifically not supported by `vulkanalia`. */
private val unsupportedExtensions = emptySet<String>()

/** Gets whether a Vulkan extension is supported by `vulkanalia`. */
private fun Extension.isSupported() =
    supported != "disabled" && !unsupportedExtensions.contains(name.original)

// ===============================================
// Filter Entities
// ===============================================

private fun Registry.getUnsupportedEntities(): Set<Identifier> {
    val unsupportedEntities = mutableSetOf<Identifier>()

    // Add entities that were added by unsupported extensions.

    unsupportedEntities.addAll(
        extensions.values
            .filter { !it.isSupported() || !it.isVulkanApi() }
            .flatMap { it.require.commands + it.require.types.map { n -> n.intern() } },
    )

    // Add entities only present in unsupported versions.

    val (vulkan, nonvulkan) = versions.values.partition { it.isVulkanApi() }

    val vulkanEntities = vulkan
        .flatMap { it.require.commands + it.require.types.map { n -> n.intern() } }
        .toSet()

    unsupportedEntities.addAll(
        nonvulkan
            .flatMap { it.require.commands + it.require.types.map { n -> n.intern() } }
            .filter { !vulkanEntities.contains(it) },
    )

    return unsupportedEntities
}

// ===============================================
// Filter Children
// ===============================================

private fun Map<Identifier, Bitmask>.filterBitmasks() =
    filterChildren({ it.bitflags }, { e, c -> e.copy(bitflags = c.toMutableList()) })
private fun Map<Identifier, Command>.filterCommands() =
    filterChildren({ it.params }, { e, c -> e.copy(params = c) })
private fun Map<Identifier, Enum>.filterEnums() =
    filterChildren({ it.variants }, { e, c -> e.copy(variants = c.toMutableList()) })
private fun Map<Identifier, Structure>.filterStructures() =
    filterChildren({ it.members }, { e, c -> e.copy(members = c) })

private fun <T : Entity, C : Entity> Map<Identifier, T>.filterChildren(
    get: (T) -> List<C>,
    set: (T, List<C>) -> T,
) = mapValues {
    val children = get(it.value)
    set(it.value, children.filter { c -> c.isVulkanApi() })
}
