// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.registry

import com.kylemayes.generator.support.toSnakeCase

/** Renames entities to be more Rust-friendly. */
fun Registry.renameEntities() {
    aliases.forEach { it.value.rename(::renameType) }
    basetypes.forEach { it.value.rename(::renameType) }
    constants.forEach { it.value.rename(::renameConstant) }
    extensions.forEach { it.value.rename(::renameConstant) }
    handles.forEach { it.value.rename(::renameType) }

    for (bitmask in bitmasks.values) {
        bitmask.rename(::renameType)
        bitmask.bitflags.forEach { it.rename { n -> renameVariantOrBitflag(n, bitmask.name.value, bitflag = true) } }

        // https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFlags.html#_description
        //
        // As described in the Vulkan documentation linked above, each bitmask
        // has a corresponding type alias which aliases the `VkFlags` type. For
        // example, the `VkCullModeFlagBits` bitmask has the `VkCullModeFlags`
        // type alias.
        //
        // While this generator ignores these type aliases, the Vulkan API uses
        // these type aliases in structs and commands instead of the actual
        // bitmask type. Therefore we need to rename these type aliases to
        // match the names of the corresponding bitmasks (which simply consists
        // of getting the identifiers of the type aliases and removing the `Vk`
        // prefix).
        "Vk${bitmask.name}".intern().rename(bitmask.name.value)
    }

    for (command in commands.values) {
        command.rename(::renameCommand)
        command.params.forEach { p -> p.rename(::renameMemberOrParameter) }
    }

    for (enum in enums.values) {
        enum.rename(::renameType)
        enum.variants.forEach { it.rename { n -> renameVariantOrBitflag(n, enum.name.value) } }
    }

    for (struct in structs.values) {
        struct.rename(::renameType)
        struct.members.forEach { it.rename(::renameMemberOrParameter) }

        // Some structs (e.g., `VkAccelerationStructureBuildGeometryInfoKHR`)
        // have two fields that, after the pointer prefixes are stripped (`p` or
        // `pp`), have the same name (e.g., `pGeometries` and `ppGeometries`).
        //
        // In this case we need to give one of them a different name, so the
        // `pp` version will get a `pointer_` prefix.
        val seen = mutableMapOf<String, Member>()
        for (member in struct.members) {
            if (seen[member.name.value] != null) {
                if (member.name.original.startsWith("pp")) {
                    member.name.rename("pointer_${member.name}")
                } else {
                    seen[member.name.value]!!.name.rename("pointer_${member.name}")
                }
            } else {
                seen[member.name.value] = member
            }
        }
    }

    for (union in unions.values) {
        union.rename(::renameType)
        union.members.forEach { it.rename(::renameMemberOrParameter) }
    }
}

/** Renames a command (e.g., `vkCreateInstance` to `create_instance`). */
private fun renameCommand(name: String) = name
    .removePrefix("vk")
    .toSnakeCase()

/** Renames a constant (e.g., `VK_UUID_SIZE` to `UUID_SIZE`). */
private fun renameConstant(name: String) = name
    .removePrefix("VK_")
    .toUpperCase()

/** Renames a member or parameter (e.g, `deviceLUIDValid` to `device_luid_valid`). */
private fun renameMemberOrParameter(name: String) = name
    .toSnakeCase()
    .replace(Regex("^(p+|pfn)_"), "")
    .replace(Regex("^(type)$"), "$1_")

/** Renames a type (e.g., `VkCullModeFlags` to `CullModeFlags`). */
private fun renameType(name: String) = name
    .removePrefix("Vk")

/** Renames an enum variant or bitmask bitflag (e.g., `VK_CULL_MODE_FRONT_BIT` to `FRONT`). */
private fun renameVariantOrBitflag(name: String, parent: String, bitflag: Boolean = false): String {
    // Find the start of the extension author suffix in the parent name, if any.
    // E.g., `EXT` in `VkDebugReportObjectTypeEXT`.
    var index = parent.length - 1
    while (index > 0 && parent[index].isUpperCase()) {
        index -= 1
    }

    // Determine the prefix to strip from the value name (parent name).
    // E.g., `VK_DEBUG_REPORT_OBJECT_TYPE_` for `VkDebugReportObjectTypeEXT` (variant).
    // E.g., `VK_DEBUG_REPORT_` for `VkDebugReportFlagsEXT` (bitflag).
    var prefix = "${parent.substring(0, index + 1).toSnakeCase().toUpperCase()}_"
    if (bitflag) prefix = prefix.replace("FLAGS_", "")

    // Determine the suffix to strip from the value name (parent extension author).
    // E.g., `_EXT` for `VkDebugReportObjectTypeEXT`
    val suffix = "_${parent.substring(index + 1)}".trimEnd('_')

    val renamed = name
        .removePrefix("VK_")
        .removePrefix(prefix)
        .removeSuffix(suffix)
        // Some value names start with digits after the prefixes have been
        // stripped which would make them invalid identifiers.
        .replace(Regex("^([0-9])"), "_$1")
        // Some value names include lowercase characters that need to be
        // capitalized (e.g., `VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK_EXT`).
        .toUpperCase()

    // Remove `BIT` component from bitflag name even when followed by extension author.
    return if (bitflag) {
        renamed.replace(Regex("_BIT(_[A-Z]+)?$"), "$1")
    } else {
        renamed
    }
}
