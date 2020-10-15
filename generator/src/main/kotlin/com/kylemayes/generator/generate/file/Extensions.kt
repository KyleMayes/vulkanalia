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

use crate::MAX_EXTENSION_NAME_SIZE;

/// A Vulkan extension name.
pub type ExtensionName = [c_char; MAX_EXTENSION_NAME_SIZE];

${getExtensionGroups().values.flatten().sortedBy { it.name }.joinToString("") { generateExtension(it) }}

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

/** Generates a Rust constant for a Vulkan extension. */
private fun Registry.generateExtension(extension: Extension): String {
    val deprecation = generateDeprecation(extension)?.let { "\n$it" } ?: ""
    return """
/// <${generateManualUrl(extension)}>$deprecation
pub const ${extension.name}_EXTENSION: ExtensionName = name(b"${extension.name.original}");
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
