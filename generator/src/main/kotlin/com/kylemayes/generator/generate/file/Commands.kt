// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.CommandType
import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.generate.support.getCommandType
import com.kylemayes.generator.generate.support.getUnsupportedExtensionEntities
import com.kylemayes.generator.registry.Command
import com.kylemayes.generator.registry.Registry

/** Generates Rust type aliases for Vulkan commands. */
fun Registry.generateCommands() =
    """
use std::os::raw::{c_char, c_int, c_void};

use crate::*;

${commands.values
        .filter { !getUnsupportedExtensionEntities().contains(it.name) }
        .sortedBy { it.name }
        .joinToString("\n") { generateCommand(it) }}
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

/** Generates Rust structs for Vulkan commands. */
fun Registry.generateCommandStructs(): String {
    val structs = commands.values
        .groupBy { getCommandType(it) }
        .entries
        .sortedBy { it.key.display }
        .joinToString("") {
            val supported = it.value
                .filter { c -> !getUnsupportedExtensionEntities().contains(c.name) }
                .sortedBy { c -> c.name }
            generateCommandStruct(it.key, supported)
        }
    return """
use std::mem;
use std::os::raw::{c_char, c_int, c_void};

use super::*;

$structs
    """
}

/** Generates a Rust struct for a group of Vulkan commands of the same type. */
private fun Registry.generateCommandStruct(type: CommandType, commands: List<Command>) =
    """
/// Loaded Vulkan ${type.display.toLowerCase()} commands.
#[derive(Copy, Clone)]
pub struct ${type.display}Commands {
    ${commands.joinToString { "pub ${it.name}: PFN_${it.name.original}" }}
}

impl ${type.display}Commands {
    #[inline]
    pub unsafe fn load(mut loader: impl FnMut(*const c_char) -> Option<unsafe extern "system" fn()>) -> Self {
        Self { ${commands.joinToString { generateLoad(it) }} }
    }
}
    """

/** Generates a Rust struct field-value pair to load a command. */
private fun Registry.generateLoad(command: Command) =
    """
${command.name}: {
    let value = loader(b"${command.name.original}\0".as_ptr().cast());
    if let Some(value) = value {
        mem::transmute(value)
    } else {
        ${generateSignature(command, "fallback")} {
            panic!("could not load ${command.name.original}")
        }
        fallback
    }
}
    """

/** Generates a Rust function signature for a Vulkan command. */
private fun generateSignature(command: Command, name: String = ""): String {
    val params = command.params.joinToString { "_${it.name.value.removePrefix("_")}: ${it.type.generateForCommand()}" }
    val actual = command.result.generateForCommand()
    val result = if (actual != "c_void") { " -> $actual" } else { "" }
    return "unsafe extern \"system\" fn $name($params)$result"
}
