// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.CommandType
import com.kylemayes.generator.generate.support.generateCommandWrapper
import com.kylemayes.generator.generate.support.getCommandType
import com.kylemayes.generator.registry.Command
import com.kylemayes.generator.registry.Registry

/** Generates Rust traits and implementations for Vulkan versions. */
fun Registry.generateVersionTraits(): String {
    var versions =
        """
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::ffi::{c_void, CStr};
use core::mem::MaybeUninit;
use core::ptr;

use super::*;
        """

    var previousSuffix: String? = null
    for (group in this.versions.values.groupBy { it.number }) {
        val groupNumber = group.value.first().number
        val groupCommands = group.value.flatMap { it.require.commands }.toSet()

        val suffix = groupNumber.toString().replace('.', '_')
        val commands =
            commands.values
                .filter { groupCommands.contains(it.name) }
                .groupBy { getCommandType(it) }

        versions +=
            generateVersionTrait(
                groupNumber,
                commands[CommandType.ENTRY]?.sortedBy { it.name } ?: emptyList(),
                CommandType.ENTRY,
                "EntryV$suffix",
                previousSuffix?.let { "EntryV$previousSuffix" },
                false,
            )

        versions +=
            generateVersionTrait(
                groupNumber,
                commands[CommandType.INSTANCE]?.sortedBy { it.name } ?: emptyList(),
                CommandType.INSTANCE,
                "InstanceV$suffix",
                previousSuffix?.let { "InstanceV$previousSuffix" },
                true,
            )

        versions +=
            generateVersionTrait(
                groupNumber,
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
    number: Float,
    commands: List<Command>,
    type: CommandType,
    name: String,
    extends: String?,
    handle: Boolean,
): String {
    val doc = "Vulkan $number ${type.display.lowercase()} command wrappers."
    val commandType = "${type.display}Commands"
    val commandWrappers = commands.joinToString("") { generateCommandWrapper(it) }

    return if (extends == null) {
        // ======================================================================
        // V1_0 version traits
        // ======================================================================

        val simpleImpl =
            if (handle) {
                """
impl<C: Borrow<$commandType>> $name for (C, ${type.display}) {
    #[inline] fn commands(&self) -> &$commandType { self.0.borrow() }

    #[inline] fn handle(&self) -> ${type.display} { self.1 }
}
                """
            } else {
                """
impl<C: Borrow<$commandType>> $name for C {
    #[inline] fn commands(&self) -> &$commandType { self.borrow() }
}
                """
            }

        """
/// $doc
pub trait $name {
    fn commands(&self) -> &$commandType;

    ${if (handle) "fn handle(&self) -> ${type.display};" else ""}

    $commandWrappers
}

impl $name for crate::${type.display} {
    #[inline] fn commands(&self) -> &$commandType { &self.commands }

    ${if (handle) "#[inline] fn handle(&self) -> ${type.display} { self.handle }" else ""}
}

$simpleImpl
        """
    } else {
        // ======================================================================
        // V1_1+ version traits
        // ======================================================================

        """
/// $doc
pub trait $name: $extends {
    $commandWrappers
}

impl<C: ${type.display}V1_0 + ?Sized> $name for C {}
        """
    }
}
