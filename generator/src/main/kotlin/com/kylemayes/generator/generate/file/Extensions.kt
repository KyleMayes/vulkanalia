// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.generateCommandWrapper
import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.generate.support.getExtensionGroups
import com.kylemayes.generator.registry.Extension
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.support.toPascalCase

/** Generates Rust modules and constants for Vulkan extensions. */
fun Registry.generateExtensions() =
    """
use std::os::raw::c_char;

use crate::MAX_EXTENSION_NAME_SIZE;

/// A Vulkan extension name.
pub type ExtensionName = [c_char; MAX_EXTENSION_NAME_SIZE];

${getExtensionGroups().values.flatten().sortedBy { it.name }.joinToString("") { generateExtension(it) }}

/// Converts a byte string into a Vulkan extension name.
#[inline]
pub const fn to_extension_name(bytes: &[u8]) -> ExtensionName {
    let mut name = [0; MAX_EXTENSION_NAME_SIZE];

    let mut index = 0;
    while index < bytes.len() {
        name[index] = bytes[index] as c_char;
        index += 1;
    }

    if bytes.is_empty() || bytes[bytes.len() - 1] != 0 {
        name[bytes.len()] = 0;
    }

    name
}
    """

/** Generates a Rust constant for a Vulkan extension. */
private fun Registry.generateExtension(extension: Extension): String {
    val deprecation = generateDeprecation(extension)?.let { "\n$it" } ?: ""
    return """
/// <${generateManualUrl(extension)}>$deprecation
pub const ${extension.name}_EXTENSION: ExtensionName =
    to_extension_name(b"${extension.name.original}");
    """.trim()
}

/** Generates Rust modules and traits for Vulkan extensions. */
fun Registry.generateExtensionTraits() =
    """
use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::os::raw::{c_int, c_void};
use std::ptr::{self, copy_nonoverlapping as memcpy};

use super::*;

/// A Vulkan type that can be converted to or from a borrowed C string.
pub trait ConvertCStr {
    /// Converts a borrowed C string into a value.
    fn from_cstr(string: &CStr) -> Self;

    /// Converts this value into a borrowed C string.
    fn to_cstr(&self) -> &CStr;
}

impl ConvertCStr for ExtensionName {
    #[inline]
    fn from_cstr(string: &CStr) -> Self {
        let mut name = [0; MAX_EXTENSION_NAME_SIZE];
        let count = string.to_bytes().len();
        unsafe { memcpy(string.as_ptr(), name.as_mut_ptr(), count) };
        name
    }

    #[inline]
    fn to_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_ptr()) }
    }
}

${getExtensionGroups().values.flatten().sortedBy { it.name }.joinToString("") { generateExtensionTrait(it) }}
    """

/** Generates a Rust trait and implementation for a Vulkan extension. */
private fun Registry.generateExtensionTrait(extension: Extension): String {
    val deprecation = generateDeprecation(extension)?.let { "\n$it" } ?: ""
    val name = "${extension.name.value.toPascalCase()}Extension"
    val type = extension.type!!.capitalize()
    val commands = extension.require.commands.mapNotNull { commands[it] }.sortedBy { it.name }
    return """
/// <${generateManualUrl(extension)}>$deprecation
pub trait $name: ${type}V1_0 {
    ${commands.joinToString("") { generateCommandWrapper(it) }}
}

${if (deprecation.isNotEmpty()) { "#[allow(deprecated)]" } else { "" }}
impl $name for crate::$type { }
    """
}

/** Generates a Rust deprecation annotation for a Vulkan extension. */
private fun generateDeprecation(extension: Extension) = extension.deprecatedby?.let {
    if (it.isNotBlank()) {
        """#[deprecated(note = "deprecated in favor of `$it`")]"""
    } else {
        "#[deprecated]"
    }
}
