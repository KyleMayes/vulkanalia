// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.registry.Function
import com.kylemayes.generator.registry.Registry

/** Generates Rust type aliases for Vulkan function pointer types. */
fun Registry.generateFunctions() =
    """
use core::ffi::{c_char, c_void};

use crate::*;

${functions.values
        .sortedBy { it.name }
        .joinToString("\n") { generateFunction(it) }}
    """

/** Generates a Rust type alias for a Vulkan function pointer type. */
private fun Registry.generateFunction(function: Function): String {
    val params = function.params.joinToString { it.generateForCommand() }
    val result = function.result?.let { "-> ${it.generateForCommand()}" } ?: ""
    return """
/// <${generateManualUrl(function)}>
pub type ${function.name} = Option<unsafe extern "system" fn ($params)$result>;
    """
}
