// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.generateAliases
import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.generate.support.getLengthValue
import com.kylemayes.generator.generate.support.getStructDerives
import com.kylemayes.generator.registry.Member
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.Structure
import com.kylemayes.generator.registry.getIdentifier
import com.kylemayes.generator.registry.isPlatformPointer
import com.kylemayes.generator.registry.isPointer

/** Generates Rust structs for Vulkan structs. */
fun Registry.generateStructs() =
    """
use std::fmt;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

use crate::*;

${structs.values.sortedBy { it.name }.joinToString("\n") { generateStruct(it) }}
${generateAliases(structs.keys)}
    """

/** Generates a Rust struct for a Vulkan struct. */
private fun Registry.generateStruct(struct: Structure): String {
    val derives = getStructDerives(struct)
    return """
/// <${generateManualUrl(struct)}>
#[repr(C)]
#[derive(Copy, Clone, ${derives.joinToString()})]
pub struct ${struct.name} {
    ${struct.members.joinToString { "pub ${it.name}: ${it.type.generate()}" }}
}

${if (!derives.contains("Debug")) { generateDebugImpl(struct) } else { "" }}
${if (!derives.contains("Default")) { generateDefaultImpl(struct) } else { "" }}
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
private fun Registry.generateDefaultImpl(struct: Structure): String {
    val members = struct.members.filter {
        (it.name.value == "s_type" && it.values != null) ||
            it.type.isPointer() ||
            (getLengthValue(it.type) ?: 0) > 32
    }

    val exprs = members.joinToString { "${it.name}: ${generateDefaultField(it)}" }
    val trailing = if (members.size < struct.members.size) {
        ", ..Default::default()"
    } else {
        ""
    }

    return """
impl Default for ${struct.name} {
    #[inline]
    fn default() -> Self {
        Self { $exprs$trailing }
    }
}
    """
}

/** Generates a Rust expression for defaulting a Vulkan struct field. */
private fun Registry.generateDefaultField(member: Member) = when {
    member.name.value == "s_type" && member.values != null -> "StructureType::${member.values}"
    member.type.isPlatformPointer() -> "ptr::null_mut()"
    else -> member.type.generateDefault()
}
