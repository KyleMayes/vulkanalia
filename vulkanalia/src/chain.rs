// SPDX-License-Identifier: Apache-2.0

//! Pointer chain handling.
//!
//! # Input Pointer Chains
//!
//! Input pointer chains are singly linked lists of Vulkan structs that
//! implement [`vk::InputChainStruct`] such as [`vk::InstanceCreateInfo`]. These
//! pointer chains are used to provide input data to Vulkan commands. A pointer
//! in such a chain can be represented with [`InputChainPtr`] and can be easily
//! iterated over using [`input_chain`].
//!
//! ### Example
//!
//! ```
//! # use vulkanalia::prelude::v1_0::*;
//! # use vulkanalia::chain::input_chain;
//! // Build an input pointer chain.
//!
//! let mut features = vk::ValidationFeaturesEXT::default();
//! let mut flags = vk::ValidationFlagsEXT::default();
//! let info = vk::InstanceCreateInfo::builder()
//!     .push_next(&mut features)
//!     .push_next(&mut flags)
//!     .build();
//!
//! // Iterate over the input pointer chain.
//!
//! let structs = unsafe { input_chain(info.next) }.collect::<Vec<_>>();
//! assert_eq!(structs.len(), 2);
//!
//! // Inspect the pointers in the input pointer chain.
//!
//! let base = unsafe { structs[0].as_base_ref() };
//! assert_eq!(base.s_type, vk::StructureType::VALIDATION_FLAGS_EXT);
//! let full = unsafe { structs[0].as_ref::<vk::ValidationFlagsEXT>() };
//! assert_eq!(full, &flags);
//!
//! let base = unsafe { structs[1].as_base_ref() };
//! assert_eq!(base.s_type, vk::StructureType::VALIDATION_FEATURES_EXT);
//! let full = unsafe { structs[1].as_ref::<vk::ValidationFeaturesEXT>() };
//! assert_eq!(full, &features);
//! ```
//!
//! # Output Pointer Chain
//!
//! Output pointer chains are singly linked lists of Vulkan structs that
//! implement [`vk::OutputChainStruct`] such as [`vk::PhysicalDeviceFeatures2`].
//! These pointer chains are used by Vulkan to provide output data to the
//! application. A pointer in such a chain can be represented with
//! [`OutputChainPtr`] and can be easily iterated over using [`output_chain`].
//!
//! ### Example
//!
//! ```no_run
//! # use vulkanalia::prelude::v1_0::*;
//! # use vulkanalia::chain::output_chain;
//! # use vk::KhrGetPhysicalDeviceProperties2Extension;
//! # let instance: Instance = panic!();
//! # let physical_device: vk::PhysicalDevice = panic!();
//! // Call a command that populates an output pointer chain.
//!
//! let mut features_11 = vk::PhysicalDeviceVulkan11Features::default();
//! let mut features_12 = vk::PhysicalDeviceVulkan12Features::default();
//! let mut features = vk::PhysicalDeviceFeatures2::builder()
//!     .push_next(&mut features_11)
//!     .push_next(&mut features_12);
//!
//! unsafe { instance.get_physical_device_features2_khr(physical_device, &mut features) };
//!
//! // Iterate over the pointers in the output pointer chain.
//!
//! let structs = unsafe { output_chain(features.next) }.collect::<Vec<_>>();
//! assert_eq!(structs.len(), 2);
//!
//! // Inspect the pointers in the output pointer chain.
//!
//! let base = unsafe { structs[0].as_base_ref() };
//! assert_eq!(base.s_type, vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES);
//! let full = unsafe { structs[0].as_ref::<vk::PhysicalDeviceVulkan12Features>() };
//! assert_eq!(full.descriptor_indexing, 1);       
//!
//! let base = unsafe { structs[1].as_base_ref() };
//! assert_eq!(base.s_type, vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES);
//! let full = unsafe { structs[1].as_ref::<vk::PhysicalDeviceVulkan11Features>() };
//! assert_eq!(full.protected_memory, 1);
//! ```

use core::ffi::c_void;
use core::iter;
use core::ptr::NonNull;

use crate::prelude::v1_0::*;

