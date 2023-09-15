// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::missing_safety_doc)]

use std::cell::RefCell;
use std::collections::HashSet;
use std::env;
use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::ptr::copy_nonoverlapping as memcpy;

use anyhow::{anyhow, Result};
use log::*;
use thiserror::Error;
use vulkanalia::bytecode::Bytecode;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::window as vk_window;
use vulkanalia::Version;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use vk::{KhrSurfaceExtension, KhrSwapchainExtension};

/// The required instance and device layer if validation is enabled.
const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

/// The required device extensions.
const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];
/// The Vulkan SDK version that started requiring the portability subset extension for macOS.
const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

/// The number of frames that will processed concurrently.
const MAX_FRAMES_IN_FLIGHT: usize = 2;

/// An example Vulkan app implementation.
pub trait Example {
    /// Creates the render pass for this implementation.
    unsafe fn create_render_pass(&self, device: &Device, data: &mut AppData) -> Result<()>;
    /// Creates the pipeline layout and pipeline for this implementation.
    unsafe fn create_pipeline(&self, device: &Device, data: &mut AppData) -> Result<()>;
    /// Records commands in the command buffers for this implementation.
    unsafe fn record_commands(&mut self, instance: &Instance, device: &Device, data: &AppData) -> Result<()>;
    /// Destroys the Vulkan handles used by this implementation.
    unsafe fn destroy(&self, device: &Device);
}

/// An example Vulkan app.
pub struct App {
    pub example: RefCell<Box<dyn Example>>,
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,
    pub entry: Entry,
    pub instance: Instance,
    pub data: AppData,
    pub device: Device,
    pub frame: usize,
    pub resized: bool,
}

impl App {
    /// Creates an example Vulkan app.
    pub unsafe fn new(title: &str, mut example: impl Example + 'static) -> Result<Self> {
        pretty_env_logger::init();
        info!("Starting app.");

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(800, 600))
            .build(&event_loop)?;

        let mut data = AppData::default();

        let enable = env::var("VK_ENABLE_VALIDATION").unwrap_or_else(|_| "".into());
        data.validation = enable == "1" || enable == "true";
        if data.validation {
            info!("Validation layers requested.");
        }

        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let instance = create_instance(&window, &entry, &mut data)?;
        data.surface = vk_window::create_surface(&instance, &window, &window)?;
        pick_physical_device(&instance, &mut data)?;
        let device = create_logical_device(&entry, &instance, &mut data)?;
        create_swapchain(&window, &instance, &device, &mut data)?;
        create_swapchain_image_views(&device, &mut data)?;
        example.create_render_pass(&device, &mut data)?;
        example.create_pipeline(&device, &mut data)?;
        create_command_pool(&instance, &device, &mut data)?;
        create_framebuffers(&device, &mut data)?;
        create_command_buffers(&device, &mut data)?;
        example.record_commands(&instance, &device, &data)?;
        create_sync_objects(&device, &mut data)?;

