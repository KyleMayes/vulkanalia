// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.Typedef

/** Generates Rust type aliases for Vulkan typedefs and platform types. */
fun Registry.generateTypedefs() =
    """
use std::os::raw::{c_ulong, c_void};

${basetypes.values.sortedBy { it.name }.joinToString("\n") { generateTypedef(it) }}

// Android

pub type ANativeWindow = c_void;
pub type AHardwareBuffer = c_void;

// DirectFB

pub type IDirectFB = c_void;
pub type IDirectFBSurface = c_void;

// iOS / macOS

pub type CAMetalLayer = c_void;
pub type GgpFrameToken = u32;
pub type GgpStreamDescriptor = u32;
pub type IOSurfaceRef = *mut c_void;
pub type MTLBuffer_id = *mut c_void;
pub type MTLCommandQueue_id = *mut c_void;
pub type MTLDevice_id = *mut c_void;
pub type MTLSharedEvent_id = *mut c_void;
pub type MTLTexture_id = *mut c_void;

// QNX

pub type _screen_buffer = c_void;
pub type _screen_context = c_void;
pub type _screen_window = c_void;

// Wayland

pub type wl_display = c_void;
pub type wl_surface = c_void;

// Windows

pub type DWORD = c_ulong;
pub type HANDLE = *mut c_void;
pub type HINSTANCE = *mut c_void;
pub type HMONITOR = *mut c_void;
pub type HWND = *mut c_void;
pub type LPCWSTR = *const u16;
pub type SECURITY_ATTRIBUTES = c_void;

// X11

pub type Display = *const c_void;
pub type RROutput = c_ulong;
pub type VisualID = c_ulong;
pub type Window = c_ulong;
pub type xcb_connection_t = c_void;
pub type xcb_visualid_t = u32;
pub type xcb_window_t = u32;
pub type zx_handle_t = u32;

// NvSciBuf / NvSciSync
    
pub type NvSciBufAttrList = *mut c_void;
pub type NvSciBufObj = *mut c_void;
pub type NvSciSyncAttrList = *mut c_void;
pub type NvSciSyncObj = *mut c_void;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct NvSciSyncFence {
    pub payload: [u64; 6],
}
    """

/** Generates a Rust type alias for a Vulkan typedef. */
fun Registry.generateTypedef(typedef: Typedef) =
    """
/// <${generateManualUrl(typedef)}>
pub type ${typedef.name} = ${typedef.type.generate()};
    """.trim()
