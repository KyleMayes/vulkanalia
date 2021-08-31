// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.generateAliases
import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.registry.Bitflag
import com.kylemayes.generator.registry.Bitmask
import com.kylemayes.generator.registry.Registry
import java.math.BigInteger

/** Generates Rust `bitflags!` structs for Vulkan bitmasks. */
fun Registry.generateBitmasks() =
    """
use bitflags::bitflags;

use crate::{Flags, Flags64};

${bitmasks.values
        .sortedBy { it.name }
        .joinToString("\n") { generateBitmask(it) }}

${generateAliases(bitmasks.keys)}
    """

/** Generates a Rust `bitflags!` struct for a Vulkan bitmask. */
private fun Registry.generateBitmask(bitmask: Bitmask): String {
    val long = bitmask.bitflags.any { it.value.bitLength() > 32 }
    val repr = "Flags" + (if (long) { "64" } else { "" })

    val values = bitmask.bitflags.associateBy { it.value }
    val flags = generateBitflags(values, bitmask.bitflags).joinToString("\n        ")
    val block = if (flags.isNotBlank()) {
        "{\n        $flags\n    }"
    } else {
        "{ }"
    }

    return """
bitflags! {
    /// <${generateManualUrl(bitmask)}>
    #[repr(transparent)]
    #[derive(Default)]
    pub struct ${bitmask.name}: $repr $block
}
    """
}

/** Generates Rust `bitflags!` bitflags for a list of Vulkan bitflags. */
private fun generateBitflags(values: Map<BigInteger, Bitflag>, bitflags: List<Bitflag>) =
    if (bitflags.isNotEmpty()) {
        bitflags
            .sortedBy { it.value }
            .map { "const ${it.name} = ${generateExpr(values, it.value)};" }
    } else {
        emptyList()
    }

/** Generates a Rust expression for a Vulkan bitflag value. */
private fun generateExpr(values: Map<BigInteger, Bitflag>, value: BigInteger): String {
    val bits = (0..value.bitLength())
        .map { it to value.testBit(it) }
        .filter { it.second }
        .map { it.first }

    return if (bits.isEmpty()) {
        "0"
    } else if (bits.size == 1) {
        val bit = bits[0]
        if (bit == 0) { "1" } else { "1 << $bit" }
    } else if (bits.size == 31) {
        return "i32::MAX as u32"
    } else {
        return bits
            .map { values[BigInteger.valueOf(1L.shl(it))]!!.name }
            .joinToString(" | ") { "Self::$it.bits" }
    }
}