        Ok(Self {
            example: RefCell::new(Box::new(example)),
            event_loop: Some(event_loop),
            window,
            entry,
            instance,
            data,
            device,
            frame: 0,
            resized: false,
        })
    }

    /// Starts the rendering loop for this app.
    #[rustfmt::skip]
    pub unsafe fn run(mut self) -> ! {
        let mut destroying = false;
        self.event_loop.take().unwrap().run(move |event, _, flow| {
            *flow = ControlFlow::Poll;
            match event {
                // Render a frame for this app.
                Event::MainEventsCleared if !destroying => self.render().unwrap(),
                // Recreate the swapchain on next render.
                Event::WindowEvent { event: WindowEvent::Resized(_), .. } => self.resized = true,
                // Destroy this app.
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    *flow = ControlFlow::Exit;
                    destroying = true;
                    self.device.device_wait_idle().unwrap();
                    self.destroy();
                }
                _ => {}
            }
        });
    }

    /// Renders a frame for this app.
    unsafe fn render(&mut self) -> Result<()> {
        // Wait for any previous render of the current frame to complete.
        let fence = self.data.in_flight_fences[self.frame];
        self.device.wait_for_fences(&[fence], true, u64::max_value())?;

        // Get the swapchain image for the current frame.
        let result = self.device.acquire_next_image_khr(
            self.data.swapchain,
            u64::max_value(),
            self.data.image_available_semaphores[self.frame],
            vk::Fence::null(),
        );

        // Recreate the swapchain if the swapchain image is out of date.
        let image_index = match result {
            Ok((_, vk::SuccessCode::SUBOPTIMAL_KHR)) => return self.recreate_swapchain(),
            Ok((image_index, _)) => image_index,
            Err(vk::ErrorCode::OUT_OF_DATE_KHR) => return self.recreate_swapchain(),
            Err(e) => return Err(anyhow!(e)),
        };

        // Wait for any previous render using the current swapchain image to complete.
        let previous = self.data.images_in_flight[image_index as usize];
        if previous != vk::Fence::null() {
            self.device.wait_for_fences(&[previous], true, u64::max_value())?;
        }

        // Mark this render as using the current swapchain image.
        self.data.images_in_flight[image_index as usize] = fence;

        // Construct the command buffer submission information.
        let wait_semaphores = &[self.data.image_available_semaphores[self.frame]];
        let signal_semaphores = &[self.data.render_finished_semaphores[self.frame]];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = &[self.data.command_buffers[image_index as usize]];
        let submit = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores);

        // Submit the command buffer to render to the current swapchain image.
        self.device.reset_fences(&[fence])?;
        self.device.queue_submit(self.data.graphics_queue, &[submit], fence)?;

        // Construct the swapchain image presentation info.
        let swapchains = &[self.data.swapchain];
        let image_indices = &[image_index];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(signal_semaphores)
            .swapchains(swapchains)
            .image_indices(image_indices);

        // Present the current swapchain image and recreate the swapchain if the
        // current swapchain image is out of date or suboptimal.
        let result = self.device.queue_present_khr(self.data.present_queue, &present_info);
        let changed = result == Ok(vk::SuccessCode::SUBOPTIMAL_KHR) || result == Err(vk::ErrorCode::OUT_OF_DATE_KHR);
        if self.resized || changed {
            self.resized = false;
            self.recreate_swapchain()?;
        } else if let Err(e) = result {
            return Err(anyhow!(e));
        }

        // Advance the current frame.
        self.frame = (self.frame + 1) % MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

    /// Destroys the Vulkan handles used by this app.
    #[rustfmt::skip]
    unsafe fn destroy(&self) {
        info!("Destroying app.");
        self.destroy_swapchain();
        self.example.borrow().destroy(&self.device);
        self.data.in_flight_fences.iter().for_each(|f| self.device.destroy_fence(*f, None));
        self.data.render_finished_semaphores.iter().for_each(|s| self.device.destroy_semaphore(*s, None));
        self.data.image_available_semaphores.iter().for_each(|s| self.device.destroy_semaphore(*s, None));
        self.device.destroy_device(None);
        self.instance.destroy_surface_khr(self.data.surface, None);
        self.instance.destroy_instance(None);
    }

    /// Recreates the swapchain used by this app.
    #[rustfmt::skip]
    unsafe fn recreate_swapchain(&mut self) -> Result<()> {
        let mut example = self.example.borrow_mut();
        self.device.device_wait_idle()?;
        self.destroy_swapchain();
        create_swapchain(&self.window, &self.instance, &self.device, &mut self.data)?;
        create_swapchain_image_views(&self.device, &mut self.data)?;
        example.create_render_pass(&self.device, &mut self.data)?;
        example.create_pipeline(&self.device, &mut self.data)?;
        create_framebuffers(&self.device, &mut self.data)?;
        create_command_pool(&self.instance, &self.device, &mut self.data)?;
        create_command_buffers(&self.device, &mut self.data)?;
        example.record_commands(&self.instance, &self.device, &self.data)?;
        Ok(())
    }

    /// Destroys the swapchain-related Vulkan handles used by this app.
    #[rustfmt::skip]
    unsafe fn destroy_swapchain(&self) {
        self.device.free_command_buffers(self.data.command_pool, &self.data.command_buffers);
        self.data.framebuffers.iter().for_each(|f| self.device.destroy_framebuffer(*f, None));
        self.device.destroy_command_pool(self.data.command_pool, None);
        self.device.destroy_pipeline(self.data.pipeline, None);
        self.device.destroy_pipeline_layout(self.data.pipeline_layout, None);
        self.device.destroy_render_pass(self.data.render_pass, None);
        self.data.swapchain_image_views.iter().for_each(|v| self.device.destroy_image_view(*v, None));
        self.device.destroy_swapchain_khr(self.data.swapchain, None);
    }
}

