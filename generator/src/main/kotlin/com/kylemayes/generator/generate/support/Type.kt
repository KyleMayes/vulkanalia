// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.support

import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.Type
import com.kylemayes.generator.registry.getLength

/** Gets the length of this Vulkan array type as an integer. */
fun Registry.getLengthValue(type: Type) =
    type.getLength()?.let { constants[it]?.expr?.toLong() ?: it.value.toLong() }
