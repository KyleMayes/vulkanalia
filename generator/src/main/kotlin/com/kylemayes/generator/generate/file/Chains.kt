// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.getChainStructs
import com.kylemayes.generator.registry.PointerType
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.Structure
import java.lang.Error

fun Registry.generateChains(): String {
    return """
use core::ffi::c_void;

use super::*;

/// A Vulkan struct that could be part of an input pointer chain.
pub unsafe trait InputChainStruct {
    /// The structure type for this struct type.
    const TYPE: StructureType;

    /// The structure type for this struct.
    fn s_type(&self) -> StructureType;

    /// The next struct in the pointer chain for this struct.
    fn next(&self) -> *const c_void;
}

/// A Vulkan struct that could be part of an output pointer chain.
pub unsafe trait OutputChainStruct: InputChainStruct {
    /// The next struct in the pointer chain for this struct.
    fn next_mut(&self) -> *mut c_void;
}

${getChainStructs().values
        .sortedBy { it.name }
        .joinToString("\n") { generateChainStruct(it) }}
    """
}

private fun Registry.generateChainStruct(struct: Structure): String {
    val member = struct.members.find { it.name.original == "sType" }
    val type = member?.values?.value ?: throw Error("Missing structure type (${struct.name}).")
    val output = !(struct.members[1].type as PointerType).const
    return """
unsafe impl InputChainStruct for ${struct.name} {
    const TYPE: StructureType = StructureType::$type;

    #[inline]
    fn s_type(&self) -> StructureType {
        self.s_type
    }

    #[inline]
    fn next(&self) -> *const c_void {
        self.next
    }
}

${if (output) generateOutputChainStruct(struct) else ""}
    """
}

private fun generateOutputChainStruct(struct: Structure): String {
    return """
unsafe impl OutputChainStruct for ${struct.name} {
    #[inline]
    fn next_mut(&self) -> *mut c_void {
        self.next
    }
}
    """
}
