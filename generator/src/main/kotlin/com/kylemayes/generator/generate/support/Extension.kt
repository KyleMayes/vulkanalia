// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.support

import com.kylemayes.generator.registry.intern
import com.kylemayes.generator.support.isSupported

/** Gets the supported extensions grouped by author. */
val getSupportedExtensionGroups = thunk { ->
    extensions.values
        .filter { isSupported(it) }
        .groupBy { it.name.value.substring(0, it.name.value.indexOf('_')) }
}

/** Gets the entities added by unsupported extensions. */
val getUnsupportedExtensionEntities = thunk { ->
    extensions.values
        .filter { !isSupported(it) }
        .flatMap { it.require.commands + it.require.types.map { n -> n.intern() } }
        .toSet()
}
