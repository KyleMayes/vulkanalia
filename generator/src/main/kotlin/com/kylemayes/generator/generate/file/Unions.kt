// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.generateAliases
import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.Structure

/** Generates Rust unions for Vulkan unions. */
fun Registry.generateUnions() =
    """
use core::fmt;
use core::mem::MaybeUninit;
use core::ffi::{c_char, c_void};

use crate::*;

${unions.values
        .sortedBy { it.name }
        .joinToString("\n") { generateUnion(it) }}

${generateAliases(unions.keys)}
    """

/** Generates a Rust union for a Vulkan union. */
private fun Registry.generateUnion(union: Structure) =
    """
/// <${generateManualUrl(union)}>
#[repr(C)]
#[derive(Copy, Clone)]
pub union ${union.name} {
    ${union.members.joinToString { "pub ${it.name}: ${it.type.generate()}" }}
}

impl Default for ${union.name} {
    #[inline]
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl fmt::Debug for ${union.name} {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${union.name}")
    }
}
    """
