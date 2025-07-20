// SPDX-License-Identifier: Apache-2.0

#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::{anyhow, Result};
use log::*;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::window as vk_window;
use vulkanalia::Version;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

/// The Vulkan SDK version that started requiring the portability subset extension for macOS.
const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

fn main() -> Result<()> {
    pretty_env_logger::init();

    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut AppWindow::default())?;

    Ok(())
}

// The window for our Vulkan app.
#[derive(Default)]
struct AppWindow {
    window: Option<(Window, App)>,
}

impl ApplicationHandler<()> for AppWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create a window and initialize our Vulkan app.
        let attrs = Window::default_attributes()
            .with_title("Vulkan Tutorial (Rust)")
            .with_inner_size(LogicalSize::new(1024, 768));
        let window = event_loop.create_window(attrs).unwrap();
        let app = unsafe { App::create(&window) }.unwrap();
        self.window = Some((window, app));
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        // Request a redraw when all events were processed.
        let (window, _) = self.window.as_ref().expect("about_to_wait() without window + app");
        window.request_redraw();
    }

    #[rustfmt::skip]
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        // Handle window events.
        let (window, app) = self.window.as_mut().expect("window_event() without window + app");
        match event {
            // Render a frame if our Vulkan app is not being destroyed.
            WindowEvent::RedrawRequested if !event_loop.exiting() => unsafe { app.render(&window) }.unwrap(),
            // Destroy our Vulkan app.
            WindowEvent::CloseRequested => {
                event_loop.exit();
                unsafe { app.destroy() };
            }
            _ => {}
        }
    }
}

/// Our Vulkan app.
#[derive(Clone, Debug)]
struct App {
    entry: Entry,
    instance: Instance,
}

impl App {
    /// Creates our Vulkan app.
    unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let instance = create_instance(window, &entry)?;
        Ok(Self { entry, instance })
    }

    /// Renders a frame for our Vulkan app.
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app.
    unsafe fn destroy(&mut self) {
        self.instance.destroy_instance(None);
    }
}

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {}

//================================================
// Instance
//================================================

unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance> {
    // Application Info

    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"Vulkan Tutorial (Rust)\0")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"No Engine\0")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));

    // Extensions

    let mut extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    // Required by Vulkan SDK on macOS since 1.3.216.
    let flags = if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
        info!("Enabling extensions for macOS portability.");
        extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
        extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    } else {
        vk::InstanceCreateFlags::empty()
    };

    // Create

    let info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_extension_names(&extensions)
        .flags(flags);

    Ok(entry.create_instance(&info, None)?)
}
