// SPDX-License-Identifier: Apache-2.0

#![allow(
    non_camel_case_types,
    non_snake_case,
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::upper_case_acronyms
)]

use alloc::borrow::Cow;
use alloc::string::String;
use core::ffi::{c_char, CStr};
use core::fmt;
use core::hash;
use core::ops;

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
///
/// # Equality / Hashing
///
/// For the purposes of comparing and hashing array strings, any characters
/// after the first null terminator are ignored. The below example demonstrates
/// this property with two strings that differ in the characters that come
/// after the first null terminators.
///
/// ```
/// # use std::collections::hash_map::DefaultHasher;
/// # use std::hash::{Hash, Hasher};
/// # use vulkanalia_sys::StringArray;
/// let string1 = StringArray::<3>::new([0x61, 0, 0]);
/// let string2 = StringArray::<3>::new([0x61, 0, 0x61]);
///
/// assert_eq!(string1, string2);
///
/// let mut hasher1 = DefaultHasher::new();
/// string1.hash(&mut hasher1);
/// let mut hasher2 = DefaultHasher::new();
/// string2.hash(&mut hasher2);
///
/// assert_eq!(hasher1.finish(), hasher2.finish());
/// ```
#[derive(Copy, Clone, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StringArray<const N: usize>([c_char; N]);

impl<const N: usize> StringArray<N> {
    /// Constructs a string array from a character array.
    ///
    /// # Panics
    ///
    /// * `characters` does not contain a null-terminator
    #[inline]
    pub fn new(array: [c_char; N]) -> Self {
        assert!(array.contains(&0));
        Self(array)
    }

    /// Constructs a string array from a byte string.
    ///
    /// If the byte string is longer than `N - 1`, then the byte string will
    /// be truncated to fit inside of the constructed string array (the last
    /// character is reserved for a null terminator). The constructed string
    /// array will always be null-terminated regardless if the byte string is
    /// or is not null-terminated.
    #[inline]
    pub const fn from_bytes(bytes: &[u8]) -> Self {
        let mut array = [0; N];

        let mut index = 0;
        while index < bytes.len() && index + 1 < N {
            if bytes[index] != 0 {
                array[index] = bytes[index] as c_char;
                index += 1;
            } else {
                break;
            }
        }

        Self(array)
    }

    /// Constructs a string array from a borrowed C string.
    ///
    /// If the borrowed C string is longer than `N - 1`, then the borrowed C
    /// string will be truncated to fit inside of the constructed string array
    /// (the last character is reserved for a null terminator).
    #[inline]
    pub fn from_cstr(cstr: &CStr) -> Self {
        Self::from_bytes(cstr.to_bytes())
    }

    /// Constructs a string array from a pointer to a null-terminated string.
    ///
    /// If the null-terminated string is longer than `N - 1`, then the
    /// null-terminated string will be truncated to fit inside of the
    /// constructed string array (the last character is reserved for a null
    /// terminator).
    ///
    /// # Safety
    ///
    /// * `ptr` must be a pointer to a null-terminated string
    #[inline]
    pub unsafe fn from_ptr(ptr: *const c_char) -> Self {
        Self::from_cstr(CStr::from_ptr(ptr))
    }

    /// Gets the underlying character array for this string array.
    #[inline]
    pub fn as_array(&self) -> &[c_char; N] {
        &self.0
    }

    /// Gets this string array as a slice of bytes.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { self.as_array().align_to::<u8>().1 }
    }

    /// Gets this string array as a borrowed C string.
    #[inline]
    pub fn as_cstr(&self) -> &CStr {
        let bytes = self.as_bytes();
        let nul = bytes.iter().position(|b| *b == b'\0');
        let end = nul.unwrap_or(N - 1);
        unsafe { CStr::from_bytes_with_nul_unchecked(&bytes[..end + 1]) }
    }

    /// Converts this string array to a UTF-8 string (lossily).
    #[inline]
    pub fn to_string_lossy(&self) -> Cow<str> {
        let bytes = self.as_bytes();
        let nul = bytes.iter().position(|b| *b == b'\0');
        let end = nul.unwrap_or(N);
        String::from_utf8_lossy(&bytes[..end])
    }
}

