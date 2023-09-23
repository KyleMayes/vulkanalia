//! SPIR-V bytecode.

use alloc::alloc::{alloc as malloc, dealloc as free, Layout};
use core::fmt;
use core::slice;

#[cfg(all(feature = "no_std_error", not(feature = "std")))]
use core::error;
#[cfg(feature = "std")]
use std::error;

/// An error raised by a failure to construct a [`Bytecode`].
#[derive(Clone, Debug, PartialEq)]
pub enum BytecodeError {
    /// Indicates a failure to allocate a SPIR-V bytecode buffer.
    Alloc,
    /// Indicates an invalid SPIR-V bytecode buffer length.
    Length(usize),
}

impl fmt::Display for BytecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BytecodeError::*;
        match self {
            Alloc => write!(f, "failed to allocate SPIR-V bytecode buffer"),
            Length(length) => write!(f, "invalid SPIR-V bytecode buffer length ({length})"),
        }
    }
}

#[cfg(any(feature = "std", feature = "no_std_error"))]
impl error::Error for BytecodeError {}

/// A 4-byte aligned SPIR-V bytecode buffer.
///
/// This helper struct can be used to ensure the correct alignment of the SPIR-V
/// bytecode for a shader before providing it to a
/// [`crate::vk::ShaderModuleCreateInfo`] to create a shader module.
///
/// ```
/// # use vulkanalia::bytecode::Bytecode;
/// # use vulkanalia::prelude::v1_0::*;
/// let vert_bytes = include_bytes!("../../tutorial/shaders/09/vert.spv");
/// let vert_bytecode = Bytecode::new(vert_bytes).unwrap();
/// let vert_info = vk::ShaderModuleCreateInfo::builder()
///     .code_size(vert_bytecode.code_size())
///     .code(vert_bytecode.code());
/// ```
pub struct Bytecode(*mut u8, usize);

impl Bytecode {
    /// Copies a SPIR-V bytecode slice into a new 4-byte aligned SPIR-V bytecode buffer.
    pub fn new(bytecode: &[u8]) -> Result<Self, BytecodeError> {
        if bytecode.is_empty() || bytecode.len() % 4 != 0 {
            return Err(BytecodeError::Length(bytecode.len()));
        }

        let layout = Layout::from_size_align(bytecode.len(), 4).unwrap();
        debug_assert!(layout.size() != 0);

        // SAFETY: Safe because `layout` is guaranteed to have non-zero size
        // because `size` is always non-zero and `align` is always 4.
        let pointer = unsafe { malloc(layout) };
        if pointer.is_null() {
            return Err(BytecodeError::Alloc);
        }

        // SAFETY: Safe because `pointer` points to a single readable allocation
        // of the approriate size and alignment.
        let slice = unsafe { slice::from_raw_parts_mut(pointer, layout.size()) };
        slice.copy_from_slice(bytecode);

        Ok(Self(pointer, layout.size()))
    }

    /// The length of this SPIR-V bytecode buffer **in bytes**.
    pub fn code_size(&self) -> usize {
        self.1
    }

    /// The value of this SPIR-V bytecode buffer.
    pub fn code(&self) -> &[u32] {
        let pointer: *const u32 = self.0.cast();
        let length = self.1 / 4;
        unsafe { slice::from_raw_parts(pointer, length) }
    }
}

impl Drop for Bytecode {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.1, 4).unwrap();
        unsafe { free(self.0, layout) };
    }
}
