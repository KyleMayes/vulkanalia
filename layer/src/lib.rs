// SPDX-License-Identifier: Apache-2.0

//! This example layer was based off of:
//! https://github.com/baldurk/sample_layer/blob/b9e688b9c76388c53a940b11583483a97754a882/sample_layer.cpp
//!
//! See this documentation for how layers interact with the Vulkan loader:
//! https://vulkan.lunarg.com/doc/view/1.4.313.0/windows/LoaderLayerInterface.html

#![allow(unsafe_op_in_unsafe_fn)]

use std::collections::HashMap;
use std::ffi::{CStr, c_char};
use std::mem;
use std::sync::RwLock;

use lazy_static::lazy_static;
use vulkanalia::chain::input_chain;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::DispatchableHandle;
use vulkanalia::vk::layer::{LayerDeviceCreateInfo, LayerFunction, LayerInstanceCreateInfo};

/// Gets the address of a function exported from this shared library.
macro_rules! proc_addr {
    ($proc:expr) => {
        Some(mem::transmute($proc as *const ()))
    };
}

/// Info tracked about an instance created using this layer.
struct InstanceInfo {
    /// The instance created by this layer.
    instance: Instance,
    /// The function provided by the Vulkan loader to load the real instance commands.
    next_get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
}

/// Info tracked about a device created using this layer.
struct DeviceInfo {
    /// The device created by this layer.
    device: Device,
    /// The function provided by the Vulkan loader to load the real device commands.
    next_get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
}

