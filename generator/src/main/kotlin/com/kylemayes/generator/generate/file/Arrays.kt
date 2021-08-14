package com.kylemayes.generator.generate.file

import com.kylemayes.generator.registry.Registry

/** Generates Rust types for Vulkan arrays. */
fun Registry.generateArrays() =
    """
use std::borrow::Cow;
use std::ffi::CStr;
use std::fmt;
use std::ops;
use std::os::raw::c_char;

/// An array containing a sequence of bytes.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ByteArray<const N: usize>(pub [u8; N]);

impl<const N: usize> Default for ByteArray<N> {
    #[inline]
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> ops::Deref for ByteArray<N> {
    type Target = [u8; N];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> fmt::Debug for ByteArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ByteArray<{}>({:?})", N, self.0)
    }
}

impl<const N: usize> fmt::Display for ByteArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<const N: usize> From<[u8; N]> for ByteArray<N> {
    #[inline]
    fn from(array: [u8; N]) -> Self {
        Self(array)
    }
}

impl<const N: usize> From<ByteArray<N>> for [u8; N] {
    #[inline]
    fn from(array: ByteArray<N>) -> Self {
        array.0
    }
}

/// An array containing a null-terminated string.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct StringArray<const N: usize>(pub [c_char; N]);

impl<const N: usize> StringArray<N> {
    /// Constructs a string array from a byte string.
    #[inline]
    pub const fn from_bytes(bytes: &[u8]) -> Self {
        let mut array = [0; N];

        let mut index = 0;
        while index < bytes.len() {
            array[index] = bytes[index] as c_char;
            index += 1;
        }

        if bytes.is_empty() || bytes[bytes.len() - 1] != 0 {
            array[bytes.len()] = 0;
        }

        Self(array)
    }

    /// Constructs a string array from a pointer to a C string.
    #[inline]
    pub unsafe fn from_ptr(ptr: *const c_char) -> Self {
        Self::from_bytes(CStr::from_ptr(ptr).to_bytes())
    }

    /// Converts this string array to a UTF-8 string.
    #[inline]
    pub fn to_string_lossy(&self) -> Cow<str> {
        let (_, bytes, _) = unsafe { self.0.align_to::<u8>() };
        let nul = bytes.iter().position(|b| *b == b'\0');
        let end = nul.unwrap_or(bytes.len());
        String::from_utf8_lossy(&bytes[..end])
    }
}

impl<const N: usize> Default for StringArray<N> {
    #[inline]
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> ops::Deref for StringArray<N> {
    type Target = [c_char; N];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> fmt::Debug for StringArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StringArray<{}>({:?})", N, self.to_string_lossy())
    }
}

impl<const N: usize> fmt::Display for StringArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string_lossy())
    }
}

impl<const N: usize> From<[c_char; N]> for StringArray<N> {
    #[inline]
    fn from(array: [c_char; N]) -> Self {
        Self(array)
    }
}

impl<const N: usize> From<StringArray<N>> for [c_char; N] {
    #[inline]
    fn from(array: StringArray<N>) -> Self {
        array.0
    }
}
    """
