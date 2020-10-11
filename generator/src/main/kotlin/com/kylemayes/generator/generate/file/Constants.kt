// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.registry.Constant
import com.kylemayes.generator.registry.Registry

/** Generates Rust constants for Vulkan constants. */
fun Registry.generateConstants() =
    constants.values.sortedBy { it.name }.joinToString("\n") { generateConstant(it) }

/** Generates a Rust constant for a Vulkan constant. */
private fun generateConstant(constant: Constant) =
    "pub const ${constant.name}: ${constant.type.generate()} = ${constant.expr};"
