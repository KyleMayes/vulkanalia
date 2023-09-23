// SPDX-License-Identifier: Apache-2.0

use core::fmt;

/// A pair of bitfields stored in an `u32`.
///
/// * `low` - 24 least significant bits
/// * `high` - 8 most significant bits
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Bitfield24_8(u32);

impl Bitfield24_8 {
    /// Stores a pair of bitfields in an `u32`.
    ///
    /// **NOTE:** `low` will be truncated to fit in a 24-bit unsigned integer.
    #[inline]
    pub const fn new(low: u32, high: u8) -> Self {
        Self((low & 0x00FF_FFFF) | ((high as u32) << 24))
    }

    /// The value of the 24 least significant bits in the `u32`.
    #[inline]
    pub const fn low(self) -> u32 {
        self.0 & 0x00FF_FFFF
    }

    /// The value of the 8 most significant bits in the `u32`.
    #[inline]
    pub const fn high(self) -> u8 {
        (self.0 >> 24) as u8
    }
}

impl fmt::Debug for Bitfield24_8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bitfield24_8(low={}, high={})", self.low(), self.high())
    }
}
