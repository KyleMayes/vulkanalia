// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.registry.Command
import com.kylemayes.generator.registry.Registry

/** Generates Rust type aliases for Vulkan commands. */
fun Registry.generateCommands() =
    """
#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_int, c_void};

use crate::vk::*;

${commands.values.sortedBy { it.name }.joinToString("\n") { generateCommand(it) }}
    """

/** Generates a Rust type alias for a Vulkan command. */
private fun Registry.generateCommand(command: Command): String {
    val aliased = commandAliases[command.name]
    val type = aliased?.let { "PFN_${it.original}" } ?: generateSignature(command)
    return """
/// <${generateManualUrl(command)}>
pub type PFN_${command.name.original} = $type;
    """
}

/** Generates a Rust function signature for a Vulkan command. */
private fun generateSignature(command: Command, name: String = ""): String {
    val params = command.params.joinToString { "_${it.name.value.removePrefix("_")}: ${it.type.generateForCommand()}" }
    val actual = command.result.generateForCommand()
    val result = if (actual != "c_void") { " -> $actual" } else { "" }
    return "extern \"system\" fn $name($params)$result"
}
