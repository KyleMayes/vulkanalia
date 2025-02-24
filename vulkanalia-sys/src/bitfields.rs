// SPDX-License-Identifier: Apache-2.0

/// Defines a sequence of bitfields packed into a `u32`.
macro_rules! bitfields32 {
    (
        $(#[$outer:meta])*
        struct $name:ident {
            $(($get:ident, $with:ident)[$start:literal .. $end:literal]), *,
        }
    ) => (
        $(#[$outer])*
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        pub struct $name(pub u32);

        impl $name {
            $(
                #[inline]
                pub const fn $get(self) -> u32 {
                    let mask = (1 << ($end - $start)) - 1;
                    (self.0 & (mask << $start)) >> $start
                }

                #[inline]
                pub const fn $with(self, value: u32) -> Self {
                    let mask = (1 << ($end - $start)) - 1;
                    Self((self.0 & !(mask << $start)) | ((value & mask) << $start))
                }
            )*
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.debug_struct(stringify!($name))
                    $(.field(stringify!($get), &self.$get()))*
                    .finish()
            }
        }
    );
}

#[cfg(test)]
mod test {
    use core::u32;

    bitfields32! {
        /// A test sequence of bitfields.
        struct Bitfields {
            (foo, with_foo)[0 .. 4],
            (bar, with_bar)[15 .. 16],
            (baz, with_baz)[16 .. 17],
            (qux, with_qux)[28 .. 32],
        }
    }

    #[test]
    fn test_bitfields() {
        let bf = Bitfields(0b00111100_01010101_01010101_00111100);

        assert_eq!(bf.foo(), 0b1100);
        assert_eq!(bf.bar(), 0);
        assert_eq!(bf.baz(), 1);
        assert_eq!(bf.qux(), 0b0011);

        assert_eq!(bf.with_foo(0b0000).0, 0b00111100_01010101_01010101_00110000);
        assert_eq!(bf.with_foo(0b1111).0, 0b00111100_01010101_01010101_00111111);
        assert_eq!(
            bf.with_foo(u32::MAX).0,
            0b00111100_01010101_01010101_00111111
        );

        assert_eq!(bf.with_bar(0).0, 0b00111100_01010101_01010101_00111100);
        assert_eq!(bf.with_bar(1).0, 0b00111100_01010101_11010101_00111100);
        assert_eq!(
            bf.with_bar(u32::MAX).0,
            0b00111100_01010101_11010101_00111100
        );

        assert_eq!(bf.with_baz(0).0, 0b00111100_01010100_01010101_00111100);
        assert_eq!(bf.with_baz(1).0, 0b00111100_01010101_01010101_00111100);
        assert_eq!(
            bf.with_baz(u32::MAX).0,
            0b00111100_01010101_01010101_00111100
        );

        assert_eq!(bf.with_qux(0b0000).0, 0b00001100_01010101_01010101_00111100);
        assert_eq!(bf.with_qux(0b1111).0, 0b11111100_01010101_01010101_00111100);
        assert_eq!(
            bf.with_qux(u32::MAX).0,
            0b11111100_01010101_01010101_00111100
        );

        assert_eq!(
            format!("{bf:?}"),
            "Bitfields { foo: 12, bar: 0, baz: 1, qux: 3 }"
        );
    }
}
