// SPDX-License-Identifier: Apache-2.0

//! Vulkan layer interface types.
//!
//! <https://github.com/KhronosGroup/Vulkan-Headers/blob/main/include/vulkan/vk_layer.h>

#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_void};
use core::fmt;

use bitflags::bitflags;

use crate as vk;

//================================================
// Bitmasks
//================================================

bitflags! {
    #[repr(transparent)]
    #[derive(Default)]
    pub struct LoaderFeatureFlags: vk::Flags {
        const PHYSICAL_DEVICE_SORTING = 1;
    }
}

//================================================
// Enums
//================================================

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayerFunction(i32);

impl LayerFunction {
    pub const LAYER_LINK_INFO: Self = Self(0);
    pub const LOADER_DATA_CALLBACK: Self = Self(1);
    pub const LOADER_LAYER_CREATE_DEVICE_CALLBACK: Self = Self(2);
    pub const LOADER_FEATURES: Self = Self(3);

    /// Constructs an instance of this enum with the supplied underlying value.
    #[inline]
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }

    /// Gets the underlying value for this enum instance.
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl fmt::Debug for LayerFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            0 => write!(f, "LAYER_LINK_INFO"),
            1 => write!(f, "LOADER_DATA_CALLBACK"),
            2 => write!(f, "LOADER_LAYER_CREATE_DEVICE_CALLBACK"),
            3 => write!(f, "LOADER_FEATURES"),
            _ => self.0.fmt(f),
        }
    }
}

//================================================
// Functions
//================================================

pub type PFN_vkGetPhysicalDeviceProcAddr = unsafe extern "system" fn(
    _instance: vk::Instance,
    _name: *const c_char,
) -> vk::PFN_vkVoidFunction;

pub type PFN_vkLayerCreateDevice = unsafe extern "system" fn(
    _instance: vk::Instance,
    _physical_device: vk::PhysicalDevice,
    _create_info: *const vk::DeviceCreateInfo,
    _allocator: *const vk::AllocationCallbacks,
    _device: *mut vk::Device,
    _layer_gipa: vk::PFN_vkGetInstanceProcAddr,
    _next_gdpa: *mut vk::PFN_vkGetInstanceProcAddr,
) -> vk::Result;

pub type PFN_vkLayerDestroyDevice = unsafe extern "system" fn(
    _device: vk::Device,
    _allocator: *const vk::AllocationCallbacks,
    _destroy_function: vk::PFN_vkDestroyDevice,
) -> vk::Result;

pub type PFN_vkSetDeviceLoaderData =
    unsafe extern "system" fn(_device: vk::Device, _object: *mut c_void) -> vk::Result;

pub type PFN_vkSetInstanceLoaderData =
    unsafe extern "system" fn(_instance: vk::Instance, _object: *mut c_void) -> vk::Result;

//================================================
// Layer Instance Create Info
//================================================

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LayerInstanceLink {
    pub next: *mut Self,
    pub next_get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
    pub next_get_physical_device_proc_addr: PFN_vkGetPhysicalDeviceProcAddr,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LayerDevice {
    pub layer_create_device: PFN_vkLayerCreateDevice,
    pub layer_destroy_device: PFN_vkLayerDestroyDevice,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union LayerInstanceCreateInfoPayload {
    pub layer_info: *mut LayerInstanceLink,
    pub set_instance_loader_data: PFN_vkSetInstanceLoaderData,
    pub layer_device: LayerDevice,
    pub loader_features: LoaderFeatureFlags,
}

impl fmt::Debug for LayerInstanceCreateInfoPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LayerInstanceCreateInfoPayload")
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LayerInstanceCreateInfo {
    pub s_type: vk::StructureType,
    pub next: *const c_void,
    pub function: LayerFunction,
    pub payload: LayerInstanceCreateInfoPayload,
}

//================================================
// Layer Device Create Info
//================================================

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LayerDeviceLink {
    pub next: *mut Self,
    pub next_get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
    pub next_get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union LayerDeviceCreateInfoPayload {
    pub layer_info: *mut LayerDeviceLink,
    pub set_device_loader_data: PFN_vkSetDeviceLoaderData,
}

impl fmt::Debug for LayerDeviceCreateInfoPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LayerDeviceCreateInfoPayload")
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LayerDeviceCreateInfo {
    pub s_type: vk::StructureType,
    pub next: *const c_void,
    pub function: LayerFunction,
    pub payload: LayerDeviceCreateInfoPayload,
}
