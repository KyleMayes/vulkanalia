// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.support

import com.kylemayes.generator.registry.Identifier
import com.kylemayes.generator.registry.PointerType
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.Structure
import com.kylemayes.generator.registry.Type
import com.kylemayes.generator.registry.getBaseIdentifier
import com.kylemayes.generator.registry.getElement
import com.kylemayes.generator.registry.getIdentifier
import com.kylemayes.generator.registry.isPointer
import kotlin.math.max

/** Gets the non-pointer dependencies of a Vulkan struct on other Vulkan structs. */
val getStructDependencies =
    thunk { struct: Structure ->
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

    val floats = struct.members.any { m -> m.type.getBaseIdentifier()?.value == "float" }
    val functions = struct.members.any { m -> functions.containsKey(m.type.getIdentifier()) }
    val pointers = struct.members.any { m -> m.type.isPointer() }
    val unions = struct.members.any { m -> unions.containsKey(m.type.getIdentifier()) }

    // These traits will be "manually" implemented for structs they can't be
    // derived for so we don't need to worry about whether the struct
    // dependencies will derive them.

    val required = HashSet<String>()
    if (!functions) required.add("Debug")
    if (!pointers) required.add("Default")

    // These traits will not be "manually" implemented so they can only be
    // applied to structs that meet the requirements and don't (transitively)
    // contain other structs that do not meet the requirements.

    val optional = HashSet<String>()
    val partialEq = !functions && !unions
    if (partialEq) optional.add("PartialEq")
    if (partialEq && !floats) {
        optional.add("Eq")
        optional.add("Hash")
    }

    for (dependency in getStructDependencies(struct)) {
        val derives = getStructDerives(structs[dependency] ?: error("Missing struct."))
        optional.removeIf { !derives.contains(it) }
    }

    val result = required.union(optional)
    getStructDerivesResults[struct.name] = result
    return result
}

/** Gets the length of the longest array in a Vulkan struct. */
private fun Registry.getMaxArrayLength(struct: Structure) =
    struct.members
        .mapNotNull { getMaxArrayLength(it.type) }
        .maxOrNull()

/** Gets the length of the longest array in a Vulkan type. */
private fun Registry.getMaxArrayLength(type: Type): Long? {
    val length = getLengthValue(type) ?: return null
    val elementLength = type.getElement()?.let { getMaxArrayLength(it) } ?: 0
    return max(length, elementLength)
}

/** Gets the Vulkan structs that can be used to extend other Vulkan structs. */
val getStructExtensions =
    thunk { ->
        structs.values
            .filter { it.structextends != null }
            .flatMap { it.structextends!!.map { e -> e to it.name } }
            .groupBy({ it.first }, { it.second })
    }

private val getStructLifetimeResults = HashMap<Identifier, Boolean>()

/** Gets whether a Vulkan struct requires a lifetime for its builder. */
fun Registry.getStructLifetime(struct: Structure): Boolean {
    val memoized = getStructLifetimeResults[struct.name]
    if (memoized != null) {
        return memoized
    }

    // Builder methods for pointer members will use references with lifetimes
    // so any struct that contains a pointer will need a lifetime. An exception
    // is made for the `next` member if there are no extending structs since
    // the corresponding builder method will be omitted in this case.
    val members =
        struct.members.any {
            it.type is PointerType &&
                (it.name.value != "next" || getStructExtensions()[struct.name]?.isNotEmpty() ?: false)
        }

    // Builder method for struct members will use
    val dependencies =
        getStructDependencies(struct).any {
            getStructLifetime(structs[it] ?: error("Missing struct."))
        }

    val result = members || dependencies
    getStructLifetimeResults[struct.name] = result
    return result
}

/** Gets the Vulkan structs that can be part of a pointer chain. */
val getChainStructs =
    thunk { ->
        structs.filter {
            val name = it.key.original
            if (name == "VkBaseInStructure" || name == "VkBaseOutStructure") {
                // These are helper structs used for iterating through pointer
                // chains, not pointer chain structs themselves.
                false
            } else {
                val type = it.value.members.getOrNull(0)?.name?.original == "sType"
                val next = it.value.members.getOrNull(1)?.name?.original == "pNext"
                type && next
            }
        }
    }
