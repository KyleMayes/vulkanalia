// SPDX-License-Identifier: Apache-2.0

// DO NOT EDIT.
//
// This file has been generated by the Kotlin project in the `generator`
// directory from a Vulkan API registry.

#![allow(
    non_camel_case_types,
    non_snake_case,
    clippy::let_unit_value,
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::unnecessary_cast,
    clippy::upper_case_acronyms,
    clippy::useless_transmute
)]

use std::error;
use std::fmt;

use super::Result;

/// Result codes that indicate successes.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SuccessCode(i32);

impl SuccessCode {
    pub const SUCCESS: Self = Self(0);
    pub const NOT_READY: Self = Self(1);
    pub const TIMEOUT: Self = Self(2);
    pub const EVENT_SET: Self = Self(3);
    pub const EVENT_RESET: Self = Self(4);
    pub const INCOMPLETE: Self = Self(5);
    pub const PIPELINE_COMPILE_REQUIRED: Self = Self(1000297000);
    pub const SUBOPTIMAL_KHR: Self = Self(1000001003);
    pub const THREAD_IDLE_KHR: Self = Self(1000268000);
    pub const THREAD_DONE_KHR: Self = Self(1000268001);
    pub const OPERATION_DEFERRED_KHR: Self = Self(1000268002);
    pub const OPERATION_NOT_DEFERRED_KHR: Self = Self(1000268003);
    pub const INCOMPATIBLE_SHADER_BINARY_EXT: Self = Self(1000482000);

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

impl fmt::Debug for SuccessCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            0 => write!(f, "SUCCESS"),
            1 => write!(f, "NOT_READY"),
            2 => write!(f, "TIMEOUT"),
            3 => write!(f, "EVENT_SET"),
            4 => write!(f, "EVENT_RESET"),
            5 => write!(f, "INCOMPLETE"),
            1000297000 => write!(f, "PIPELINE_COMPILE_REQUIRED"),
            1000001003 => write!(f, "SUBOPTIMAL_KHR"),
            1000268000 => write!(f, "THREAD_IDLE_KHR"),
            1000268001 => write!(f, "THREAD_DONE_KHR"),
            1000268002 => write!(f, "OPERATION_DEFERRED_KHR"),
            1000268003 => write!(f, "OPERATION_NOT_DEFERRED_KHR"),
            1000482000 => write!(f, "INCOMPATIBLE_SHADER_BINARY_EXT"),
            _ => self.0.fmt(f),
        }
    }
}

impl From<Result> for SuccessCode {
    #[inline]
    fn from(result: Result) -> Self {
        Self::from_raw(result.as_raw())
    }
}

impl From<SuccessCode> for Result {
    #[inline]
    fn from(code: SuccessCode) -> Self {
        Result::from_raw(code.as_raw())
    }
}

/// Result codes that indicate errors.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ErrorCode(i32);

