// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.support

import com.kylemayes.generator.registry.Identifier
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.Structure
import com.kylemayes.generator.registry.Type
import com.kylemayes.generator.registry.getBaseIdentifier
import com.kylemayes.generator.registry.getElement
import com.kylemayes.generator.registry.getIdentifier
import com.kylemayes.generator.registry.isPointer
import kotlin.math.max

/** Gets the non-pointer dependencies of a Vulkan struct on other Vulkan structs. */
val getStructDependencies = thunk { struct: Structure ->
    struct.members
        .mapNotNull { m -> m.type.getBaseIdentifier() }
        .filter { d -> structs.containsKey(d) }
        .toSet()
}

private val getStructDerivesResults = HashMap<Identifier, Set<String>>()

/** Gets the Rust traits which can be derived for a Vulkan struct. */
fun Registry.getStructDerives(struct: Structure): Set<String> {
    val memoized = getStructDerivesResults[struct.name]
    if (memoized != null) {
        return memoized
    }

    val arrayLengthAtMost32 = (getMaxArrayLength(struct) ?: 0) <= 32
    val floats = struct.members.any { m -> m.type.getBaseIdentifier()?.value == "float" }
    val functions = struct.members.any { m -> functions.containsKey(m.type.getIdentifier()) }
    val pointers = struct.members.any { m -> m.type.isPointer() }
    val unions = struct.members.any { m -> unions.containsKey(m.type.getIdentifier()) }

    // These traits will be "manually" implemented for structs they can't be
    // derived for so we don't need to worry about whether the struct
    // dependencies will derive them.

    val required = HashSet<String>()
    if (!functions) required.add("Debug")
    if (arrayLengthAtMost32 && !pointers) required.add("Default")

    // These traits will not be "manually" implemented so they can only be
    // applied to structs that meet the requirements and don't (transitively)
    // contain other structs that do not meet the requirements.

    val optional = HashSet<String>()
    val partialEq = !functions && !unions
    if (partialEq) optional.add("PartialEq")
    if (partialEq && !floats) { optional.add("Eq"); optional.add("Hash") }

    for (dependency in getStructDependencies(struct)) {
        val derives = getStructDerives(structs[dependency] ?: error("Missing struct."))
        optional.removeIf { !derives.contains(it) }
    }

    val result = required.union(optional)
    getStructDerivesResults[struct.name] = result
    return result
}

/** Gets the length of the longest array in a Vulkan struct. */
private fun Registry.getMaxArrayLength(struct: Structure) = struct.members
    .mapNotNull { getMaxArrayLength(it.type) }
    .maxOrNull()

/** Gets the length of the longest array in a Vulkan type. */
private fun Registry.getMaxArrayLength(type: Type): Long? {
    val length = getLengthValue(type) ?: return null
    val elementLength = type.getElement()?.let { getMaxArrayLength(it) } ?: 0
    return max(length, elementLength)
}
