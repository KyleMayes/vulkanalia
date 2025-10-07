// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.support

import com.kylemayes.generator.registry.Command
import com.kylemayes.generator.registry.Extension

/** Gets the extensions grouped by author. */
val getExtensionGroups =
    thunk { ->
        extensions.values.groupBy { it.name.value.split('_')[0] }
    }

data class ExtensionTrait(
    val type: CommandType,
    val extension: Extension,
    val commands: List<Command>,
)

val getExtensionTraits =
    thunk { ->
        getExtensionGroups()
            .values
            .flatten()
            .sortedBy { it.name }
            .flatMap { ext ->
                val type = CommandType.valueOf(ext.type!!.uppercase())
                val commands = ext.require.commands.mapNotNull { commands[it] }.sortedBy { it.name }

                // Create separate traits for instance-level and device-level
                // device extension commands if necessary.
                val (instance, other) = commands.partition { isInstanceLevelDeviceExtensionCommand(it) }

                val traits = mutableListOf<ExtensionTrait>()

                if (other.isNotEmpty()) {
                    traits.add(ExtensionTrait(type, ext, other))
                }

                if (instance.isNotEmpty()) {
                    traits.add(ExtensionTrait(CommandType.INSTANCE, ext, instance))
                }

                traits
            }
    }
