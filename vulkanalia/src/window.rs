// SPDX-License-Identifier: Apache-2.0

//! Window integration.

use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};

use crate::prelude::v1_0::*;

/// Gets the required instance extensions for window integration.
pub fn get_required_instance_extensions(
    window: &dyn HasWindowHandle,
) -> &'static [&'static vk::ExtensionName] {
    match window.window_handle().map(|handle| handle.as_raw()) {
        // BSD / Linux
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        Ok(RawWindowHandle::Wayland(_window)) => &[
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
        Ok(RawWindowHandle::Xcb(_window)) => &[
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
        Ok(RawWindowHandle::Xlib(_window)) => &[
            &vk::KHR_SURFACE_EXTENSION.name,
            &vk::KHR_XLIB_SURFACE_EXTENSION.name,
        ],
        // macOS
        #[cfg(target_os = "macos")]
        Ok(RawWindowHandle::AppKit(_window)) => &[
            &vk::KHR_SURFACE_EXTENSION.name,
            &vk::EXT_METAL_SURFACE_EXTENSION.name,
        ],
        // Windows
        #[cfg(target_os = "windows")]
        Ok(RawWindowHandle::Win32(_window)) => &[
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
/// valid so the supplied window must not be destroyed before the returned
/// `SurfaceKHR` is destroyed.
#[allow(deprecated, unused_variables)]
pub unsafe fn create_surface(
    instance: &Instance,
    display: &dyn HasDisplayHandle,
    window: &dyn HasWindowHandle,
) -> VkResult<vk::SurfaceKHR> {
    match (
        display.display_handle().map(|handle| handle.as_raw()),
        window.window_handle().map(|handle| handle.as_raw()),
    ) {
        // BSD / Linux
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        (Ok(RawDisplayHandle::Wayland(display)), Ok(RawWindowHandle::Wayland(window))) => {
            use vk::KhrWaylandSurfaceExtension;
            let info = vk::WaylandSurfaceCreateInfoKHR::builder()
                .display(display.display.as_ptr())
                .surface(window.surface.as_ptr());
            instance.create_wayland_surface_khr(&info, None)
        }
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        (Ok(RawDisplayHandle::Xcb(display)), Ok(RawWindowHandle::Xcb(window))) => {
            use vk::KhrXcbSurfaceExtension;

            let connection_ptr = display
                .connection
                .map(|connection| connection.as_ptr())
                .unwrap_or(std::ptr::null_mut());

            let info = vk::XcbSurfaceCreateInfoKHR::builder()
                .connection(connection_ptr)
                .window(window.window.get() as _);
            instance.create_xcb_surface_khr(&info, None)
        }
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        (Ok(RawDisplayHandle::Xlib(display)), Ok(RawWindowHandle::Xlib(window))) => {
            use vk::KhrXlibSurfaceExtension;

            let display_ptr = display
                .display
                .map(|display| display.as_ptr())
                .unwrap_or(std::ptr::null_mut());

            let info = vk::XlibSurfaceCreateInfoKHR::builder()
                .dpy(&mut *(display_ptr as *mut _))
                .window(window.window);

            instance.create_xlib_surface_khr(&info, None)
        }
        // macOS
        #[cfg(target_os = "macos")]
        (Ok(RawDisplayHandle::AppKit(_)), Ok(RawWindowHandle::AppKit(window))) => {
            use std::os::raw::c_void;

            use cocoa::appkit::{NSView, NSWindow};
            use cocoa::base::id;
            use metal::{MetalLayer, MetalLayerRef};
            use objc::runtime::YES;
            use vk::ExtMetalSurfaceExtension;

            let layer = {
                let view = window.ns_view.as_ptr() as id;

                let layer = MetalLayer::new();
                layer.set_contents_scale(view.backingScaleFactor());
                layer.set_edge_antialiasing_mask(0);
                layer.set_presents_with_transaction(false);
                layer.remove_all_animations();

                let layer_ref = layer.as_ref() as *const metal::MetalLayerRef;
                view.setLayer(layer_ref as *mut objc::runtime::Object);
                view.setWantsLayer(YES);

                layer
            };

            let layer = (layer.as_ref() as *const MetalLayerRef).cast::<c_void>();
            let info = vk::MetalSurfaceCreateInfoEXT::builder().layer(layer);
            instance.create_metal_surface_ext(&info, None)
        }
        // Windows
        #[cfg(target_os = "windows")]
        (Ok(RawDisplayHandle::Windows(_)), Ok(RawWindowHandle::Win32(window))) => {
            use vk::KhrWin32SurfaceExtension;

            let hinstance_ptr = window
                .hinstance
                .map(|hinstance| hinstance.get() as vk::HINSTANCE)
                .unwrap_or(std::ptr::null_mut());
            let hwnd_ptr = window.hwnd.get() as vk::HWND;

            let info = vk::Win32SurfaceCreateInfoKHR::builder()
                .hinstance(hinstance_ptr)
                .hwnd(hwnd_ptr);

            instance.create_win32_surface_khr(&info, None)
        }
        // Unsupported (currently)
        _ => unimplemented!(),
    }
}
