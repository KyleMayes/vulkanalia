// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.support

/** Gets the extensions grouped by author. */
val getExtensionGroups =
    thunk { ->
        extensions.values.groupBy { it.name.value.split('_')[0] }
    }