lazy_static! {
    static ref INSTANCES: RwLock<HashMap<usize, InstanceInfo>> = Default::default();
    static ref DEVICES: RwLock<HashMap<usize, DeviceInfo>> = Default::default();
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn get_instance_proc_addr(
    instance: vk::Instance,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    match CStr::from_ptr(name).to_bytes() {
        // Return this layer's implementations of these functions.
        b"vkGetInstanceProcAddr" => proc_addr!(get_instance_proc_addr),
        b"vkCreateInstance" => proc_addr!(create_instance),
        b"vkDestroyInstance" => proc_addr!(destroy_instance),
        b"vkCreateDevice" => proc_addr!(create_device),
        b"vkDestroyDevice" => proc_addr!(destroy_device),
        // Return the default implementation of all other functions.
        _ => (INSTANCES
            .read()
            .unwrap()
            .get(&instance.dispatch_key())
            .unwrap()
            .next_get_instance_proc_addr)(instance, name),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn get_device_proc_addr(
    device: vk::Device,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    match CStr::from_ptr(name).to_bytes() {
        // Return this layer's implementations of these functions.
        b"vkGetDeviceProcAddr" => proc_addr!(get_device_proc_addr),
        b"vkCreateDevice" => proc_addr!(create_device),
        b"vkDestroyDevice" => proc_addr!(destroy_device),
        b"vkBeginCommandBuffer" => proc_addr!(begin_command_buffer),
        b"vkEndCommandBuffer" => proc_addr!(end_command_buffer),
        // Return the default implementation of all other functions.
        _ => (DEVICES
            .read()
            .unwrap()
            .get(&device.dispatch_key())
            .unwrap()
            .next_get_device_proc_addr)(device, name),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn create_instance(
    create_info: *const vk::InstanceCreateInfo,
    allocator: *const vk::AllocationCallbacks,
    instance: *mut vk::Instance,
) -> vk::Result {
    // The instance create info will have had loader information appended to its
    // structure pointer chain by the Vulkan loader. We'll use this to get the
    // function pointer to load Vulkan instance commands in our layer.

    let layer_link_info = input_chain((*create_info).next)
        .filter(|p| p.as_base_ref().s_type == vk::StructureType::LOADER_INSTANCE_CREATE_INFO)
        .map(|p| *p.as_ref::<LayerInstanceCreateInfo>())
        .filter(|p| p.function == LayerFunction::LAYER_LINK_INFO)
        .next()
        .expect("Missing layer link info.");

    let layer_info = &mut *layer_link_info.payload.layer_info;

    let static_commands = vk::StaticCommands {
        get_instance_proc_addr: layer_info.next_get_instance_proc_addr,
    };

    // Create an instance that will use our layer.

    let entry = Entry::from_commands(&static_commands);

    let result = (entry.commands().create_instance)(create_info, allocator, instance);
    if result != vk::Result::SUCCESS {
        return result;
    }

    let instance = Instance::from_created(&static_commands, &*create_info, *instance).unwrap();

    // Store info about the instance in a global map so that we can access it
    // in Vulkan commands that our layer intercepts.
    //
    // See the documentation for `DispatchableHandle::dispatch_key` for how it works as a key:
    // https://docs.rs/vulkanalia/latest/vulkanalia/vk/trait.DispatchableHandle.html#method.dispatch_key

    let info = InstanceInfo {
        instance: instance.clone(),
        next_get_instance_proc_addr: layer_info.next_get_instance_proc_addr,
    };

    INSTANCES
        .write()
        .unwrap()
        .insert(instance.handle().dispatch_key(), info);

    vk::Result::SUCCESS
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn destroy_instance(
    instance: vk::Instance,
    allocator: *const vk::AllocationCallbacks,
) {
    let mut instances = INSTANCES.write().unwrap();
    if let Some(info) = instances.remove(&instance.dispatch_key()) {
        (info.instance.commands().destroy_instance)(instance, allocator);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn create_device(
    physical_device: vk::PhysicalDevice,
    create_info: *const vk::DeviceCreateInfo,
    allocator: *const vk::AllocationCallbacks,
    device: *mut vk::Device,
) -> vk::Result {
    // The device create info will have had loader information appended to its
    // structure pointer chain by the Vulkan loader. We'll use this to get the
    // function pointer to load Vulkan instance or device commands in our layer.

    let layer_link_info = input_chain((*create_info).next)
        .filter(|p| p.as_base_ref().s_type == vk::StructureType::LOADER_DEVICE_CREATE_INFO)
        .map(|p| *p.as_ref::<LayerDeviceCreateInfo>())
        .filter(|p| p.function == LayerFunction::LAYER_LINK_INFO)
        .next()
        .expect("Missing layer link info.");

    let layer_info = &mut *layer_link_info.payload.layer_info;

    let static_commands = vk::StaticCommands {
        get_instance_proc_addr: layer_info.next_get_instance_proc_addr,
    };

    let load = |n| (static_commands.get_instance_proc_addr)(vk::Instance::null(), n);
    let create_device = load(c"vkCreateDevice".as_ptr()).unwrap();
    let create_device = mem::transmute::<_, vk::PFN_vkCreateDevice>(create_device);

    let result = (create_device)(physical_device, create_info, allocator, device);
    if result != vk::Result::SUCCESS {
        return result;
    }

    // Create a device that will use our layer.

    let device = Device::from_created(
        layer_info.next_get_device_proc_addr,
        physical_device,
        &*create_info,
        *device,
    )
    .unwrap();

    // Store info about the device in a global map so that we can access it
    // in Vulkan commands that our layer intercepts.
    //
    // See the documentation for `DispatchableHandle::dispatch_key` for how it works as a key:
    // https://docs.rs/vulkanalia/latest/vulkanalia/vk/trait.DispatchableHandle.html#method.dispatch_key

    let info = DeviceInfo {
        device: device.clone(),
        next_get_device_proc_addr: layer_info.next_get_device_proc_addr,
    };

    DEVICES
        .write()
        .unwrap()
        .insert(device.handle().dispatch_key(), info);

    vk::Result::SUCCESS
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn destroy_device(
    device: vk::Device,
    allocator: *const vk::AllocationCallbacks,
) {
    let mut devices = DEVICES.write().unwrap();
    if let Some(info) = devices.remove(&device.dispatch_key()) {
        (info.device.commands().destroy_device)(device, allocator);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn begin_command_buffer(
    command_buffer: vk::CommandBuffer,
    begin_info: *const vk::CommandBufferBeginInfo,
) -> vk::Result {
    // Do custom stuff before calling the real command.
    println!("im in ur vulkan intercepting ur vkBeginCommandBuffer");

    // Call the real command.
    (DEVICES
        .read()
        .unwrap()
        .get(&command_buffer.dispatch_key())
        .unwrap()
        .device
        .commands()
        .begin_command_buffer)(command_buffer, begin_info)
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn end_command_buffer(command_buffer: vk::CommandBuffer) -> vk::Result {
    // Do custom stuff before calling the real command.
    println!("im in ur vulkan intercepting ur vkEndCommandBuffer");

    // Call the real command.
    (DEVICES
        .read()
        .unwrap()
        .get(&command_buffer.dispatch_key())
        .unwrap()
        .device
        .commands()
        .end_command_buffer)(command_buffer)
}
