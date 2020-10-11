// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.registry.Registry

/** Generates Rust constant functions for Vulkan macros. */
fun Registry.generateMacros() =
    """
/// <${generateManualUrl("VK_MAKE_VERSION")}>
#[inline]
pub const fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}

/// <${generateManualUrl("VK_VERSION_MAJOR")}>
#[inline]
pub const fn version_major(version: u32) -> u32 {
    version >> 22
}

/// <${generateManualUrl("VK_VERSION_MINOR")}>
#[inline]
pub const fn version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3FF
}

/// <${generateManualUrl("VK_VERSION_PATCH")}>
#[inline]
pub const fn version_patch(version: u32) -> u32 {
    version & 0xFFF
}
    """
