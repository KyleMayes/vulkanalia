// SPDX-License-Identifier: Apache-2.0

//! An integration of [Vulkan Memory Allocator](https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator) with [`vulkanalia`](https://github.com/KyleMayes/vulkanalia).

#![no_std]
#![allow(clippy::missing_safety_doc)]

extern crate alloc;

mod allocation;
mod allocator;
mod enums;
mod flags;
mod pool;
mod r#virtual;

/// Raw bindings to VMA.
pub mod vma;

pub use self::allocation::*;
pub use self::allocator::*;
pub use self::enums::*;
pub use self::flags::*;
pub use self::pool::*;
pub use self::r#virtual::*;
