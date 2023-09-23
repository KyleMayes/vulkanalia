// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.CommandType
import com.kylemayes.generator.generate.support.generateCommandWrapper
import com.kylemayes.generator.generate.support.getCommandType
import com.kylemayes.generator.registry.Command
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.Version

/** Generates Rust traits and implementations for Vulkan versions. */
fun Registry.generateVersionTraits(): String {
    var versions =
        """
use alloc::vec::Vec;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr;

use super::*;
        """

    var previousSuffix: String? = null
    for (version in this.versions.values) {
        val suffix = version.number.toString().replace('.', '_')
        val commands = commands.values
            .filter { version.require.commands.contains(it.name) }
            .groupBy { getCommandType(it) }

        versions += generateVersionTrait(
            version,
            commands[CommandType.ENTRY]?.sortedBy { it.name } ?: emptyList(),
            CommandType.ENTRY,
            "EntryV$suffix",
            previousSuffix?.let { "EntryV$previousSuffix" },
            false,
        )

        versions += generateVersionTrait(
            version,
            commands[CommandType.INSTANCE]?.sortedBy { it.name } ?: emptyList(),
            CommandType.INSTANCE,
            "InstanceV$suffix",
            previousSuffix?.let { "InstanceV$previousSuffix" },
            true,
        )

        versions += generateVersionTrait(
            version,
            commands[CommandType.DEVICE]?.sortedBy { it.name } ?: emptyList(),
            CommandType.DEVICE,
            "DeviceV$suffix",
            previousSuffix?.let { "DeviceV$previousSuffix" },
            true,
        )

        previousSuffix = suffix
    }

    return versions
}

/** Generates a Rust trait and implementation for a Vulkan version and command type. */
private fun Registry.generateVersionTrait(
    version: Version,
    commands: List<Command>,
    type: CommandType,
    name: String,
    extends: String?,
    handle: Boolean,
) =
    """
/// Vulkan ${version.number} ${type.display.lowercase()} command wrappers.
pub trait $name${extends?.let { ": $it" } ?: ""} {
    ${if (extends == null) { "fn commands(&self) -> &${type.display}Commands;\n" } else { "" }}
    ${if (handle && extends == null) { "fn handle(&self) -> ${type.display};\n" } else { "" }}
    ${commands.joinToString("") { generateCommandWrapper(it) }}
}

impl $name for crate::${type.display} {
    ${if (extends == null) { "#[inline] fn commands(&self) -> &${type.display}Commands { &self.commands }\n" } else { "" }}
    ${if (handle && extends == null) { "#[inline] fn handle(&self) -> ${type.display} { self.handle }\n" } else { "" }}
}
    """