/// Creates an iterator over a Vulkan input pointer chain.
///
/// # Safety
///
/// See [`InputChainPtr::new`].
pub unsafe fn input_chain(head: *const c_void) -> impl Iterator<Item = InputChainPtr> {
    let mut next = InputChainPtr::new(head);
    iter::from_fn(move || {
        let current = next?;
        next = current.next();
        Some(current)
    })
}

/// Creates an iterator over a Vulkan output pointer chain.
///
/// # Safety
///
/// See [`OutputChainPtr::new`].
pub unsafe fn output_chain(head: *mut c_void) -> impl Iterator<Item = OutputChainPtr> {
    let mut next = OutputChainPtr::new(head);
    iter::from_fn(move || {
        let current = next?;
        next = current.next();
        Some(current)
    })
}

/// A non-null pointer in a Vulkan input pointer chain.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputChainPtr(NonNull<vk::BaseInStructure>);

impl InputChainPtr {
    /// Creates a non-null pointer in a Vulkan input pointer chain.
    ///
    /// # Safety
    ///
    /// `head` must either be null or be a pointer to a struct that could be
    /// part of an input pointer chain (i.e., [`vk::InputChainStruct`]). The
    /// same requirement recursively applies to any pointers in the chain.
    pub unsafe fn new(head: *const c_void) -> Option<Self> {
        NonNull::new(head.cast_mut()).map(|p| Self(p.cast()))
    }

    /// Gets the pointee of this pointer as a non-specific Vulkan struct.
    ///
    /// # Safety
    ///
    /// The pointer passed to [`Self::new`] must still satisfy the safety
    /// requirements of that method.
    pub unsafe fn as_base_ref(&self) -> &vk::BaseInStructure {
        self.0.as_ref()
    }

    /// Gets the pointee of this pointer as a specific Vulkan struct.
    ///
    /// # Safety
    ///
    /// The pointer passed to [`Self::new`] must still satisfy the safety
    /// requirements of that method **and** it must also be a pointer to a valid
    /// instance of the provided Vulkan struct type.
    pub unsafe fn as_ref<T: vk::InputChainStruct>(&self) -> &T {
        assert_eq!(self.as_base_ref().s_type, T::TYPE);
        self.0.cast::<T>().as_ref()
    }

    /// Gets the next non-null pointer in this Vulkan input pointer chain.
    ///
    /// # Safety
    ///
    /// The pointer passed to [`Self::new`] must still satisfy the safety
    /// requirements of that method.
    pub unsafe fn next(&self) -> Option<Self> {
        Self::new(self.as_base_ref().next.cast())
    }
}

/// A non-null pointer in a Vulkan output pointer chain.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutputChainPtr(NonNull<vk::BaseOutStructure>);

impl OutputChainPtr {
    /// Creates a non-null pointer in a Vulkan output pointer chain.
    ///
    /// # Safety
    ///
    /// `head` must either be null or be a pointer to a struct that could be
    /// part of an output pointer chain (i.e., [`vk::OutputChainStruct`]). The
    /// same requirement recursively applies to any pointers in the chain.
    pub unsafe fn new(head: *mut c_void) -> Option<Self> {
        NonNull::new(head).map(|p| Self(p.cast()))
    }

    /// Gets the pointee of this pointer as a non-specific Vulkan struct.
    ///
    /// # Safety
    ///
    /// The pointer passed to [`Self::new`] must still satisfy the safety
    /// requirements of that method.
    pub unsafe fn as_base_ref(&self) -> &vk::BaseOutStructure {
        self.0.as_ref()
    }

    /// Gets the pointee of this pointer as a specific Vulkan struct.
    ///
    /// # Safety
    ///
    /// The pointer passed to [`Self::new`] must still satisfy the safety
    /// requirements of that method **and** it must also be a pointer to a valid
    /// instance of the provided Vulkan struct type.
    pub unsafe fn as_ref<T: vk::OutputChainStruct>(&self) -> &T {
        assert_eq!(self.as_base_ref().s_type, T::TYPE);
        self.0.cast::<T>().as_ref()
    }

    /// Gets the next non-null pointer in this Vulkan output pointer chain.
    ///
    /// # Safety
    ///
    /// The pointer passed to [`Self::new`] must still satisfy the safety
    /// requirements of that method.
    pub unsafe fn next(&self) -> Option<Self> {
        Self::new(self.as_base_ref().next.cast())
    }
}
