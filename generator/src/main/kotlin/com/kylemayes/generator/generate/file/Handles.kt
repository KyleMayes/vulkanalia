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
use core::ffi::c_void;
use core::fmt;
use core::hash::Hash;

use crate::*;

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

/// A [`Handle`] with a representation of `usize` (a pointer to an opaque type).
///
/// <https://docs.vulkan.org/spec/latest/chapters/fundamentals.html#fundamentals-objectmodel-overview>
pub trait DispatchableHandle: Handle<Repr = usize> {
    /// Gets the dispatch key for this dispatchable handle.
    ///
    /// As described in the
    /// [driver interface to the Vulkan loader](https://vulkan.lunarg.com/doc/view/latest/linux/LoaderDriverInterface.html#driver-dispatchable-object-creation),
    /// all dispatchable handles created by Vulkan drivers can be cast to
    /// `void**` (a.k.a., `*mut *mut c_void`). The Vulkan loader will always
    /// replace the first entry in this array of pointers with a pointer to the
    /// dispatch table (created and owned by the loader) for the dispatchable
    /// handle. This guarantee can be used to extract a key from a dispatchable
    /// handle that is stable and unique to an [`Instance`] or [`Device`] or the
    /// [`Instance`] or [`Device`] another type of dispatchable device is
    /// associated with (e.g., a [`CommandBuffer`] is associated with the
    /// [`Device`] it was created for).
    ///
    /// # Safety
    ///
    /// This handle must be a valid Vulkan dispatchable handle.
    unsafe fn dispatch_key(self) -> usize {
        let pointer = self.as_raw() as *mut *mut c_void;
        (unsafe { *pointer }) as usize
    }
}

impl<H: Handle<Repr = usize>> DispatchableHandle for H {}

/// A [`Handle`] with a representation of `u64` (a value with an implementation-dependent meaning).
///
/// <https://docs.vulkan.org/spec/latest/chapters/fundamentals.html#fundamentals-objectmodel-overview>
pub trait NonDispatchableHandle: Handle<Repr = u64> {}

impl<H: Handle<Repr = u64>> NonDispatchableHandle for H {}

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
