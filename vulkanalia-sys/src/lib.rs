// SPDX-License-Identifier: Apache-2.0

//! Raw Vulkan bindings for Rust.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "no_std_error", feature(error_in_core))]

extern crate alloc;

#[macro_use]
mod bitfields;

mod arrays;

#[rustfmt::skip]
mod bitmasks;
#[rustfmt::skip]
mod commands;
#[rustfmt::skip]
mod constants;
#[rustfmt::skip]
mod enums;
#[rustfmt::skip]
mod extensions;
#[rustfmt::skip]
mod functions;
#[rustfmt::skip]
mod handles;
#[rustfmt::skip]
mod macros;
#[rustfmt::skip]
mod structs;
#[rustfmt::skip]
mod typedefs;
#[rustfmt::skip]
mod unions;

#[rustfmt::skip]
pub mod video;

pub use self::arrays::*;
pub use self::bitmasks::*;
pub use self::commands::*;
pub use self::constants::*;
pub use self::enums::*;
pub use self::extensions::*;
pub use self::functions::*;
pub use self::handles::*;
pub use self::macros::*;
pub use self::structs::*;
pub use self::typedefs::*;
pub use self::unions::*;
