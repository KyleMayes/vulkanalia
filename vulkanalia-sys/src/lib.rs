// SPDX-License-Identifier: Apache-2.0

//! Raw Vulkan bindings for Rust.

#[rustfmt::skip]
pub mod bitmasks;
#[rustfmt::skip]
pub mod commands;
#[rustfmt::skip]
pub mod constants;
#[rustfmt::skip]
pub mod enums;
#[rustfmt::skip]
pub mod extensions;
#[rustfmt::skip]
pub mod functions;
#[rustfmt::skip]
pub mod handles;
#[rustfmt::skip]
pub mod macros;
#[rustfmt::skip]
pub mod structs;
#[rustfmt::skip]
pub mod typedefs;
#[rustfmt::skip]
pub mod unions;

/// Raw Vulkan bindings for Rust.
pub mod vk {
    pub use super::bitmasks::*;
    pub use super::commands::*;
    pub use super::constants::*;
    pub use super::enums::*;
    pub use super::extensions::*;
    pub use super::functions::*;
    pub use super::handles::*;
    pub use super::macros::*;
    pub use super::structs::*;
    pub use super::typedefs::*;
    pub use super::unions::*;
}