/// The Vulkan handles and associated properties used by an example Vulkan app.
#[derive(Clone, Debug, Default)]
pub struct AppData {
    pub validation: bool,
    // Surface
    pub surface: vk::SurfaceKHR,
    // Physical Device / Logical Device
    pub physical_device: vk::PhysicalDevice,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
    // Swapchain
    pub swapchain_format: vk::Format,
    pub swapchain_extent: vk::Extent2D,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_image_views: Vec<vk::ImageView>,
    // Pipeline
    pub render_pass: vk::RenderPass,
    pub pipeline_layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
    // Command Pool
    pub command_pool: vk::CommandPool,
    // Framebuffers
    pub framebuffers: Vec<vk::Framebuffer>,
    // Command Buffers
    pub command_buffers: Vec<vk::CommandBuffer>,
    // Sync Objects
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub in_flight_fences: Vec<vk::Fence>,
    pub images_in_flight: Vec<vk::Fence>,
}

//================================================
// Instance
//================================================

/// Creates an instance.
unsafe fn create_instance(window: &Window, entry: &Entry, data: &mut AppData) -> Result<Instance> {
    // Application Info

    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"Vulkanalia\0")
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

    if data.validation && !available_layers.contains(&VALIDATION_LAYER) {
        return Err(anyhow!("Validation layers requested but not supported."));
    }

    let layers = if data.validation {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        Vec::new()
    };

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

    if data.validation {
        extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
    }

    // Create

    let mut info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .flags(flags);

    let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
        .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
        .user_callback(Some(debug_callback));

    if data.validation {
        info = info.push_next(&mut debug_info);
    }

    Ok(entry.create_instance(&info, None)?)
}

/// Logs debug messages.
extern "system" fn debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    type_: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

    if severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR) {
        error!("({:?}) {}", type_, message);
    } else if severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::WARNING) {
        warn!("({:?}) {}", type_, message);
    } else if severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::INFO) {
        debug!("({:?}) {}", type_, message);
    } else {
        trace!("({:?}) {}", type_, message);
    }

    vk::FALSE
}

//================================================
// Physical Device
//================================================

