// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate

import com.kylemayes.generator.generate.file.generateBitmasks
import com.kylemayes.generator.generate.file.generateBuilders
import com.kylemayes.generator.generate.file.generateChains
import com.kylemayes.generator.generate.file.generateCommandStructs
import com.kylemayes.generator.generate.file.generateCommands
import com.kylemayes.generator.generate.file.generateConstants
import com.kylemayes.generator.generate.file.generateEnums
import com.kylemayes.generator.generate.file.generateExtensionTraits
import com.kylemayes.generator.generate.file.generateExtensions
import com.kylemayes.generator.generate.file.generateFunctions
import com.kylemayes.generator.generate.file.generateHandles
import com.kylemayes.generator.generate.file.generateHeaders
import com.kylemayes.generator.generate.file.generateMacros
import com.kylemayes.generator.generate.file.generateResultEnums
import com.kylemayes.generator.generate.file.generateStructs
import com.kylemayes.generator.generate.file.generateTypedefs
import com.kylemayes.generator.generate.file.generateUnions
import com.kylemayes.generator.generate.file.generateVersionTraits
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.support.rustfmt
import mu.KotlinLogging
import java.nio.file.Files
import java.nio.file.Path

private val log = KotlinLogging.logger { /* */ }

/** The additional `bindgen` options for the Vulkan video headers. */
private val videoOptions =
    listOf(
        "--allowlist-item", "StdVideo.*",
        "--allowlist-item", "STD_VIDEO_.*",
        "--no-prepend-enum-name",
        "--default-enum-style", "newtype_global",
        "--with-derive-custom-enum", ".*=Default",
    )

/** Generates Rust files for a Vulkan API registry and Vulkan video headers. */
fun generateRustFiles(
    registry: Registry,
    video: Map<String, String>,
) = listOf(
    generateRustFile("vulkanalia-sys", "bitmasks.rs", registry.generateBitmasks()),
    generateRustFile("vulkanalia-sys", "commands.rs", registry.generateCommands()),
    generateRustFile("vulkanalia-sys", "constants.rs", registry.generateConstants()),
    generateRustFile("vulkanalia-sys", "enums.rs", registry.generateEnums()),
    generateRustFile("vulkanalia-sys", "extensions.rs", registry.generateExtensions()),
    generateRustFile("vulkanalia-sys", "functions.rs", registry.generateFunctions()),
    generateRustFile("vulkanalia-sys", "handles.rs", registry.generateHandles()),
    generateRustFile("vulkanalia-sys", "macros.rs", registry.generateMacros()),
    generateRustFile("vulkanalia-sys", "structs.rs", registry.generateStructs()),
    generateRustFile("vulkanalia-sys", "typedefs.rs", registry.generateTypedefs()),
    generateRustFile("vulkanalia-sys", "unions.rs", registry.generateUnions()),
    generateRustFile("vulkanalia-sys", "video.rs", generateHeaders("video", video, videoOptions)),
    generateRustFile("vulkanalia", "vk/builders.rs", registry.generateBuilders()),
    generateRustFile("vulkanalia", "vk/chains.rs", registry.generateChains()),
    generateRustFile("vulkanalia", "vk/commands.rs", registry.generateCommandStructs()),
    generateRustFile("vulkanalia", "vk/enums.rs", registry.generateResultEnums()),
    generateRustFile("vulkanalia", "vk/extensions.rs", registry.generateExtensionTraits()),
    generateRustFile("vulkanalia", "vk/versions.rs", registry.generateVersionTraits()),
)

/** A generated Rust file. */
data class File(
    val path: Path,
    var contents: String,
    var matches: Boolean? = null,
) {
    /** Attempts to format this generated Rust file. */
    fun format(): Boolean {
        return try {
            contents = rustfmt(contents)
            true
        } catch (e: Throwable) {
            log.error { "$path could not be formatted." }
            e.printStackTrace(System.err)
            false
        }
    }

    /** Checks whether this generated Rust file matches the file on disk. */
    fun matches(directory: Path): Boolean {
        val full = directory.resolve(path)

        matches =
            if (Files.exists(full)) {
                val contents = Files.readString(full)
                val matches = contents == this.contents
                if (!matches) log.info { "$path does not match file on disk." }
                matches
            } else {
                log.info { "$path does not exist on disk." }
                false
            }

        return matches!!
    }

    /** Writes this generated Rust file to disk. */
    fun write(directory: Path) {
        if (matches != true) {
            log.info { "Writing $path..." }
            Files.writeString(directory.resolve(path), contents)
        }
    }
}

/** Generates a Rust file for one of the Rust crates. */
private fun generateRustFile(
    crate: String,
    name: String,
    contents: String,
): File {
    val path = Path.of(crate).resolve("src").resolve(name)
    log.info { "Generating $path..." }
    return File(path, "$PREFIX\n$contents".replace(Regex("\\r\\n?"), "\n"))
}

/** The Rust file prefix. */
private const val PREFIX =
    """
// SPDX-License-Identifier: Apache-2.0

// DO NOT EDIT.
//
// This file has been generated by the Kotlin project in the `generator`
// directory from a Vulkan API registry.

#![allow(
    non_camel_case_types,
    non_snake_case,
    clippy::bad_bit_mask,
    clippy::let_unit_value,
    clippy::missing_safety_doc,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::unnecessary_cast,
    clippy::upper_case_acronyms,
    clippy::useless_transmute
)]
    """
