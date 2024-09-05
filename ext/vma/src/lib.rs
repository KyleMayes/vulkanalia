// SPDX-License-Identifier: Apache-2.0

//! An integration of [Vulkan Memory Allocator](https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator) with [`vulkanalia`](https://github.com/KyleMayes/vulkanalia).
//!
//! ### Example
//!
//! ```
//! use vulkanalia::prelude::v1_0::*;
//! use vulkanalia_vma::{self as vma, Alloc};
//!
//! fn example(instance: &Instance, device: &Device, physical_device: vk::PhysicalDevice) {
//!     // Create an allocator.
//!     let allocator_options = vma::AllocatorOptions::new(instance, device, physical_device);
//!     let allocator = unsafe { vma::Allocator::new(&allocator_options) }.unwrap();
//!
//!     // Allocate a buffer using the allocator.
//!     let buffer_create_info = vk::BufferCreateInfo::builder()
//!         .size(65_536)
//!         .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
//!         .sharing_mode(vk::SharingMode::CONCURRENT);
//!     let allocation_options = vma::AllocationOptions::default();
//!     let (buffer, allocation) = unsafe { allocator.create_buffer(buffer_create_info, &allocation_options) }.unwrap();
//!
//!     // Deallocate the buffer using the allocator.
//!     unsafe { allocator.destroy_buffer(buffer, allocation) };
//! }
//! ```

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