/// An error that indicates a missing requirement for a physical device.
#[derive(Debug, Error)]
#[error("{0}")]
pub struct SuitabilityError(pub &'static str);

/// Picks a suitable physical device.
unsafe fn pick_physical_device(instance: &Instance, data: &mut AppData) -> Result<()> {
    for physical_device in instance.enumerate_physical_devices()? {
        let properties = instance.get_physical_device_properties(physical_device);

        if let Err(error) = check_physical_device(instance, data, physical_device) {
            warn!("Skipping physical device (`{}`): {}", properties.device_name, error);
        } else {
            info!("Selected physical device (`{}`).", properties.device_name);
            data.physical_device = physical_device;
            return Ok(());
        }
    }

    Err(anyhow!("Failed to find suitable physical device."))
}

/// Checks that a physical device is suitable.
unsafe fn check_physical_device(
    instance: &Instance,
    data: &AppData,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    QueueFamilyIndices::get(instance, data, physical_device)?;
    check_physical_device_extensions(instance, physical_device)?;

    let support = SwapchainSupport::get(instance, data, physical_device)?;
    if support.formats.is_empty() || support.present_modes.is_empty() {
        return Err(anyhow!(SuitabilityError("Insufficient swapchain support.")));
    }

    Ok(())
}

/// Checks that a physical device supports the required device extensions.
unsafe fn check_physical_device_extensions(instance: &Instance, physical_device: vk::PhysicalDevice) -> Result<()> {
    let extensions = instance
        .enumerate_device_extension_properties(physical_device, None)?
        .iter()
        .map(|e| e.extension_name)
        .collect::<HashSet<_>>();
    if DEVICE_EXTENSIONS.iter().all(|e| extensions.contains(e)) {
        Ok(())
    } else {
        Err(anyhow!(SuitabilityError("Missing required device extensions.")))
    }
}

//================================================
// Logical Device
//================================================

/// Creates a logical device for the picked physical device.
#[allow(unused_variables)]
unsafe fn create_logical_device(entry: &Entry, instance: &Instance, data: &mut AppData) -> Result<Device> {
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

    let layers = if data.validation {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        vec![]
    };

    // Extensions

    let mut extensions = DEVICE_EXTENSIONS.iter().map(|n| n.as_ptr()).collect::<Vec<_>>();

    // Required by Vulkan SDK on macOS since 1.3.216.
    if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
        extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
    }

    // Features

    let features = vk::PhysicalDeviceFeatures::builder().sampler_anisotropy(true);

    // Create

    let info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .enabled_features(&features);

    let device = instance.create_device(data.physical_device, &info, None)?;

    // Queues

    data.graphics_queue = device.get_device_queue(indices.graphics, 0);
    data.present_queue = device.get_device_queue(indices.present, 0);

    Ok(device)
}

//================================================
// Swapchain
//================================================

/// Creates a swapchain and swapchain images.
unsafe fn create_swapchain(window: &Window, instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
    // Image

    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;
    let support = SwapchainSupport::get(instance, data, data.physical_device)?;

    let surface_format = get_swapchain_surface_format(&support.formats);
    let present_mode = get_swapchain_present_mode(&support.present_modes);
    let extent = get_swapchain_extent(window, support.capabilities);

    data.swapchain_format = surface_format.format;
    data.swapchain_extent = extent;

    let image_count = (support.capabilities.min_image_count + 1).min(support.capabilities.max_image_count);

    let mut queue_family_indices = vec![];
    let image_sharing_mode = if indices.graphics != indices.present {
        queue_family_indices.push(indices.graphics);
        queue_family_indices.push(indices.present);
        vk::SharingMode::CONCURRENT
    } else {
        vk::SharingMode::EXCLUSIVE
    };

    // Create

    let info = vk::SwapchainCreateInfoKHR::builder()
        .surface(data.surface)
        .min_image_count(image_count)
        .image_format(surface_format.format)
        .image_color_space(surface_format.color_space)
        .image_extent(extent)
        .image_array_layers(1)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(image_sharing_mode)
        .queue_family_indices(&queue_family_indices)
        .pre_transform(support.capabilities.current_transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true);

    data.swapchain = device.create_swapchain_khr(&info, None)?;

    // Images

    data.swapchain_images = device.get_swapchain_images_khr(data.swapchain)?;

    Ok(())
}

/// Gets a suitable swapchain surface format.
fn get_swapchain_surface_format(formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
    formats
        .iter()
        .cloned()
        .find(|f| f.format == vk::Format::R8G8B8_SRGB && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR)
        .unwrap_or_else(|| formats[0])
}

