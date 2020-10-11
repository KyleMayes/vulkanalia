// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.support

import com.kylemayes.generator.registry.Identifier
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.Typedef

/** Generates Rust type aliases for Vulkan aliases. */
fun Registry.generateAliases(filter: Set<Identifier>) = aliases.values
    .filter { filter.contains(it.type.identifier) }
    .sortedBy { it.name }
    .joinToString("\n") { generateAlias(it) }

/** Generates a Rust type alias for a Vulkan alias. */
private fun Registry.generateAlias(alias: Typedef) =
    """
/// <${generateManualUrl(alias)}>
pub type ${alias.name} = ${alias.type.generate()};
    """.trim()
