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
use crate::{StringArray, MAX_EXTENSION_NAME_SIZE};

/// A Vulkan extension name.
pub type ExtensionName = StringArray<MAX_EXTENSION_NAME_SIZE>;

/// A collection of metadata for a Vulkan extension.
#[derive(Copy, Clone, Debug)]
pub struct Extension {
    /// The name of the extension.
    pub name: ExtensionName,
    /// The unique number assigned to the extension.
    pub number: i32,

    /// The type of the extension (`device` or `instance`).
    pub type_: &'static str,

    /// The author of the extension (e.g., `KHR`).
    pub author: &'static str,
    /// The primary contact(s) for the extension.
    pub contact: &'static str,

    /// The platform the extension applies to (e.g., `wayland` or `win32`).
    pub platform: Option<&'static str>,

    /// The other extensions required by the extension.
    pub required_extensions: Option<&'static [ExtensionName]>,
    /// The Vulkan version required by the extension (e.g., `1.1`).
    pub required_version: Option<&'static str>,

    /// The Vulkan extension or version that deprecated the extension (e.g., `VK_VERSION_1_1`).
    pub deprecated_by: Option<&'static str>,
    /// The Vulkan extension or version that obsoleted the extension (e.g., `VK_VERSION_1_1`).
    pub obsoleted_by: Option<&'static str>,
    /// The Vulkan version the extension was promoted to core in (e.g., `VK_VERSION_1_1`).
    pub promoted_to: Option<&'static str>,
}

${getExtensionGroups().values
        .flatten()
        .sortedBy { it.name }
        .joinToString("") { generateExtension(it) }}
    """

/** Generates a Rust constant for a Vulkan extension. */
private fun Registry.generateExtension(extension: Extension): String {
    val provisional = if (extension.provisional) PROVISIONAL else ""
    val deprecation = generateDeprecation(extension)?.let { "\n$it" } ?: ""

    val extensions =
        if (extension.requires != null) {
            val names =
                extension.requires.split(",")
                    .map { extensions[it.intern()]!!.name }
                    .joinToString { "${it}_EXTENSION.name" }
            "Some(&[$names])"
        } else {
            "None"
        }

    return """
/// <${generateManualUrl(extension)}>$provisional$deprecation
#[allow(deprecated)]
pub const ${extension.name}_EXTENSION: Extension = Extension {
    name: ExtensionName::from_bytes(b"${extension.name.original}"),
    number: ${extension.number},
    type_: "${extension.type}",
    author: "${extension.author}",
    contact: "${extension.contact}",
    platform: ${extension.platform?.let { "Some(\"$it\")" } ?: "None"},
    required_extensions: $extensions,
    required_version: ${extension.requiresCore?.let { "Some(\"$it\")" } ?: "None"},
    deprecated_by: ${extension.deprecatedby?.let { "Some(\"$it\")" } ?: "None"},
    obsoleted_by: ${extension.obsoletedby?.let { "Some(\"$it\")" } ?: "None"},
    promoted_to: ${extension.promotedto?.let { "Some(\"$it\")" } ?: "None"},
};
    """
}

/** Generates Rust modules and traits for Vulkan extensions. */
fun Registry.generateExtensionTraits() =
    """
use alloc::vec::Vec;
use core::ffi::{c_int, c_void};
use core::mem::MaybeUninit;
use core::ptr;

use super::*;

${getExtensionGroups().values
        .flatten()
        .sortedBy { it.name }
        .joinToString("") { generateExtensionTrait(it) }}
    """

/** Generates a Rust trait and implementation for a Vulkan extension. */
private fun Registry.generateExtensionTrait(extension: Extension): String {
    val provisional = if (extension.provisional) PROVISIONAL else ""
    val deprecation = generateDeprecation(extension)?.let { "\n$it" } ?: ""

    val name = "${extension.name.value.toPascalCase()}Extension"
    val type = extension.type!!.replaceFirstChar { it.uppercase() }

    val commands = extension.require.commands.mapNotNull { commands[it] }.sortedBy { it.name }

    val implAttributes =
        listOf(
            if (extension.provisional) "#[cfg(feature = \"provisional\")]" else "",
            if (deprecation.isNotEmpty()) "#[allow(deprecated)]" else "",
        ).filter { it.isNotBlank() }.joinToString("\n")

    return """
/// <${generateManualUrl(extension)}>$provisional$deprecation
pub trait $name: ${type}V1_0 {
    /// The metadata for this extension.
    #[allow(deprecated)]
    const METADATA: Extension = ${extension.name}_EXTENSION;

    ${commands.joinToString("") { generateCommandWrapper(it) }}
}

$implAttributes
impl $name for crate::$type { }
    """
}

/** Generates a Rust deprecation annotation for a Vulkan extension. */
private fun generateDeprecation(extension: Extension) =
    extension.deprecatedby?.let {
        if (it.isNotBlank()) {
            """#[deprecated(note = "deprecated in favor of `$it`")]"""
        } else {
            "#[deprecated]"
        }
    }
