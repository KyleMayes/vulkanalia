// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.generateAliases
import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.registry.Bitflag
import com.kylemayes.generator.registry.Bitmask
import com.kylemayes.generator.registry.Registry
import java.math.BigInteger
import kotlin.math.log2

/** Generates Rust `bitflags!` structs for Vulkan bitmasks. */
fun Registry.generateBitmasks() =
    """
use bitflags::bitflags;

use crate::{Flags, Flags64};

${bitmasks.values.sortedBy { it.name }.joinToString("\n") { generateBitmask(it) }}
${generateAliases(bitmasks.keys)}
    """

/** Generates a Rust `bitflags!` struct for a Vulkan bitmask. */
private fun Registry.generateBitmask(bitmask: Bitmask): String {
    val long = bitmask.bitflags.any { it.value.bitLength() > 32 }
    val flags = "Flags" + (if (long) { "64" } else { "" })
    return """
bitflags! {
    /// <${generateManualUrl(bitmask)}>
    #[repr(transparent)]
    #[derive(Default)]
    pub struct ${bitmask.name}: $flags {
        ${generateBitflags(bitmask.bitflags).joinToString("\n        ")}
    }
}
    """
}

/** Generates Rust `bitflags!` bitflags for a list of Vulkan bitflags. */
private fun generateBitflags(bitflags: List<Bitflag>) =
    if (bitflags.isNotEmpty()) {
        bitflags.sortedBy { it.value }.map { "const ${it.name} = ${generateExpr(it.value)};" }
    } else {
        listOf(
            "/// Workaround for `bitflags!` not supporting empty bitflags.",
            "///",
            "/// <https://github.com/bitflags/bitflags/issues/179>",
            "const EMPTY = 0;",
        )
    }

/** Generates a Rust expression for a Vulkan bitflag value. */
private fun generateExpr(value: BigInteger) = when (value) {
    BigInteger.ZERO -> "0"
    BigInteger.ONE -> "1"
    else -> "1 << ${log2(value.toDouble()).toLong()}"
}
