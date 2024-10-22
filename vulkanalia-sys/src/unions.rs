// SPDX-License-Identifier: Apache-2.0

// DO NOT EDIT.
//
// This file has been generated by the Kotlin project in the `generator`
// directory from a Vulkan API registry.

#![allow(
    non_camel_case_types,
    non_snake_case,
    clippy::bad_bit_mask,
    clippy::let_unit_value,
    clippy::missing_safety_doc,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::unnecessary_cast,
    clippy::upper_case_acronyms,
    clippy::useless_transmute
)]

use core::ffi::{c_char, c_void};
use core::fmt;
use core::mem::MaybeUninit;

use crate::*;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryDataKHR.html>
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

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceDataNV.html>
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

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearColorValue.html>
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

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearValue.html>
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

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorDataEXT.html>
#[repr(C)]
#[derive(Copy, Clone)]
pub union DescriptorDataEXT {
    pub sampler: *const Sampler,
    pub combined_image_sampler: *const DescriptorImageInfo,
    pub input_attachment_image: *const DescriptorImageInfo,
    pub sampled_image: *const DescriptorImageInfo,
    pub storage_image: *const DescriptorImageInfo,
    pub uniform_texel_buffer: *const DescriptorAddressInfoEXT,
    pub storage_texel_buffer: *const DescriptorAddressInfoEXT,
    pub uniform_buffer: *const DescriptorAddressInfoEXT,
    pub storage_buffer: *const DescriptorAddressInfoEXT,
    pub acceleration_structure: DeviceAddress,
}

impl Default for DescriptorDataEXT {
    #[inline]
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl fmt::Debug for DescriptorDataEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DescriptorDataEXT")
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceOrHostAddressConstAMDX.html>
#[repr(C)]
#[derive(Copy, Clone)]
pub union DeviceOrHostAddressConstAMDX {
    pub device_address: DeviceAddress,
    pub host_address: *const c_void,
}

impl Default for DeviceOrHostAddressConstAMDX {
    #[inline]
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl fmt::Debug for DeviceOrHostAddressConstAMDX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceOrHostAddressConstAMDX")
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceOrHostAddressConstKHR.html>
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

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceOrHostAddressKHR.html>
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

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsTokenDataEXT.html>
#[repr(C)]
#[derive(Copy, Clone)]
pub union IndirectCommandsTokenDataEXT {
    pub push_constant: *const IndirectCommandsPushConstantTokenEXT,
    pub vertex_buffer: *const IndirectCommandsVertexBufferTokenEXT,
    pub index_buffer: *const IndirectCommandsIndexBufferTokenEXT,
    pub execution_set: *const IndirectCommandsExecutionSetTokenEXT,
}

impl Default for IndirectCommandsTokenDataEXT {
    #[inline]
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl fmt::Debug for IndirectCommandsTokenDataEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IndirectCommandsTokenDataEXT")
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectExecutionSetInfoEXT.html>
#[repr(C)]
#[derive(Copy, Clone)]
pub union IndirectExecutionSetInfoEXT {
    pub pipeline_info: *const IndirectExecutionSetPipelineInfoEXT,
    pub shader_info: *const IndirectExecutionSetShaderInfoEXT,
}

impl Default for IndirectExecutionSetInfoEXT {
    #[inline]
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl fmt::Debug for IndirectExecutionSetInfoEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IndirectExecutionSetInfoEXT")
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterResultKHR.html>
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

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueDataINTEL.html>
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

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticValueKHR.html>
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
