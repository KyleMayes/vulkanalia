// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.registry

import com.kylemayes.generator.generate.generateRustFiles
import com.kylemayes.generator.generate.support.getCommandType
import com.kylemayes.generator.generate.support.getStructExtensions
import com.kylemayes.generator.generate.support.manualUrls
import com.kylemayes.generator.support.toPascalCase

/**
 * Generates a documentation index for the entities in a Vulkan API registry.
 *
 * The returned index maps Vulkan entities (e.g., structs, enums, bitmasks) to
 * the relevant Vulkan manual page and also maps the Rust items generated for
 * the Vulkan entities to the relevant `vulkanalia` documentation page.
 *
 * Each line in the index is an entity name (e.g., `VkInstanceCreateInfo` or
 * `vk::InstanceCreateInfo`) and the relevant documentation URL separated by
 * tabs. URLs for `vulkanalia` documentation pages will contain the string
 * `%VERSION%` which should be replaced with the crate version in use.
 */
fun Registry.indexEntities(): String {
    generateRustFiles(this)
    val entities = manualUrls.toMutableList()

    entities.addEntity("vk::make_version", "/vk/fn.make_version.html")
    entities.addEntity("vk::version_major", "/vk/fn.version_major.html")
    entities.addEntity("vk::version_minor", "/vk/fn.version_minor.html")
    entities.addEntity("vk::version_patch", "/vk/fn.version_patch.html")

    entities.addEntity("vk::StaticCommands", "/vk/struct.StaticCommands.html")
    entities.addEntity("vk::EntryCommands", "/vk/struct.EntryCommands.html")
    entities.addEntity("vk::InstanceCommands", "/vk/struct.InstanceCommands.html")
    entities.addEntity("vk::DeviceCommands", "/vk/struct.DeviceCommands.html")

    entities.addEntities("type", aliases)
    entities.addEntities("type", basetypes)
    entities.addEntities("struct", bitmasks)
    entities.addSubentities("struct", "associatedconstant", bitmasks) { it.bitflags }
    entities.addEntities("constant", constants)
    entities.addEntities("struct", enums)
    entities.addSubentities("struct", "associatedconstant", enums) { it.variants }
    entities.addEntities("type", functions)
    entities.addEntities("struct", handles)
    entities.addEntities("struct", structs)
    entities.addEntities("union", unions)

    val versionCommands = mutableMapOf<Identifier, String>()
    val extensionCommands = mutableMapOf<Identifier, String>()

    // Add builder structs and extends traits.

    for (struct in structs.values) {
        val builder = "${struct.name}Builder"
        entities.addEntity("vk::$builder", "/vk/struct.$builder.html")
        if (getStructExtensions().containsKey(struct.name)) {
            val extends = "Extends${struct.name}"
            entities.addEntity("vk::$extends", "/vk/trait.$extends.html")
        }
    }

    // Add version traits.

    for (version in versions.values) {
        val suffix = version.number.toString().replace('.', '_')
        entities.addEntity("vk::EntryV$suffix", "/vk/trait.EntryV$suffix.html")
        entities.addEntity("vk::InstanceV$suffix", "/vk/trait.InstanceV$suffix.html")
        entities.addEntity("vk::DeviceV$suffix", "/vk/trait.DeviceV$suffix.html")
        for (command in version.require.commands) {
            val type = getCommandType(commands[command] ?: error("Missing command."))
            versionCommands[command] = "${type.display}V$suffix"
        }
    }

    // Add extension constants.

    for (extension in extensions.values) {
        val name = "${extension.name}_EXTENSION"
        entities.addEntity("vk::$name", "/vk/constant.$name.html")
    }

    // Add extension traits.

    for (extension in extensions.values) {
        val name = "${extension.name.value.toPascalCase()}Extension"
        entities.addEntity("vk::$name", "/vk/trait.$name.html")
        for (command in extension.require.commands) {
            extensionCommands[command] = name
        }
    }

    // Add command trait methods and types.

    for (command in commands.values) {
        val trait = extensionCommands[command.name] ?: versionCommands[command.name] ?: error("!")
        entities.addEntity(command.name.value, "/vk/trait.$trait.html#method.${command.name}")
        val pfn = "PFN_${command.name.original}"
        entities.addEntity("vk::$pfn", "/vk/type.$pfn.html")
    }

    // Add success codes and error codes.

    val results = (enums["VkResult".intern()] ?: error("Missing result.")).variants
    val (successes, errors) = results.partition { it.value >= 0 }
    val success = Enum(name = "SuccessCode".intern(), variants = successes.toMutableList())
    val error = Enum(name = "ErrorCode".intern(), variants = errors.toMutableList())
    entities.addSubentities("struct", "associatedconstant", mapOf(success.name to success)) { it.variants }
    entities.addSubentities("struct", "associatedconstant", mapOf(error.name to error)) {
        // Remove `ERROR_` prefix to match `ErrorCode` variants.
        it.variants.map { v -> v.copy(name = v.name.value.replace(Regex("^ERROR_"), "").intern()) }
    }

    return entities.joinToString("\n") { "${it.first}\t${it.second}" }
}

private typealias Index = MutableList<Pair<String, String>>

private fun Index.addEntity(name: String, path: String) =
    add(name to "https://docs.rs/vulkanalia/%VERSION%/vulkanalia$path")

private fun Index.addEntities(type: String, entities: Map<Identifier, Entity>) {
    for (name in entities.keys) {
        addEntity("vk::$name", "/vk/$type.$name.html")
    }
}

private fun <T : Entity> Index.addSubentities(
    type: String,
    subtype: String,
    entities: Map<Identifier, T>,
    subentities: (T) -> Iterable<Entity>,
) {
    for (entity in entities.values) {
        for (subentity in subentities(entity)) {
            val name = "vk::${entity.name}::${subentity.name}"
            val path = "/vk/$type.${entity.name}.html#$subtype.${subentity.name}"
            addEntity(name, path)
        }
    }
}
