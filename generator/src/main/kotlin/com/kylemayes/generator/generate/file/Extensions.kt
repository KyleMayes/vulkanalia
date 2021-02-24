// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.generateCommandWrapper
import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.generate.support.getExtensionGroups
import com.kylemayes.generator.registry.Extension
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.intern
import com.kylemayes.generator.support.toPascalCase

/** The warning and `cfg` applied to the documentation for provisional extensions. */
private val PROVISIONAL: String =
    """
///
/// ## WARNING
///
/// This is a
/// [provisional extension](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/provisional-headers.html).
/// Provisional extensions are not guaranteed to be backwards compatible and are
/// not intended to be used in production applications.
#[cfg(feature = "provisional")]
   """.trimEnd()

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
    val provisional = if (extension.provisional) { PROVISIONAL } else { "" }
    val deprecation = generateDeprecation(extension)?.let { "\n$it" } ?: ""
    return """
/// <${generateManualUrl(extension)}>$provisional$deprecation
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

/// A collection of metadata for a Vulkan extension.
#[derive(Copy, Clone, Debug)]
pub struct ExtensionMetadata {
    /// The name of the extension.
    pub name: ExtensionName,
    /// The number assigned to the extension.
    pub number: i32,

    /// The type of the extension (`device` or `instance`).
    pub type_: &'static str,

    /// The author of the extension.
    pub author: &'static str,
    /// The primary contact(s) for the extension.
    pub contact: &'static str,

    /// The platform the extension can be used on (e.g., `wayland` or `win32`).
    pub platform: Option<&'static str>,

    /// The extensions required by the extension.
    pub requires: Option<&'static [ExtensionName]>,
    /// The Vulkan version required by the extension (e.g., `1.1`).
    pub requires_core: Option<&'static str>,

    /// The Vulkan extension or version that deprecated the extension (e.g., `VK_VERSION_1_1`).
    pub deprecated_by: Option<&'static str>,
    /// The Vulkan extension or version that obsoleted the extension (e.g., `VK_VERSION_1_1`).
    pub obsoleted_by: Option<&'static str>,
    /// The Vulkan version the extension was promoted to core in (e.g., `VK_VERSION_1_1`).
    pub promoted_to: Option<&'static str>,
}

${getExtensionGroups().values.flatten().sortedBy { it.name }.joinToString("") { generateExtensionTrait(it) }}
    """
/** Generates a Rust trait and implementation for a Vulkan extension. */
private fun Registry.generateExtensionTrait(extension: Extension): String {
    val provisional = if (extension.provisional) { PROVISIONAL } else { "" }
    val deprecation = generateDeprecation(extension)?.let { "\n$it" } ?: ""

    val name = "${extension.name.value.toPascalCase()}Extension"
    val type = extension.type!!.capitalize()

    val commands = extension.require.commands.mapNotNull { commands[it] }.sortedBy { it.name }

    val implAttributes = listOf(
        if (extension.provisional) { "#[cfg(feature = \"provisional\")]" } else { "" },
        if (deprecation.isNotEmpty()) { "#[allow(deprecated)]" } else { "" }
    ).filter { it.isNotBlank() }.joinToString("\n")

    return """
/// <${generateManualUrl(extension)}>$provisional$deprecation
pub trait $name: ${type}V1_0 {
    /// The metadata for this extension.
    #[allow(deprecated)]
    const METADATA: ExtensionMetadata = ${generateMetadata(extension)};

    ${commands.joinToString("") { generateCommandWrapper(it) }}
}

$implAttributes
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

/** Generates a Rust struct for the metadata for a Vulkan extension. */
private fun Registry.generateMetadata(extension: Extension): String {
    val requires = if (extension.requires != null) {
        val names = extension.requires
            .split(",")
            .map { extensions[it.intern()]!!.name }
            .joinToString(", ") { "super::${it}_EXTENSION" }
        "Some(&[$names])"
    } else {
        "None"
    }

    return """
ExtensionMetadata {
    name: super::${extension.name}_EXTENSION,
    number: ${extension.number},
    type_: "${extension.type}",
    author: "${extension.author}",
    contact: "${extension.contact}",
    platform: ${extension.platform?.let { "Some(\"$it\")" } ?: "None"},
    requires: $requires,
    requires_core: ${extension.requiresCore?.let { "Some(\"$it\")" } ?: "None"},
    deprecated_by: ${extension.deprecatedby?.let { "Some(\"$it\")" } ?: "None"},
    obsoleted_by: ${extension.obsoletedby?.let { "Some(\"$it\")" } ?: "None"},
    promoted_to: ${extension.promotedto?.let { "Some(\"$it\")" } ?: "None"},
}
    """
}
