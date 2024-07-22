// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;
use core::mem;

use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::{DeviceCommands, InstanceCommands};
use vulkanalia::ResultExt;

use crate::flags::AllocatorCreateFlags;
use crate::vma::*;
use crate::Allocation;

#[derive(Copy, Clone, Debug)]
pub struct AllocatorOptions<'a> {
    pub instance: &'a Instance,
    pub device: &'a Device,
    pub physical_device: vk::PhysicalDevice,
    pub flags: AllocatorCreateFlags,
    pub preferred_large_heap_block_size: vk::DeviceSize,
    pub heap_size_limits: &'a [vk::DeviceSize],
    pub external_memory_handle_types: &'a [vk::ExternalMemoryHandleTypeFlagsKHR],
}

impl<'a> AllocatorOptions<'a> {
    pub fn new(
        instance: &'a Instance,
        device: &'a Device,
        physical_device: vk::PhysicalDevice,
    ) -> Self {
        Self {
            instance,
            device,
            physical_device,
            flags: AllocatorCreateFlags::empty(),
            preferred_large_heap_block_size: 0,
            heap_size_limits: &[],
            external_memory_handle_types: &[],
        }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Allocator(pub(crate) VmaAllocator);

impl Allocator {
    pub unsafe fn new(options: &AllocatorOptions) -> VkResult<Self> {
        let functions = get_functions(options.instance.commands(), options.device.commands());

        let heap_size_limits = if !options.heap_size_limits.is_empty() {
            options.heap_size_limits.as_ptr()
        } else {
            core::ptr::null()
        };

        let external_memory_handle_types = if !options.external_memory_handle_types.is_empty() {
            options.external_memory_handle_types.as_ptr()
        } else {
            core::ptr::null()
        };

        let info = VmaAllocatorCreateInfo {
            instance: options.instance.handle(),
            device: options.device.handle(),
            physicalDevice: options.physical_device,
            flags: options.flags.bits(),
            preferredLargeHeapBlockSize: 0,
            pAllocationCallbacks: core::ptr::null(),
            pDeviceMemoryCallbacks: core::ptr::null(),
            pHeapSizeLimit: heap_size_limits,
            pVulkanFunctions: &functions,
            vulkanApiVersion: options.instance.version().into(),
            pTypeExternalMemoryHandleTypes: external_memory_handle_types,
        };

        unsafe {
            let mut allocator: VmaAllocator = mem::zeroed();
            vmaCreateAllocator(&info, &mut allocator).result()?;
            Ok(Self(allocator))
        }
    }

    pub fn get_physical_device_properties(&self) -> vk::PhysicalDeviceProperties {
        unsafe {
            let mut properties = vk::PhysicalDeviceProperties::default();
            let pointer = &mut properties as *mut _ as *mut *const _;
            vmaGetPhysicalDeviceProperties(self.0, pointer);
            properties
        }
    }

    pub fn get_memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties {
        unsafe {
            let mut properties = core::ptr::null();
            vmaGetMemoryProperties(self.0, &mut properties);
            &*properties
        }
    }

    pub fn calculate_statistics(&self) -> VkResult<VmaTotalStatistics> {
        unsafe {
            let mut statistics: VmaTotalStatistics = mem::zeroed();
            vmaCalculateStatistics(self.0, &mut statistics);
            Ok(statistics)
        }
    }

    pub fn get_heap_budgets(&self) -> VkResult<Vec<VmaBudget>> {
        unsafe {
            let len = self.get_memory_properties().memory_heap_count as usize;
            let mut budgets = Vec::with_capacity(len);
            vmaGetHeapBudgets(self.0, budgets.as_mut_ptr());
            budgets.set_len(len);
            Ok(budgets)
        }
    }

    pub fn check_corruption(&self, flags: vk::MemoryPropertyFlags) -> VkResult<()> {
        unsafe { vmaCheckCorruption(self.0, flags.bits()).result() }
    }

    pub fn get_allocation_info(&self, allocation: Allocation) -> VmaAllocationInfo {
        unsafe {
            let mut info: VmaAllocationInfo = mem::zeroed();
            vmaGetAllocationInfo(self.0, allocation.0, &mut info);
            info
        }
    }

    pub unsafe fn set_current_frame_index(&self, frame_index: u32) {
        vmaSetCurrentFrameIndex(self.0, frame_index);
    }

    pub unsafe fn map_memory(&self, allocation: Allocation) -> VkResult<*mut u8> {
        let mut data = core::ptr::null_mut();
        vmaMapMemory(self.0, allocation.0, &mut data).result()?;
        Ok(data.cast())
    }

    pub unsafe fn unmap_memory(&self, allocation: Allocation) {
        vmaUnmapMemory(self.0, allocation.0);
    }

    pub unsafe fn bind_buffer_memory(
        &self,
        allocation: Allocation,
        buffer: vk::Buffer,
    ) -> VkResult<()> {
        vmaBindBufferMemory(self.0, allocation.0, buffer).result()
    }

    pub unsafe fn bind_buffer_memory2(
        &self,
        allocation: Allocation,
        allocation_local_offset: vk::DeviceSize,
        buffer: vk::Buffer,
        next: *const core::ffi::c_void,
    ) -> VkResult<()> {
        vmaBindBufferMemory2(self.0, allocation.0, allocation_local_offset, buffer, next).result()
    }

    pub unsafe fn bind_image_memory(
        &self,
        allocation: Allocation,
        image: vk::Image,
    ) -> VkResult<()> {
        vmaBindImageMemory(self.0, allocation.0, image).result()
    }

    pub unsafe fn bind_image_memory2(
        &self,
        allocation: Allocation,
        allocation_local_offset: vk::DeviceSize,
        image: vk::Image,
        next: *const core::ffi::c_void,
    ) -> VkResult<()> {
        vmaBindImageMemory2(self.0, allocation.0, allocation_local_offset, image, next).result()
    }

    pub unsafe fn free_memory(&self, allocation: Allocation) {
        vmaFreeMemory(self.0, allocation.0);
    }

    pub unsafe fn free_memory_pages(&self, allocations: &[Allocation]) {
        vmaFreeMemoryPages(self.0, allocations.len(), allocations.as_ptr() as *mut _);
    }

    pub unsafe fn destroy_buffer(&self, buffer: vk::Buffer, allocation: Allocation) {
        vmaDestroyBuffer(self.0, buffer, allocation.0)
    }

    pub unsafe fn destroy_image(&self, image: vk::Image, allocation: Allocation) {
        vmaDestroyImage(self.0, image, allocation.0)
    }

    pub unsafe fn flush_allocation(
        &self,
        allocation: Allocation,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
    ) -> VkResult<()> {
        vmaFlushAllocation(self.0, allocation.0, offset, size).result()
    }

    pub unsafe fn invalidate_allocation(
        &self,
        allocation: Allocation,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
    ) -> VkResult<()> {
        vmaInvalidateAllocation(self.0, allocation.0, offset, size).result()
    }
}

impl Drop for Allocator {
    fn drop(&mut self) {
        unsafe { vmaDestroyAllocator(self.0) };
        self.0 = core::ptr::null_mut();
    }
}

unsafe impl Send for Allocator {}
unsafe impl Sync for Allocator {}

#[rustfmt::skip]
fn get_functions(instance: &InstanceCommands, device: &DeviceCommands) -> VmaVulkanFunctions {
    extern "system" fn get_instance_proc_addr(
        _: vk::Instance,
        _: *const core::ffi::c_char,
    ) -> vk::PFN_vkVoidFunction {
        panic!("VMA_DYNAMIC_VULKAN_FUNCTIONS is not supported!")
    }

    extern "system" fn get_device_proc_addr(
        _: vk::Device,
        _: *const core::ffi::c_char,
    ) -> vk::PFN_vkVoidFunction {
        panic!("VMA_DYNAMIC_VULKAN_FUNCTIONS is not supported!")
    }

    VmaVulkanFunctions {
        vkGetInstanceProcAddr: get_instance_proc_addr,
        vkGetDeviceProcAddr: get_device_proc_addr,
        vkAllocateMemory: device.allocate_memory,
        vkBindBufferMemory: device.bind_buffer_memory,
        vkBindBufferMemory2KHR: device.bind_buffer_memory2_khr,
        vkBindImageMemory: device.bind_image_memory,
        vkBindImageMemory2KHR: device.bind_image_memory2_khr,
        vkCmdCopyBuffer: device.cmd_copy_buffer,
        vkCreateBuffer: device.create_buffer,
        vkCreateImage: device.create_image,
        vkDestroyBuffer: device.destroy_buffer,
        vkDestroyImage: device.destroy_image,
        vkFlushMappedMemoryRanges: device.flush_mapped_memory_ranges,
        vkFreeMemory: device.free_memory,
        vkGetBufferMemoryRequirements: device.get_buffer_memory_requirements,
        vkGetBufferMemoryRequirements2KHR: device.get_buffer_memory_requirements2_khr,
        vkGetDeviceBufferMemoryRequirements: device.get_device_buffer_memory_requirements,
        vkGetDeviceImageMemoryRequirements: device.get_device_image_memory_requirements,
        vkGetImageMemoryRequirements: device.get_image_memory_requirements,
        vkGetImageMemoryRequirements2KHR: device.get_image_memory_requirements2_khr,
        vkGetPhysicalDeviceMemoryProperties: instance.get_physical_device_memory_properties,
        vkGetPhysicalDeviceMemoryProperties2KHR: instance.get_physical_device_memory_properties2_khr,
        vkGetPhysicalDeviceProperties: instance.get_physical_device_properties,
        vkInvalidateMappedMemoryRanges: device.invalidate_mapped_memory_ranges,
        vkMapMemory: device.map_memory,
        vkUnmapMemory: device.unmap_memory,
    }
}
