// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::ffi::{c_char, c_void, CStr};
use std::mem;
use std::sync::Mutex;

use lazy_static::lazy_static;
use vulkanalia::chain::input_chain;
use vulkanalia::vk::layer::{LayerDeviceCreateInfo, LayerFunction, LayerInstanceCreateInfo};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::DispatchableHandle;

macro_rules! proc_addr {
    ($proc:expr) => {
        Some(mem::transmute($proc as *const ()))
    };
}

struct InstanceInfo {
    static_commands: vk::StaticCommands,
    instance: Instance,
}

struct DeviceInfo {
    static_commands: vk::StaticCommands,
    device: Device,
}

unsafe fn get_key<H: DispatchableHandle>(handle: H) -> usize {
    // Not sure what is going on here, mindless copying this:
    // https://github.com/baldurk/sample_layer/blob/master/sample_layer.cpp#L21-L26
    // Need to research more about dispatching.
    let pointer = *(handle.as_raw() as *const *const c_void);
    pointer as usize
}

lazy_static! {
    static ref INSTANCES: Mutex<HashMap<usize, InstanceInfo>> = Default::default();
    static ref DEVICES: Mutex<HashMap<usize, DeviceInfo>> = Default::default();
}

#[no_mangle]
pub unsafe extern "system" fn get_instance_proc_addr(
    instance: vk::Instance,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    match CStr::from_ptr(name).to_bytes() {
        b"vkGetInstanceProcAddr" => proc_addr!(get_instance_proc_addr),
        b"vkCreateInstance" => proc_addr!(create_instance),
        b"vkDestroyInstance" => proc_addr!(destroy_instance),
        b"vkCreateDevice" => proc_addr!(create_device),
        b"vkDestroyDevice" => proc_addr!(destroy_device),
        _ => (INSTANCES
            .lock()
            .unwrap()
            .get(&get_key(instance))
            .unwrap()
            .static_commands
            .get_instance_proc_addr)(instance, name),
    }
}

#[no_mangle]
pub unsafe extern "system" fn create_instance(
    create_info: *const vk::InstanceCreateInfo,
    allocator: *const vk::AllocationCallbacks,
    instance: *mut vk::Instance,
) -> vk::Result {
    let layer_link_info = input_chain((*create_info).next)
        .filter(|p| p.as_base_ref().s_type == vk::StructureType::LOADER_INSTANCE_CREATE_INFO)
        .map(|p| *p.as_ref::<LayerInstanceCreateInfo>())
        .filter(|p| p.function == LayerFunction::LAYER_LINK_INFO)
        .next()
        .expect("Missing layer link info.");

    let layer_info = &mut *layer_link_info.payload.layer_info;

    let static_commands = vk::StaticCommands {
        get_instance_proc_addr: layer_info.next_get_instance_proc_addr,
        get_device_proc_addr: get_device_proc_addr,
    };

    let entry = Entry::from_commands(&static_commands);

    let result = (entry.commands().create_instance)(create_info, allocator, instance);
    if result != vk::Result::SUCCESS {
        return result;
    }

    let instance = Instance::from_created(&entry, &*create_info, *instance).unwrap();

    let info = InstanceInfo {
        static_commands,
        instance: instance.clone(),
    };

    INSTANCES
        .lock()
        .unwrap()
        .insert(get_key(instance.handle()), info);

    vk::Result::SUCCESS
}

#[no_mangle]
pub unsafe extern "system" fn destroy_instance(
    instance: vk::Instance,
    allocator: *const vk::AllocationCallbacks,
) {
    let mut instances = INSTANCES.lock().unwrap();
    if let Some(info) = instances.remove(&get_key(instance)) {
        (info.instance.commands().destroy_instance)(instance, allocator);
    }
}

