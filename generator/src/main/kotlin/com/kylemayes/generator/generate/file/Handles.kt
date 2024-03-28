// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.generateAliases
import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.registry.Handle
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.support.toSnakeCase

/** Generates Rust structs for Vulkan handles. */
fun Registry.generateHandles() =
    """
use core::fmt;
use core::hash::Hash;

use crate::ObjectType;

/// A Vulkan handle type.
pub trait Handle: Copy + Clone + fmt::Debug + PartialEq + Eq + Hash + Default + Sized {
    /// The underlying type for this handle type.
    type Repr;

    /// The object type for this handle type.
    const TYPE: ObjectType;

    /// Constructs a null instance of this handle type.
    fn null() -> Self;

    /// Constructs an instance of this handle type with the supplied underlying value.
    fn from_raw(value: Self::Repr) -> Self;

    /// Gets the underlying value for this handle.
    fn as_raw(self) -> Self::Repr;

    /// Returns whether this handle is a null handle.
    fn is_null(self) -> bool;
}

${handles.values
        .sortedBy { it.name }
        .joinToString("\n") { generateHandle(it) }}

${generateAliases(handles.keys)}
    """

/** Generates a Rust struct for a Vulkan handle. */
private fun Registry.generateHandle(handle: Handle): String {
    val repr = if (handle.dispatchable) "usize" else "u64"
    val type = handle.name.value.toSnakeCase().uppercase()
    return """
/// <${generateManualUrl(handle)}>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ${handle.name}($repr);

impl Handle for ${handle.name} {
    type Repr = $repr;

    const TYPE: ObjectType = ObjectType::$type;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for ${handle.name} {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for ${handle.name} {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${handle.name}({:p})", self.0 as *const u8)
    }
}
    """
}