/// Gets a suitable swapchain present mode.
fn get_swapchain_present_mode(present_modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
    present_modes
        .iter()
        .cloned()
        .find(|m| *m == vk::PresentModeKHR::MAILBOX)
        .unwrap_or(vk::PresentModeKHR::FIFO)
}

/// Gets a suitable swapchain extent.
fn get_swapchain_extent(window: &Window, capabilities: vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
    if capabilities.current_extent.width != u32::max_value() {
        capabilities.current_extent
    } else {
        let size = window.inner_size();
        let clamp = |min: u32, max: u32, v: u32| min.max(max.min(v));
        vk::Extent2D::builder()
            .width(clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
                size.width,
            ))
            .height(clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
                size.height,
            ))
            .build()
    }
}

/// Creates image views for the swapchain images.
unsafe fn create_swapchain_image_views(device: &Device, data: &mut AppData) -> Result<()> {
    data.swapchain_image_views = data
        .swapchain_images
        .iter()
        .map(|i| {
            let components = vk::ComponentMapping::builder()
                .r(vk::ComponentSwizzle::IDENTITY)
                .g(vk::ComponentSwizzle::IDENTITY)
                .b(vk::ComponentSwizzle::IDENTITY)
                .a(vk::ComponentSwizzle::IDENTITY);

            let subresource_range = vk::ImageSubresourceRange::builder()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1);

            let info = vk::ImageViewCreateInfo::builder()
                .image(*i)
                .view_type(vk::ImageViewType::_2D)
                .format(data.swapchain_format)
                .components(components)
                .subresource_range(subresource_range);

            device.create_image_view(&info, None)
        })
        .collect::<Result<_, _>>()?;
    Ok(())
}

//================================================
// Command Pool
//================================================

/// Creates a command pool.
unsafe fn create_command_pool(instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

    let info = vk::CommandPoolCreateInfo::builder().queue_family_index(indices.graphics);

    data.command_pool = device.create_command_pool(&info, None)?;

    Ok(())
}

//================================================
// Command Buffers
//================================================

/// Creates setup and render command buffers.
unsafe fn create_command_buffers(device: &Device, data: &mut AppData) -> Result<()> {
    let info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(data.command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(data.swapchain_images.len() as u32);

    data.command_buffers = device.allocate_command_buffers(&info)?;

    Ok(())
}

//================================================
// Framebuffers
//================================================

/// Creates framebuffers for the swapchain image views.
unsafe fn create_framebuffers(device: &Device, data: &mut AppData) -> Result<()> {
    data.framebuffers = data
        .swapchain_image_views
        .iter()
        .map(|iv| {
            let attachments = &[*iv];
            let info = vk::FramebufferCreateInfo::builder()
                .render_pass(data.render_pass)
                .attachments(attachments)
                .width(data.swapchain_extent.width)
                .height(data.swapchain_extent.height)
                .layers(1);
            device.create_framebuffer(&info, None)
        })
        .collect::<Result<_, _>>()?;
    Ok(())
}

//================================================
// Sync Objects
//================================================

/// Creates synchronization objects to manage command buffer reuse and rendering.
unsafe fn create_sync_objects(device: &Device, data: &mut AppData) -> Result<()> {
    let semaphore_info = vk::SemaphoreCreateInfo::builder();
    let fence_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        data.image_available_semaphores
            .push(device.create_semaphore(&semaphore_info, None)?);
        data.render_finished_semaphores
            .push(device.create_semaphore(&semaphore_info, None)?);
        data.in_flight_fences.push(device.create_fence(&fence_info, None)?);
    }

    data.images_in_flight = data.swapchain_images.iter().map(|_| vk::Fence::null()).collect();

    Ok(())
}

//================================================
// Structs
//================================================

/// The indices of the required queue families for a physical device.
#[derive(Copy, Clone, Debug)]
struct QueueFamilyIndices {
    graphics: u32,
    present: u32,
}

