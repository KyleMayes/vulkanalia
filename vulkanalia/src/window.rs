// SPDX-License-Identifier: Apache-2.0

//! Window integration.

use std::collections::HashSet;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::prelude::v1_0::*;

/// Gets the required instance extensions for window integration.
#[allow(deprecated)]
pub fn get_required_instance_extensions(
    entry: &Entry,
) -> VkResult<Vec<&'static vk::ExtensionName>> {
    let required = &[
        vk::EXT_METAL_SURFACE_EXTENSION,
        vk::KHR_ANDROID_SURFACE_EXTENSION,
        vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION,
        vk::KHR_GET_SURFACE_CAPABILITIES2_EXTENSION,
        vk::KHR_SURFACE_EXTENSION,
        vk::KHR_WAYLAND_SURFACE_EXTENSION,
        vk::KHR_WIN32_SURFACE_EXTENSION,
        vk::KHR_XCB_SURFACE_EXTENSION,
        vk::KHR_XLIB_SURFACE_EXTENSION,
        vk::MVK_IOS_SURFACE_EXTENSION,
        vk::MVK_MACOS_SURFACE_EXTENSION,
    ];

    let available = entry
        .enumerate_instance_extension_properties(None)?
        .into_iter()
        .map(|e| e.extension_name)
        .collect::<HashSet<_>>();

    Ok(required.iter().filter(|e| available.contains(*e)).collect())
}

/// Creates a surface for a window (Windows).
#[cfg(windows)]
pub fn create_surface(
    instance: &Instance,
    window: &dyn HasRawWindowHandle,
) -> VkResult<vk::SurfaceKHR> {
    use vk::KhrWin32SurfaceExtension;

    let window = if let RawWindowHandle::Windows(window) = window.raw_window_handle() {
        window
    } else {
        unreachable!()
    };

    let info = vk::Win32SurfaceCreateInfoKHR::builder()
        .hinstance(window.hinstance)
        .hwnd(window.hwnd);
    instance.create_win32_surface_khr(&info, None)
}

/// Creates a surface for a window (Android).
#[cfg(target_os = "android")]
pub fn create_surface(
    instance: &Instance,
    window: &dyn HasRawWindowHandle,
) -> VkResult<vk::SurfaceKHR> {
    unimplemented!()
}

/// Creates a surface for a window (iOS).
#[cfg(target_os = "ios")]
pub fn create_surface(
    instance: &Instance,
    window: &dyn HasRawWindowHandle,
) -> VkResult<vk::SurfaceKHR> {
    unimplemented!()
}

/// Creates a surface for a window (macOS).
#[cfg(target_os = "macos")]
#[allow(deprecated)]
pub fn create_surface(
    instance: &Instance,
    window: &dyn HasRawWindowHandle,
) -> VkResult<vk::SurfaceKHR> {
    use std::mem;
    use std::os::raw::c_void;

    use cocoa::appkit::{NSView, NSWindow};
    use cocoa::base::id;
    use metal::{CoreAnimationLayer, CoreAnimationLayerRef};
    use objc::runtime::YES;
    use vk::ExtMetalSurfaceExtension;
    use vk::MvkMacosSurfaceExtension;

    let window = if let RawWindowHandle::MacOS(window) = window.raw_window_handle() {
        window
    } else {
        unreachable!()
    };

    let (view, layer) = unsafe {
        let id = mem::transmute::<_, id>(window.ns_window);

        let view = id.contentView();

        let layer = CoreAnimationLayer::new();
        layer.set_contents_scale(view.backingScaleFactor());
        layer.set_edge_antialiasing_mask(0);
        layer.set_presents_with_transaction(false);
        layer.remove_all_animations();

        view.setLayer(mem::transmute(layer.as_ref()));
        view.setWantsLayer(YES);

        (&mut *window.ns_view, layer)
    };

    if instance
        .extensions()
        .contains(&vk::EXT_METAL_SURFACE_EXTENSION)
    {
        // 1. Metal
        let layer = (layer.as_ref() as *const CoreAnimationLayerRef).cast::<c_void>();
        let info = vk::MetalSurfaceCreateInfoEXT::builder().layer(layer);
        instance.create_metal_surface_ext(&info, None)
    } else {
        // 2. macOS
        let info = vk::MacOSSurfaceCreateInfoMVK::builder().view(view);
        instance.create_mac_os_surface_mvk(&info, None)
    }
}

/// Creates a surface for a window (BSD / Linux).
#[cfg(all(
    unix,
    not(target_os = "android"),
    not(target_os = "ios"),
    not(target_os = "macos")
))]
pub fn create_surface(
    instance: &Instance,
    window: &dyn HasRawWindowHandle,
) -> VkResult<vk::SurfaceKHR> {
    use vk::KhrWaylandSurfaceExtension;
    use vk::KhrXcbSurfaceExtension;
    use vk::KhrXlibSurfaceExtension;

    match window.raw_window_handle() {
        RawWindowHandle::Wayland(window) => {
            let info = vk::WaylandSurfaceCreateInfoKHR::builder()
                .display(window.display)
                .surface(window.surface);
            instance.create_wayland_surface_khr(&info, None)
        }
        RawWindowHandle::Xlib(window) => {
            let info = vk::XlibSurfaceCreateInfoKHR::builder()
                .dpy(unsafe { &mut (*(window.display as *mut _)) })
                .window(window.window);
            instance.create_xlib_surface_khr(&info, None)
        }
        RawWindowHandle::Xcb(window) => {
            let info = vk::XcbSurfaceCreateInfoKHR::builder()
                .connection(window.connection)
                .window(window.window as _);
            instance.create_xcb_surface_khr(&info, None)
        }
        _ => unreachable!(),
    }
}
