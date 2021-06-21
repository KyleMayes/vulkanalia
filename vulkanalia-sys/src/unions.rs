// SPDX-License-Identifier: Apache-2.0

// DO NOT EDIT.
//
// This file has been generated by the Kotlin project in the `generator`
// directory from a Vulkan API registry.

#![allow(
    non_camel_case_types,
    non_snake_case,
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::upper_case_acronyms
)]

use std::fmt;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_void};

use crate::*;

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryDataKHR.html>
#[repr(C)]
#[derive(Copy, Clone)]
pub union AccelerationStructureGeometryDataKHR {
    pub triangles: AccelerationStructureGeometryTrianglesDataKHR,
    pub aabbs: AccelerationStructureGeometryAabbsDataKHR,
    pub instances: AccelerationStructureGeometryInstancesDataKHR,
}

impl Default for AccelerationStructureGeometryDataKHR {
    #[inline]
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl fmt::Debug for AccelerationStructureGeometryDataKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AccelerationStructureGeometryDataKHR")
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMotionInstanceDataNV.html>
#[repr(C)]
#[derive(Copy, Clone)]
pub union AccelerationStructureMotionInstanceDataNV {
    pub static_instance: AccelerationStructureInstanceKHR,
    pub matrix_motion_instance: AccelerationStructureMatrixMotionInstanceNV,
    pub srt_motion_instance: AccelerationStructureSRTMotionInstanceNV,
}

impl Default for AccelerationStructureMotionInstanceDataNV {
    #[inline]
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl fmt::Debug for AccelerationStructureMotionInstanceDataNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AccelerationStructureMotionInstanceDataNV")
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearColorValue.html>
#[repr(C)]
#[derive(Copy, Clone)]
pub union ClearColorValue {
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}

impl Default for ClearColorValue {
    #[inline]
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl fmt::Debug for ClearColorValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClearColorValue")
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearValue.html>
#[repr(C)]
#[derive(Copy, Clone)]
pub union ClearValue {
    pub color: ClearColorValue,
    pub depth_stencil: ClearDepthStencilValue,
}

impl Default for ClearValue {
    #[inline]
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl fmt::Debug for ClearValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClearValue")
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceOrHostAddressConstKHR.html>
#[repr(C)]
#[derive(Copy, Clone)]
pub union DeviceOrHostAddressConstKHR {
    pub device_address: DeviceAddress,
    pub host_address: *const c_void,
}

impl Default for DeviceOrHostAddressConstKHR {
    #[inline]
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl fmt::Debug for DeviceOrHostAddressConstKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceOrHostAddressConstKHR")
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceOrHostAddressKHR.html>
#[repr(C)]
#[derive(Copy, Clone)]
pub union DeviceOrHostAddressKHR {
    pub device_address: DeviceAddress,
    pub host_address: *mut c_void,
}

impl Default for DeviceOrHostAddressKHR {
    #[inline]
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl fmt::Debug for DeviceOrHostAddressKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceOrHostAddressKHR")
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterResultKHR.html>
#[repr(C)]
#[derive(Copy, Clone)]
pub union PerformanceCounterResultKHR {
    pub int32: i32,
    pub int64: i64,
    pub uint32: u32,
    pub uint64: u64,
    pub float32: f32,
    pub float64: f64,
}

impl Default for PerformanceCounterResultKHR {
    #[inline]
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl fmt::Debug for PerformanceCounterResultKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PerformanceCounterResultKHR")
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceValueDataINTEL.html>
#[repr(C)]
#[derive(Copy, Clone)]
pub union PerformanceValueDataINTEL {
    pub value32: u32,
    pub value64: u64,
    pub value_float: f32,
    pub value_bool: Bool32,
    pub value_string: *const c_char,
}

impl Default for PerformanceValueDataINTEL {
    #[inline]
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl fmt::Debug for PerformanceValueDataINTEL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PerformanceValueDataINTEL")
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableStatisticValueKHR.html>
#[repr(C)]
#[derive(Copy, Clone)]
pub union PipelineExecutableStatisticValueKHR {
    pub b32: Bool32,
    pub i64: i64,
    pub u64: u64,
    pub f64: f64,
}

impl Default for PipelineExecutableStatisticValueKHR {
    #[inline]
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl fmt::Debug for PipelineExecutableStatisticValueKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PipelineExecutableStatisticValueKHR")
    }
}