#[no_mangle]
pub unsafe extern "system" fn get_device_proc_addr(
    device: vk::Device,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    match CStr::from_ptr(name).to_bytes() {
        b"vkGetDeviceProcAddr" => proc_addr!(get_device_proc_addr),
        b"vkCreateDevice" => proc_addr!(create_device),
        b"vkDestroyDevice" => proc_addr!(destroy_device),
        b"vkBeginCommandBuffer" => proc_addr!(begin_command_buffer),
        b"vkEndCommandBuffer" => proc_addr!(end_command_buffer),
        b"vkCmdDraw" => proc_addr!(cmd_draw),
        b"vkCmdDrawIndexed" => proc_addr!(cmd_draw_indexed),
        _ => (DEVICES
            .lock()
            .unwrap()
            .get(&get_key(device))
            .unwrap()
            .static_commands
            .get_device_proc_addr)(device, name),
    }
}

#[no_mangle]
pub unsafe extern "system" fn create_device(
    physical_device: vk::PhysicalDevice,
    create_info: *const vk::DeviceCreateInfo,
    allocator: *const vk::AllocationCallbacks,
    device: *mut vk::Device,
) -> vk::Result {
    let layer_link_info = input_chain((*create_info).next)
        .filter(|p| p.as_base_ref().s_type == vk::StructureType::LOADER_DEVICE_CREATE_INFO)
        .map(|p| *p.as_ref::<LayerDeviceCreateInfo>())
        .filter(|p| p.function == LayerFunction::LAYER_LINK_INFO)
        .next()
        .expect("Missing layer link info.");

    let layer_info = &mut *layer_link_info.payload.layer_info;

    let static_commands = vk::StaticCommands {
        get_instance_proc_addr: layer_info.next_get_instance_proc_addr,
        get_device_proc_addr: layer_info.next_get_device_proc_addr,
    };

    let entry = Entry::from_commands(&static_commands);

    let load = |n| (static_commands.get_instance_proc_addr)(vk::Instance::null(), n);
    let create_device = load(c"vkCreateDevice".as_ptr()).unwrap();
    let create_device = mem::transmute::<_, vk::PFN_vkCreateDevice>(create_device);

    let result = (create_device)(physical_device, create_info, allocator, device);
    if result != vk::Result::SUCCESS {
        return result;
    }

    let device = Device::from_created(&entry, physical_device, &*create_info, *device).unwrap();

    let info = DeviceInfo {
        static_commands,
        device: device.clone(),
    };

    DEVICES
        .lock()
        .unwrap()
        .insert(get_key(device.handle()), info);

    vk::Result::SUCCESS
}

#[no_mangle]
pub unsafe extern "system" fn destroy_device(
    device: vk::Device,
    allocator: *const vk::AllocationCallbacks,
) {
    let mut devices = DEVICES.lock().unwrap();
    if let Some(info) = devices.remove(&get_key(device)) {
        (info.device.commands().destroy_device)(device, allocator);
    }
}

#[no_mangle]
pub unsafe extern "system" fn begin_command_buffer(
    command_buffer: vk::CommandBuffer,
    begin_info: *const vk::CommandBufferBeginInfo,
) -> vk::Result {
    (DEVICES
        .lock()
        .unwrap()
        .get(&get_key(command_buffer))
        .unwrap()
        .device
        .commands()
        .begin_command_buffer)(command_buffer, begin_info)
}

#[no_mangle]
pub unsafe extern "system" fn end_command_buffer(command_buffer: vk::CommandBuffer) -> vk::Result {
    (DEVICES
        .lock()
        .unwrap()
        .get(&get_key(command_buffer))
        .unwrap()
        .device
        .commands()
        .end_command_buffer)(command_buffer)
}

#[no_mangle]
pub unsafe extern "system" fn cmd_draw(
    command_buffer: vk::CommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    (DEVICES
        .lock()
        .unwrap()
        .get(&get_key(command_buffer))
        .unwrap()
        .device
        .commands()
        .cmd_draw)(
        command_buffer,
        vertex_count,
        instance_count,
        first_vertex,
        first_instance,
    );
}

#[no_mangle]
pub unsafe extern "system" fn cmd_draw_indexed(
    command_buffer: vk::CommandBuffer,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
) {
    (DEVICES
        .lock()
        .unwrap()
        .get(&get_key(command_buffer))
        .unwrap()
        .device
        .commands()
        .cmd_draw_indexed)(
        command_buffer,
        index_count,
        instance_count,
        first_index,
        vertex_offset,
        first_instance,
    );
}
