// SPDX-License-Identifier: Apache-2.0

#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

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
struct App {}

impl App {
    /// Creates our Vulkan app.
    unsafe fn create(window: &Window) -> Result<Self> {
        Ok(Self {})
    }

    /// Renders a frame for our Vulkan app.
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app.
    unsafe fn destroy(&mut self) {}
}

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {}
