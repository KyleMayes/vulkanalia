// SPDX-License-Identifier: Apache-2.0

//! Window integration.

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::prelude::v1_0::*;

/// Gets the required instance extensions for window integration.
#[allow(deprecated)]
pub fn get_required_instance_extensions(
    window: &dyn HasRawWindowHandle,
) -> &'static [&'static vk::ExtensionName] {
    match window.raw_window_handle() {
        // BSD / Linux
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(window) => &[
            &vk::KHR_SURFACE_EXTENSION.name,
            &vk::KHR_WAYLAND_SURFACE_EXTENSION.name,
        ],
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xcb(window) => &[
            &vk::KHR_SURFACE_EXTENSION.name,
            &vk::KHR_XCB_SURFACE_EXTENSION.name,
        ],
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(window) => &[
            &vk::KHR_SURFACE_EXTENSION.name,
            &vk::KHR_XLIB_SURFACE_EXTENSION.name,
        ],
        // macOS
        #[cfg(target_os = "macos")]
        RawWindowHandle::MacOS(window) => &[
            &vk::KHR_SURFACE_EXTENSION.name,
            &vk::EXT_METAL_SURFACE_EXTENSION.name,
        ],
        // Windows
        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(_) => &[
            &vk::KHR_SURFACE_EXTENSION.name,
            &vk::KHR_WIN32_SURFACE_EXTENSION.name,
        ],
        // Unsupported (currently)
        _ => unimplemented!(),
    }
}

/// Creates a surface for a window.
///
/// # Safety
///
/// The returned `SurfaceKHR` will only be valid while the supplied window is
/// valid so the supplied window must not be destroyed before the the returned
/// `SurfaceKHR` is destroyed.
pub unsafe fn create_surface(
    instance: &Instance,
    window: &dyn HasRawWindowHandle,
) -> VkResult<vk::SurfaceKHR> {
    match window.raw_window_handle() {
        // BSD / Linux
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(window) => {
            use vk::KhrWaylandSurfaceExtension;
            let info = vk::WaylandSurfaceCreateInfoKHR::builder()
                .display(window.display)
                .surface(window.surface);
            instance.create_wayland_surface_khr(&info, None)
        }
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xcb(window) => {
            use vk::KhrXcbSurfaceExtension;
            let info = vk::XcbSurfaceCreateInfoKHR::builder()
                .connection(window.connection)
                .window(window.window as _);
            instance.create_xcb_surface_khr(&info, None)
        }
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(window) => {
            use vk::KhrXlibSurfaceExtension;
            let info = vk::XlibSurfaceCreateInfoKHR::builder()
                .dpy(unsafe { &mut (*(window.display as *mut _)) })
                .window(window.window);
            instance.create_xlib_surface_khr(&info, None)
        }
        // macOS
        #[cfg(target_os = "macos")]
        RawWindowHandle::MacOS(window) => {
            use std::mem;
            use std::os::raw::c_void;

            use cocoa::appkit::{NSView, NSWindow};
            use cocoa::base::id;
            use metal::{MetalLayer, MetalLayerRef};
            use objc::runtime::YES;
            use vk::ExtMetalSurfaceExtension;
            use vk::MvkMacosSurfaceExtension;

            let (view, layer) = unsafe {
                let id = mem::transmute::<_, id>(window.ns_window);

                let view = id.contentView();

                let layer = MetalLayer::new();
                layer.set_contents_scale(view.backingScaleFactor());
                layer.set_edge_antialiasing_mask(0);
                layer.set_presents_with_transaction(false);
                layer.remove_all_animations();

                view.setLayer(mem::transmute(layer.as_ref()));
                view.setWantsLayer(YES);

                (&mut *window.ns_view, layer)
            };

            let layer = (layer.as_ref() as *const MetalLayerRef).cast::<c_void>();
            let info = vk::MetalSurfaceCreateInfoEXT::builder().layer(layer);
            instance.create_metal_surface_ext(&info, None)
        }
        // Windows
        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(window) => {
            use vk::KhrWin32SurfaceExtension;
            let info = vk::Win32SurfaceCreateInfoKHR::builder()
                .hinstance(window.hinstance)
                .hwnd(window.hwnd);
            instance.create_win32_surface_khr(&info, None)
        }
        // Unsupported (currently)
        _ => unimplemented!(),
    }
}
