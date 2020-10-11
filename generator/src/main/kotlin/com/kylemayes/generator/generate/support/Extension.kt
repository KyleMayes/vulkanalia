// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.support

/** Gets the non-disabled extensions grouped by author. */
val getExtensionGroups = thunk { ->
    extensions.values
        .filter { it.supported != "disabled" }
        .groupBy { it.name.value.substring(0, it.name.value.indexOf('_')) }
}
