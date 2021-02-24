// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code, unused_variables, clippy::too_many_arguments, clippy::unnecessary_wraps)]

use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_void;

use anyhow::*;
use log::*;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::window as vk_window;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use vulkanalia::vk::ExtDebugUtilsExtension;
use vulkanalia::vk::KhrSurfaceExtension;

/// Whether the validation layers should be enabled.
const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
/// The name of the validation layers.
const VALIDATION_LAYER: vk::ExtensionName = vk::to_extension_name(b"VK_LAYER_KHRONOS_validation\0");

#[rustfmt::skip]
fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Vulkan Tutorial (Rust)")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    // App

    let mut app = App::create(&window)?;
    let mut destroying = false;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            // Render a frame if our Vulkan app is not being destroyed.
            Event::MainEventsCleared if !destroying => app.render(&window).unwrap(),
            // Destroy our Vulkan app.
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                app.destroy();
            }
            _ => {}
        }
    });
}

/// Our Vulkan app.
#[derive(Clone, Debug)]
struct App {
    entry: Entry,
    instance: Instance,
    data: AppData,
    device: Device,
}

impl App {
    /// Creates our Vulkan app.
    fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let mut data = AppData::default();
        let instance = create_instance(&entry, &mut data)?;
        data.surface = vk_window::create_surface(&instance, window)?;
        pick_physical_device(&instance, &mut data)?;
        let device = create_logical_device(&instance, &mut data)?;
        Ok(Self {
            entry,
            instance,
            data,
            device,
        })
    }

    /// Renders a frame for our Vulkan app.
    fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app.
    #[rustfmt::skip]
    fn destroy(&mut self) {
        self.device.destroy_device(None);
        self.instance.destroy_surface_khr(self.data.surface, None);

        if VALIDATION_ENABLED {
            self.instance.destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }

        self.instance.destroy_instance(None);
    }
}

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {
    // Debug
    messenger: vk::DebugUtilsMessengerEXT,
    // Surface
    surface: vk::SurfaceKHR,
    // Physical Device / Logical Device
    physical_device: vk::PhysicalDevice,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
}

//================================================
// Instance
//================================================

fn create_instance(entry: &Entry, data: &mut AppData) -> Result<Instance> {
    // Application Info

    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"Vulkan Tutorial (Rust)\0")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"No Engine\0")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));

    // Layers

    let available_layers = entry
        .enumerate_instance_layer_properties()?
        .iter()
        .map(|l| l.layer_name)
        .collect::<HashSet<_>>();

    if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
        return Err(anyhow!("Validation layer requested but not supported."));
    }

    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.to_cstr().as_ptr()]
    } else {
        Vec::new()
    };

    // Extensions

    let mut extensions = vk_window::get_required_instance_extensions(entry)?
        .iter()
        .map(|e| e.to_cstr().as_ptr())
        .collect::<Vec<_>>();

    if VALIDATION_ENABLED {
        extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.to_cstr().as_ptr());
    }

    // Create

    let mut info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions);

    let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
        .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
        .user_callback(Some(debug_callback));

    if VALIDATION_ENABLED {
        info = info.push_next(&mut debug_info);
    }

    let instance = entry.create_instance(&info, None)?;

    // Messenger

    if VALIDATION_ENABLED {
        data.messenger = instance.create_debug_utils_messenger_ext(&debug_info, None)?;
    }

    Ok(instance)
}

extern "system" fn debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    type_: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

    if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        error!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        warn!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        debug!("({:?}) {}", type_, message);
    } else {
        trace!("({:?}) {}", type_, message);
    }

    vk::FALSE
}

//================================================
// Physical Device
//================================================

fn pick_physical_device(instance: &Instance, data: &mut AppData) -> Result<()> {
    data.physical_device = instance
        .enumerate_physical_devices()?
        .into_iter()
        .find(|pd| match check_physical_device(instance, data, *pd) {
            Ok(suitable) => suitable,
            Err(e) => {
                warn!("Failed to check physical device suitability: {}", e);
                false
            }
        })
        .ok_or_else(|| anyhow!("Failed to find suitable physical device."))?;

    Ok(())
}

fn check_physical_device(instance: &Instance, data: &AppData, physical_device: vk::PhysicalDevice) -> Result<bool> {
    QueueFamilyIndices::get(instance, data, physical_device)?;
    Ok(true)
}

//================================================
// Logical Device
//================================================

fn create_logical_device(instance: &Instance, data: &mut AppData) -> Result<Device> {
    // Queue Create Infos

    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

    let mut unique_indices = HashSet::new();
    unique_indices.insert(indices.graphics);
    unique_indices.insert(indices.present);

    let queue_priorities = &[1.0];
    let queue_infos = unique_indices
        .iter()
        .map(|i| {
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(*i)
                .queue_priorities(queue_priorities)
        })
        .collect::<Vec<_>>();

    // Layers

    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.to_cstr().as_ptr()]
    } else {
        vec![]
    };

    // Features

    let features = vk::PhysicalDeviceFeatures::builder();

    // Create

    let info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_infos)
        .enabled_layer_names(&layers)
        .enabled_features(&features);

    let device = instance.create_device(data.physical_device, &info, None)?;

    // Queues

    data.graphics_queue = device.get_device_queue(indices.graphics, 0);
    data.present_queue = device.get_device_queue(indices.present, 0);

    Ok(device)
}

//================================================
// Structs
//================================================

#[derive(Copy, Clone, Debug)]
struct QueueFamilyIndices {
    graphics: u32,
    present: u32,
}

impl QueueFamilyIndices {
    fn get(instance: &Instance, data: &AppData, physical_device: vk::PhysicalDevice) -> Result<Self> {
        let properties = instance.get_physical_device_queue_family_properties(physical_device);

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        let mut present = None;
        for (index, properties) in properties.iter().enumerate() {
            if instance.get_physical_device_surface_support_khr(physical_device, index as u32, data.surface)? {
                present = Some(index as u32);
            }
        }

        if let (Some(graphics), Some(present)) = (graphics, present) {
            Ok(Self { graphics, present })
        } else {
            Err(anyhow!("Failed to get required queue family indices."))
        }
    }
}