impl QueueFamilyIndices {
    /// Gets the indices of the required queue families for a physical device.
    unsafe fn get(instance: &Instance, data: &AppData, physical_device: vk::PhysicalDevice) -> Result<Self> {
        let properties = instance.get_physical_device_queue_family_properties(physical_device);

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        let mut present = None;
        for (index, _) in properties.iter().enumerate() {
            if instance.get_physical_device_surface_support_khr(physical_device, index as u32, data.surface)? {
                present = Some(index as u32);
                break;
            }
        }

        if let (Some(graphics), Some(present)) = (graphics, present) {
            Ok(Self { graphics, present })
        } else {
            Err(anyhow!(SuitabilityError("Missing required queue families.")))
        }
    }
}

/// The swapchain support for a physical device.
#[derive(Clone, Debug)]
struct SwapchainSupport {
    capabilities: vk::SurfaceCapabilitiesKHR,
    formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
}

impl SwapchainSupport {
    /// Gets the swapchain support for a physical device.
    unsafe fn get(instance: &Instance, data: &AppData, physical_device: vk::PhysicalDevice) -> Result<Self> {
        Ok(Self {
            capabilities: instance.get_physical_device_surface_capabilities_khr(physical_device, data.surface)?,
            formats: instance.get_physical_device_surface_formats_khr(physical_device, data.surface)?,
            present_modes: instance.get_physical_device_surface_present_modes_khr(physical_device, data.surface)?,
        })
    }
}

//================================================
// Shared (buffers)
//================================================

/// Creates a device buffer.
pub unsafe fn create_buffer(
    instance: &Instance,
    device: &Device,
    data: &AppData,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
    properties: vk::MemoryPropertyFlags,
) -> Result<(vk::Buffer, vk::DeviceMemory)> {
    // Buffer

    let buffer_info = vk::BufferCreateInfo::builder()
        .size(size)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    let buffer = device.create_buffer(&buffer_info, None)?;

    // Memory

    let requirements = device.get_buffer_memory_requirements(buffer);
    let memory_type_index = get_memory_type_index(instance, data, properties, requirements)?;

    let memory_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(requirements.size)
        .memory_type_index(memory_type_index);

    let memory = device.allocate_memory(&memory_info, None)?;

    Ok((buffer, memory))
}

/// Fills a device buffer with data.
#[rustfmt::skip]
pub unsafe fn fill_buffer(device: &Device, buffer: vk::Buffer, memory: vk::DeviceMemory, data: &[impl Copy]) -> Result<()> {
    device.bind_buffer_memory(buffer, memory, 0)?;

    let dst = device.map_memory(memory, 0, mem::size_of_val(data) as u64, vk::MemoryMapFlags::empty())?;
    memcpy(data.as_ptr(), dst.cast(), data.len());
    device.unmap_memory(memory);

    Ok(())
}

//================================================
// Shared (shaders)
//================================================

/// Creates a shader module from a compiled shader.
pub unsafe fn create_shader_module(device: &Device, bytecode: &[u8]) -> Result<vk::ShaderModule> {
    let bytecode = Bytecode::new(bytecode).unwrap();

    let info = vk::ShaderModuleCreateInfo::builder()
        .code_size(bytecode.code_size())
        .code(bytecode.code());

    Ok(device.create_shader_module(&info, None)?)
}

//================================================
// Shared (other)
//================================================

/// Gets a suitable memory type index for a device buffer.
pub unsafe fn get_memory_type_index(
    instance: &Instance,
    data: &AppData,
    properties: vk::MemoryPropertyFlags,
    requirements: vk::MemoryRequirements,
) -> Result<u32> {
    let memory = instance.get_physical_device_memory_properties(data.physical_device);
    (0..memory.memory_type_count)
        .find(|i| {
            let suitable = (requirements.memory_type_bits & (1 << i)) != 0;
            let memory_type = memory.memory_types[*i as usize];
            suitable && memory_type.property_flags.contains(properties)
        })
        .ok_or_else(|| anyhow!("Failed to find suitable memory type."))
}