impl<const N: usize> Default for StringArray<N> {
    #[inline]
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> PartialEq for StringArray<N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_cstr() == other.as_cstr()
    }
}

impl<const N: usize> Eq for StringArray<N> {}

impl<const N: usize> hash::Hash for StringArray<N> {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.as_cstr().hash(hasher);
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

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn hash(hash: impl Hash) -> u64 {
        let mut hasher = DefaultHasher::new();
        hash.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_string_array_from_bytes() {
        type S1 = StringArray<1>;

        assert_eq!(b"\0", S1::from_bytes(b"").as_bytes());
        assert_eq!(b"\0", S1::from_bytes(b"\0").as_bytes());
        assert_eq!(b"\0", S1::from_bytes(b"\0bar").as_bytes());

        assert_eq!(b"\0", S1::from_bytes(b"322").as_bytes());
        assert_eq!(b"\0", S1::from_bytes(b"322\0").as_bytes());
        assert_eq!(b"\0", S1::from_bytes(b"322\0bar").as_bytes());

        type S4 = StringArray<4>;

        assert_eq!(b"\0\0\0\0", S4::from_bytes(b"").as_bytes());
        assert_eq!(b"\0\0\0\0", S4::from_bytes(b"\0").as_bytes());
        assert_eq!(b"\0\0\0\0", S4::from_bytes(b"\0bar").as_bytes());

        assert_eq!(b"322\0", S4::from_bytes(b"322").as_bytes());
        assert_eq!(b"322\0", S4::from_bytes(b"322\0").as_bytes());
        assert_eq!(b"322\0", S4::from_bytes(b"322\0bar").as_bytes());

        assert_eq!(b"128\0", S4::from_bytes(b"1288").as_bytes());
        assert_eq!(b"128\0", S4::from_bytes(b"1288\0").as_bytes());
        assert_eq!(b"128\0", S4::from_bytes(b"1288\0bar").as_bytes());
    }

    #[test]
    fn test_string_array_cmp() {
        macro_rules! assert_cmp_eq {
            ($left:expr, $right:expr) => {
                assert_eq!($left, $right);
                assert_eq!(hash($left), hash($right));
            };
        }

        macro_rules! assert_cmp_ne {
            ($left:expr, $right:expr) => {
                assert_ne!($left, $right);
                assert_ne!(hash($left), hash($right));
            };
        }

        type S32 = StringArray<32>;

        assert_cmp_eq!(S32::from_bytes(b""), S32::from_bytes(b""));
        assert_cmp_eq!(S32::from_bytes(b"\0"), S32::from_bytes(b""));
        assert_cmp_eq!(S32::from_bytes(b""), S32::from_bytes(b"\0"));
        assert_cmp_eq!(S32::from_bytes(b"\0"), S32::from_bytes(b"\0"));
        assert_cmp_eq!(S32::from_bytes(b"\0foo"), S32::from_bytes(b"\0bar"));

        assert_cmp_eq!(S32::from_bytes(b"322"), S32::from_bytes(b"322"));
        assert_cmp_eq!(S32::from_bytes(b"322\0"), S32::from_bytes(b"322"));
        assert_cmp_eq!(S32::from_bytes(b"322"), S32::from_bytes(b"322\0"));
        assert_cmp_eq!(S32::from_bytes(b"322\0"), S32::from_bytes(b"322\0"));
        assert_cmp_eq!(S32::from_bytes(b"322\0foo"), S32::from_bytes(b"322\0bar"));

        assert_cmp_ne!(S32::from_bytes(b"322"), S32::from_bytes(b"422"));
        assert_cmp_ne!(S32::from_bytes(b"322"), S32::from_bytes(b"332"));
        assert_cmp_ne!(S32::from_bytes(b"322"), S32::from_bytes(b"323"));

        assert_cmp_ne!(S32::from_bytes(b"322"), S32::from_bytes(b"32"));
        assert_cmp_ne!(S32::from_bytes(b"322"), S32::from_bytes(b"3222"));
    }
}
