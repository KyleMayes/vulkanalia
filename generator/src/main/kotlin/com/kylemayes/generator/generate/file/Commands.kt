// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.CommandType
import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.generate.support.getCommandType
import com.kylemayes.generator.registry.Command
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.getIdentifier

/** Generates Rust type aliases for Vulkan commands. */
fun Registry.generateCommands() =
    """
use core::ffi::{c_char, c_int, c_void};

use crate::*;

${commands.values
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
    val structs =
        commands.values
            .groupBy { getCommandType(it) }
            .entries
            .sortedBy { it.key.display }
            .joinToString("") {
                val supported = it.value.sortedBy { c -> c.name }
                generateCommandStruct(it.key, supported)
            }
    return """
use core::mem;
use core::ffi::{c_char, c_int, c_void};

use super::*;

$structs
    """
}

private val loader = "impl FnMut(*const c_char) -> Option<unsafe extern \"system\" fn()>"

/** Generates a Rust struct for a group of Vulkan commands of the same type. */
private fun Registry.generateCommandStruct(
    type: CommandType,
    commands: List<Command>,
) = """
/// Loaded Vulkan ${type.display.lowercase()} commands.
#[derive(Copy, Clone)]
pub struct ${type.display}Commands {
    ${commands.joinToString { "pub ${it.name}: PFN_${it.name.original}" }}
}

impl ${type.display}Commands {
    #[inline]
    pub unsafe fn load(
        ${if (type == CommandType.DEVICE) "mut instance_loader: $loader," else ""}
        mut loader: $loader,
    ) -> Self {
        Self { ${commands.joinToString { generateLoad(it, type) }} }
    }
}
    """

/** Generates a Rust struct field-value pair to load a command. */
private fun Registry.generateLoad(
    command: Command,
    type: CommandType,
): String {
    // It seems that device extension commands are instance-level commands if
    // they take a `VkPhysicalDevice` as their first parameter.
    val first = command.params.getOrNull(0)?.type?.getIdentifier()?.value
    val loader =
        if (type == CommandType.DEVICE && first == "PhysicalDevice") {
            "instance_loader"
        } else {
            "loader"
        }

    return """
${command.name}: {
    let value = $loader(c"${command.name.original}".as_ptr());
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
}

/** Generates a Rust function signature for a Vulkan command. */
private fun generateSignature(
    command: Command,
    name: String = "",
): String {
    val params = command.params.joinToString { "_${it.name.value.removePrefix("_")}: ${it.type.generateForCommand()}" }
    val actual = command.result.generateForCommand()
    val result = if (actual != "c_void") " -> $actual" else ""
    return "unsafe extern \"system\" fn $name($params)$result"
}
