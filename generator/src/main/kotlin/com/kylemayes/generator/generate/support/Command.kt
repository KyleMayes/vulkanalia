// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.support

import com.kylemayes.generator.registry.Command
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.getIdentifier

// The commands whose types cannot be determined by their extension membership or first parameter.
private val STATIC = setOf("vkGetInstanceProcAddr", "vkGetDeviceProcAddr")
private val ENTRY = setOf("vkCreateInstance", "vkEnumerateInstanceExtensionProperties", "vkEnumerateInstanceLayerProperties", "vkEnumerateInstanceVersion")
private val INSTANCE = setOf("vkCreateDevice")

/** A type of command which indicates how it is loaded. */
enum class CommandType(val display: String) {
    /** A command which is loaded from the Vulkan dynamic library. */
    STATIC("Static"),
    /** A command which is loaded with `vkGetInstanceProcAddr` and a null instance. */
    ENTRY("Entry"),
    /** A command which is loaded with `vkGetInstanceProcAddr` and a valid instance. */
    INSTANCE("Instance"),
    /** A command which is loaded with `vkGetDeviceProcAddr` and a valid device. */
    DEVICE("Device"),
}

/** Gets the command type of a Vulkan command. */
fun Registry.getCommandType(command: Command): CommandType {
    val extensionCommandTypes = getExtensionCommandTypes()
    return when {
        STATIC.contains(command.name.original) -> CommandType.STATIC
        ENTRY.contains(command.name.original) -> CommandType.ENTRY
        INSTANCE.contains(command.name.original) -> CommandType.INSTANCE
        else -> {
            val extension = extensionCommandTypes[command.name]
            if (extension != null) return extension
            val first = command.params.getOrNull(0)?.type?.getIdentifier()?.value
            if (first == "CommandBuffer" || first == "Device" || first == "Queue") {
                CommandType.DEVICE
            } else {
                CommandType.INSTANCE
            }
        }
    }
}

/** Gets the command types for the Vulkan extension commands. */
private val getExtensionCommandTypes = thunk { ->
    extensions.values
        .filter { e -> e.type != null }
        .flatMap { e -> e.require.commands.map { c -> Pair(e, c) } }
        .associate { (e, c) -> c to CommandType.valueOf(e.type!!.toUpperCase()) }
}

/** Gets the non-`SUCCESS` and non-`INCOMPLETE` success codes for a Vulkan command. */
val getCommandSuccessCodes = thunk { command: Command ->
    command.successcodes
        .filter { it.value != "SUCCESS" && it.value != "INCOMPLETE" }
        .toSet()
}