impl ErrorCode {
    pub const OUT_OF_HOST_MEMORY: Self = Self(-1);
    pub const OUT_OF_DEVICE_MEMORY: Self = Self(-2);
    pub const INITIALIZATION_FAILED: Self = Self(-3);
    pub const DEVICE_LOST: Self = Self(-4);
    pub const MEMORY_MAP_FAILED: Self = Self(-5);
    pub const LAYER_NOT_PRESENT: Self = Self(-6);
    pub const EXTENSION_NOT_PRESENT: Self = Self(-7);
    pub const FEATURE_NOT_PRESENT: Self = Self(-8);
    pub const INCOMPATIBLE_DRIVER: Self = Self(-9);
    pub const TOO_MANY_OBJECTS: Self = Self(-10);
    pub const FORMAT_NOT_SUPPORTED: Self = Self(-11);
    pub const FRAGMENTED_POOL: Self = Self(-12);
    pub const UNKNOWN: Self = Self(-13);
    pub const OUT_OF_POOL_MEMORY: Self = Self(-1000069000);
    pub const INVALID_EXTERNAL_HANDLE: Self = Self(-1000072003);
    pub const FRAGMENTATION: Self = Self(-1000161000);
    pub const INVALID_OPAQUE_CAPTURE_ADDRESS: Self = Self(-1000257000);
    pub const SURFACE_LOST_KHR: Self = Self(-1000000000);
    pub const NATIVE_WINDOW_IN_USE_KHR: Self = Self(-1000000001);
    pub const OUT_OF_DATE_KHR: Self = Self(-1000001004);
    pub const INCOMPATIBLE_DISPLAY_KHR: Self = Self(-1000003001);
    pub const VALIDATION_FAILED_EXT: Self = Self(-1000011001);
    pub const INVALID_SHADER_NV: Self = Self(-1000012000);
    pub const INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: Self = Self(-1000158000);
    pub const NOT_PERMITTED_KHR: Self = Self(-1000174001);
    pub const FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: Self = Self(-1000255000);
    pub const COMPRESSION_EXHAUSTED_EXT: Self = Self(-1000338000);

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

impl fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            -1 => write!(f, "OUT_OF_HOST_MEMORY"),
            -2 => write!(f, "OUT_OF_DEVICE_MEMORY"),
            -3 => write!(f, "INITIALIZATION_FAILED"),
            -4 => write!(f, "DEVICE_LOST"),
            -5 => write!(f, "MEMORY_MAP_FAILED"),
            -6 => write!(f, "LAYER_NOT_PRESENT"),
            -7 => write!(f, "EXTENSION_NOT_PRESENT"),
            -8 => write!(f, "FEATURE_NOT_PRESENT"),
            -9 => write!(f, "INCOMPATIBLE_DRIVER"),
            -10 => write!(f, "TOO_MANY_OBJECTS"),
            -11 => write!(f, "FORMAT_NOT_SUPPORTED"),
            -12 => write!(f, "FRAGMENTED_POOL"),
            -13 => write!(f, "UNKNOWN"),
            -1000069000 => write!(f, "OUT_OF_POOL_MEMORY"),
            -1000072003 => write!(f, "INVALID_EXTERNAL_HANDLE"),
            -1000161000 => write!(f, "FRAGMENTATION"),
            -1000257000 => write!(f, "INVALID_OPAQUE_CAPTURE_ADDRESS"),
            -1000000000 => write!(f, "SURFACE_LOST_KHR"),
            -1000000001 => write!(f, "NATIVE_WINDOW_IN_USE_KHR"),
            -1000001004 => write!(f, "OUT_OF_DATE_KHR"),
            -1000003001 => write!(f, "INCOMPATIBLE_DISPLAY_KHR"),
            -1000011001 => write!(f, "VALIDATION_FAILED_EXT"),
            -1000012000 => write!(f, "INVALID_SHADER_NV"),
            -1000158000 => write!(f, "INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT"),
            -1000174001 => write!(f, "NOT_PERMITTED_KHR"),
            -1000255000 => write!(f, "FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT"),
            -1000338000 => write!(f, "COMPRESSION_EXHAUSTED_EXT"),
            _ => self.0.fmt(f),
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            -1 => write!(f, "A host memory allocation has failed."),
            -2 => write!(f, "A device memory allocation has failed."),
            -3 => write!(f, "Initialization of an object could not be completed for implementation-specific reasons."),
            -4 => write!(f, "The logical or physical device has been lost. See Lost Device."),
            -5 => write!(f, "Mapping of a memory object has failed."),
            -6 => write!(f, "A requested layer is not present or could not be loaded."),
            -7 => write!(f, "A requested extension is not supported."),
            -8 => write!(f, "A requested feature is not supported."),
            -9 => write!(f, "The requested version of Vulkan is not supported by the driver or is otherwise incompatible for implementation-specific reasons."),
            -10 => write!(f, "Too many objects of the type have already been created."),
            -11 => write!(f, "A requested format is not supported on this device."),
            -12 => write!(f, "A pool allocation has failed due to fragmentation of the pool's memory. This must only be returned if no attempt to allocate host or device memory was made to accommodate the new allocation. This should be returned in preference to VK_ERROR_OUT_OF_POOL_MEMORY, but only if the implementation is certain that the pool allocation failure was due to fragmentation."),
            -13 => write!(f, "An unknown error has occurred; either the application has provided invalid input, or an implementation failure has occurred."),
            -1000069000 => write!(f, "A pool memory allocation has failed. This must only be returned if no attempt to allocate host or device memory was made to accommodate the new allocation. If the failure was definitely due to fragmentation of the pool, VK_ERROR_FRAGMENTED_POOL should be returned instead."),
            -1000072003 => write!(f, "An external handle is not a valid handle of the specified type."),
            -1000161000 => write!(f, "A descriptor pool creation has failed due to fragmentation."),
            -1000257000 => write!(f, "A buffer creation or memory allocation failed because the requested address is not available. A shader group handle assignment failed because the requested shader group handle information is no longer valid."),
            -1000000000 => write!(f, "A surface is no longer available."),
            -1000000001 => write!(f, "The requested window is already in use by Vulkan or another API in a manner which prevents it from being used again."),
            -1000001004 => write!(f, "A surface has changed in such a way that it is no longer compatible with the swapchain, and further presentation requests using the swapchain will fail. Applications must query the new surface properties and recreate their swapchain if they wish to continue presenting to the surface."),
            -1000003001 => write!(f, "The display used by a swapchain does not use the same presentable image layout, or is incompatible in a way that prevents sharing an image."),
            -1000011001 => write!(f, "VALIDATION_FAILED_EXT"),
            -1000012000 => write!(f, "One or more shaders failed to compile or link. More details are reported back to the application via VK_EXT_debug_report if enabled."),
            -1000158000 => write!(f, "INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT"),
            -1000174001 => write!(f, "NOT_PERMITTED_KHR"),
            -1000255000 => write!(f, "An operation on a swapchain created with VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT failed as it did not have exclusive full-screen access. This may occur due to implementation-dependent reasons, outside of the application's control."),
            -1000338000 => write!(f, "COMPRESSION_EXHAUSTED_EXT"),
            _ => write!(f, "unknown Vulkan result (code = {})", self.0),
        }
    }
}

impl error::Error for ErrorCode {}

impl From<Result> for ErrorCode {
    #[inline]
    fn from(result: Result) -> Self {
        Self::from_raw(result.as_raw())
    }
}

impl From<ErrorCode> for Result {
    #[inline]
    fn from(code: ErrorCode) -> Self {
        Result::from_raw(code.as_raw())
    }
}
