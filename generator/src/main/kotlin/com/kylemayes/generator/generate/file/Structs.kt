// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.generateAliases
import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.generate.support.getStructBitfields
import com.kylemayes.generator.generate.support.getStructDerives
import com.kylemayes.generator.registry.Member
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.Structure
import com.kylemayes.generator.registry.getIdentifier
import com.kylemayes.generator.registry.isPlatformPointer

/** Generates Rust structs for Vulkan structs. */
fun Registry.generateStructs(): String {
    return """
use core::ffi::{c_char, c_int, c_void};
use core::fmt;
use core::ptr;

use crate::*;

${structs.values
        .sortedBy { it.name }
        .joinToString("\n") { generateStruct(it) }}

${generateAliases(structs.keys)}
    """
}

/** Generates a Rust struct for a Vulkan struct. */
private fun Registry.generateStruct(struct: Structure): String {
    val derives = getStructDerives(struct)

    val fields = mutableListOf<String>()
    for (member in struct.members.filter { it.bits == null }) {
        fields.add("pub ${member.name}: ${member.type.generate()}")
    }

    val bitfields = mutableListOf<String>()
    for ((index, members) in getStructBitfields(struct).indexToMembers) {
        val name = "${struct.name}Bitfields$index"
        fields.add("pub bitfields$index: $name")
        bitfields.add(generateBitfield(name, members))
    }

    return """
${bitfields.joinToString("")}

/// <${generateManualUrl(struct)}>
#[repr(C)]
#[derive(Copy, Clone, ${derives.joinToString()})]
pub struct ${struct.name} {
    ${fields.joinToString()}
}

${if (!derives.contains("Debug")) generateDebugImpl(struct) else ""}
${if (!derives.contains("Default")) generateDefaultImpl(struct) else ""}
    """
}

/** Generates a Rust `Debug` trait implementation for a Vulkan struct. */
private fun Registry.generateDebugImpl(struct: Structure) =
    """
impl fmt::Debug for ${struct.name} {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("${struct.name}")
            ${struct.members.joinToString("") { ".field(\"${it.name}\", ${generateDebugField(it)})" }}
            .finish()
    }
}
    """

/** Generates a Rust expression for debugging a Vulkan struct field. */
private fun Registry.generateDebugField(member: Member) =
    if (functions.containsKey(member.type.getIdentifier())) {
        "&(self.${member.name}.map(|f| f as *const u8))"
    } else {
        "&self.${member.name}"
    }

/** Generates a Rust `Default` trait implementation for a Vulkan struct. */
private fun Registry.generateDefaultImpl(struct: Structure) =
    """
impl Default for ${struct.name} {
    #[inline]
    fn default() -> Self {
        Self { ${struct.members.joinToString { "${it.name}: ${generateDefaultField(it)}" }} }
    }
}
    """

/** Generates a Rust expression for defaulting a Vulkan struct field. */
private fun Registry.generateDefaultField(member: Member) =
    when {
        member.name.value == "s_type" && member.values != null -> "StructureType::${member.values}"
        member.type.isPlatformPointer() -> "ptr::null_mut()"
        else -> member.type.generateDefault()
    }

private fun Registry.generateBitfield(
    name: String,
    members: List<Pair<Int, Member>>,
): String {
    val fields =
        members.joinToString("\n        ") { (start, member) ->
            "(${member.name}, with_${member.name})[$start .. ${start + member.bits!!}],"
        }

    return """
bitfields32! {
    struct $name {
        $fields
    }
}
    """
}
