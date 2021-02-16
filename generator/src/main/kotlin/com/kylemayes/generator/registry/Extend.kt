// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.registry

import java.math.BigInteger

/** Extends bitmasks and enums with bitflags and variants defined in versions and extensions. */
fun Registry.extendEntities() {
    val added = HashSet<Entity>()

    for (version in versions.values) {
        for (ext in version.require.values) {
            extendBitmask(ext, bitmasks, added)
            extendEnum(ext, enums, added, null)
        }
    }

    for (extension in extensions.values) {
        for (ext in extension.require.values) {
            extendBitmask(ext, bitmasks, added)
            extendEnum(ext, enums, added, extension.number)
        }
    }
}

/** Extends a bitmask for a version/extension value if it represents a bitflag. */
private fun extendBitmask(
    ext: RequireValue,
    bitmasks: Map<Identifier, Bitmask>,
    added: MutableSet<Entity>,
) {
    val bitmask = bitmasks[ext.extends]
    val bitpos = ext.bitpos
    if (bitmask != null && bitpos != null) {
        val bitflag = Bitflag(name = ext.name, BigInteger.ONE.shiftLeft(bitpos.toInt()))
        if (added.add(bitflag)) {
            bitmask.bitflags.add(bitflag)
        }
    }
}

/** Extends an enum for a version/extension value if it represents a variant. */
private fun extendEnum(
    ext: RequireValue,
    enums: Map<Identifier, Enum>,
    added: MutableSet<Entity>,
    extnumber: Long?,
) {
    val enum = enums[ext.extends]
    val value = getVariantValue(ext, extnumber)
    if (enum != null && value != null) {
        val variant = Variant(name = ext.name, value = value)
        if (added.add(variant)) {
            enum.variants.add(variant)
        }
    }
}

/** Gets the value for a version/extension variant. */
private fun getVariantValue(ext: RequireValue, extnumber: Long?): Long? {
    if (ext.value != null) {
        return ext.value
    }

    // https://www.khronos.org/registry/vulkan/specs/1.2/styleguide.html#_assigning_extension_token_values
    val number = ext.extnumber ?: extnumber ?: return null
    val offset = ext.offset ?: return null
    val value = 1_000_000_000L + (1_000 * (number - 1)) + offset
    return if (ext.negative) { -value } else { value }
}
