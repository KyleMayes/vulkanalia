// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.generate.support.getExtensionGroups
import com.kylemayes.generator.registry.Extension
import com.kylemayes.generator.registry.Registry

/** Generates Rust modules and constants for Vulkan extensions. */
fun Registry.generateExtensions() =
    """
use std::os::raw::c_char;

use crate::constants::MAX_EXTENSION_NAME_SIZE;

/// A Vulkan extension name.
pub type ExtensionName = [c_char; MAX_EXTENSION_NAME_SIZE];

${getExtensionGroups().entries.sortedBy { it.key }.joinToString("") { generateGroup(it.key, it.value) }}

#[inline]
const fn name(bytes: &[u8]) -> ExtensionName {
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

/** Generates a Rust module and constants for a group of Vulkan extensions. */
private fun Registry.generateGroup(author: String, extensions: List<Extension>) =
    """
/// `$author` extensions.
pub mod ${author.toLowerCase()} {
    use super::*;

    ${extensions.sortedBy { it.name.value }.joinToString("\n") { generateExtension(author, it) }}
}
    """

/** Generates a Rust constant for a Vulkan extension. */
private fun Registry.generateExtension(author: String, extension: Extension): String {
    val deprecation = generateDeprecation(extension)?.let { "\n$it" } ?: ""

    val name = extension.name.value
        .removePrefix("${author}_")
        .replace(Regex("^([0-9])"), "_$1")

    return """
/// <${generateManualUrl(extension)}>$deprecation
pub const $name: ExtensionName = name(b"${extension.name.original}");
    """.trim()
}

/** Generates a Rust deprecation annotation for a Vulkan extension. */
private fun generateDeprecation(extension: Extension) = extension.deprecatedby?.let {
    if (it.isNotBlank()) {
        """#[deprecated(note = "deprecated in favor of `$it`")]"""
    } else {
        "#[deprecated]"
    }
}
