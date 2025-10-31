// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;
use core::mem;

use vulkanalia::ResultExt;
use vulkanalia::prelude::v1_0::*;

use crate::allocator::Allocator;
use crate::enums::MemoryUsage;
use crate::flags::AllocationCreateFlags;
use crate::pool::{Pool, PoolHandle};
use crate::vma::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct AllocationOptions {
    pub flags: AllocationCreateFlags,
    pub usage: MemoryUsage,
    pub required_flags: vk::MemoryPropertyFlags,
    pub preferred_flags: vk::MemoryPropertyFlags,
    pub memory_type_bits: u32,
    pub priority: f32,
}

impl From<&AllocationOptions> for VmaAllocationCreateInfo {
    fn from(value: &AllocationOptions) -> Self {
        Self {
            flags: value.flags.bits(),
            usage: value.usage.into(),
            requiredFlags: value.required_flags,
            preferredFlags: value.preferred_flags,
            memoryTypeBits: value.memory_type_bits,
            pool: core::ptr::null_mut(),
            pUserData: core::ptr::null_mut(),
            priority: value.priority,
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Allocation(pub(crate) VmaAllocation);

unsafe impl Send for Allocation {}
unsafe impl Sync for Allocation {}

pub trait Alloc {
    fn allocator(&self) -> &Allocator;

    fn pool(&self) -> PoolHandle;

    unsafe fn find_memory_type_index(
        &self,
        memory_type_bits: u32,
        options: &AllocationOptions,
    ) -> VkResult<u32> {
        let mut info: VmaAllocationCreateInfo = options.into();
        info.pool = self.pool().0;

        let mut memory_type_index = 0;
        vmaFindMemoryTypeIndex(
            self.allocator().0,
            memory_type_bits,
            &info,
            &mut memory_type_index,
        )
        .result()?;

        Ok(memory_type_index)
    }

    unsafe fn find_memory_type_index_for_buffer_info(
        &self,
        buffer_info: impl vk::Cast<Target = vk::BufferCreateInfo>,
        options: &AllocationOptions,
    ) -> VkResult<u32> {
        let mut info: VmaAllocationCreateInfo = options.into();
        info.pool = self.pool().0;

        let mut memory_type_index = 0;
        vmaFindMemoryTypeIndexForBufferInfo(
            self.allocator().0,
            buffer_info.as_ref(),
            &info,
            &mut memory_type_index,
        )
        .result()?;

        Ok(memory_type_index)
    }

    unsafe fn find_memory_type_index_for_image_info(
        &self,
        image_info: impl vk::Cast<Target = vk::ImageCreateInfo>,
        options: &AllocationOptions,
    ) -> VkResult<u32> {
        let mut info: VmaAllocationCreateInfo = options.into();
        info.pool = self.pool().0;

        let mut memory_type_index = 0;
        vmaFindMemoryTypeIndexForImageInfo(
            self.allocator().0,
            image_info.as_ref(),
            &info,
            &mut memory_type_index,
        )
        .result()?;

        Ok(memory_type_index)
    }

    unsafe fn allocate_memory(
        &self,
        requirements: impl vk::Cast<Target = vk::MemoryRequirements>,
        options: &AllocationOptions,
    ) -> VkResult<Allocation> {
        let mut info: VmaAllocationCreateInfo = options.into();
        info.pool = self.pool().0;

        let mut allocation: VmaAllocation = mem::zeroed();
        vmaAllocateMemory(
            self.allocator().0,
            requirements.as_ref(),
            &info,
            &mut allocation,
            core::ptr::null_mut(),
        )
        .result()?;

        Ok(Allocation(allocation))
    }

    unsafe fn allocate_memory_pages(
        &self,
        requirements: impl vk::Cast<Target = vk::MemoryRequirements>,
        options: &AllocationOptions,
        num_pages: usize,
    ) -> VkResult<Vec<Allocation>> {
        let mut info: VmaAllocationCreateInfo = options.into();
        info.pool = self.pool().0;

        let mut allocations = Vec::with_capacity(num_pages);
        vmaAllocateMemoryPages(
            self.allocator().0,
            requirements.as_ref(),
            &info,
            num_pages,
            allocations.as_mut_ptr(),
            core::ptr::null_mut(),
        )
        .result()?;

        allocations.set_len(num_pages);

        Ok(allocations.into_iter().map(Allocation).collect())
    }

    unsafe fn allocate_memory_for_buffer(
        &self,
        buffer: vk::Buffer,
        options: &AllocationOptions,
    ) -> VkResult<Allocation> {
        let mut info: VmaAllocationCreateInfo = options.into();
        info.pool = self.pool().0;

        let mut allocation: VmaAllocation = mem::zeroed();
        vmaAllocateMemoryForBuffer(
            self.allocator().0,
            buffer,
            &info,
            &mut allocation,
            core::ptr::null_mut(),
        )
        .result()?;

        Ok(Allocation(allocation))
    }

    unsafe fn allocate_memory_for_image(
        &self,
        image: vk::Image,
        options: &AllocationOptions,
    ) -> VkResult<Allocation> {
        let mut info: VmaAllocationCreateInfo = options.into();
        info.pool = self.pool().0;

        let mut allocation: VmaAllocation = mem::zeroed();
        vmaAllocateMemoryForImage(
            self.allocator().0,
            image,
            &info,
            &mut allocation,
            core::ptr::null_mut(),
        )
        .result()?;

        Ok(Allocation(allocation))
    }

    unsafe fn create_buffer(
        &self,
        buffer_info: impl vk::Cast<Target = vk::BufferCreateInfo>,
        options: &AllocationOptions,
    ) -> VkResult<(vk::Buffer, Allocation)> {
        let mut info: VmaAllocationCreateInfo = options.into();
        info.pool = self.pool().0;

        let mut buffer = vk::Buffer::null();
        let mut allocation: VmaAllocation = mem::zeroed();
        vmaCreateBuffer(
            self.allocator().0,
            buffer_info.as_ref(),
            &info,
            &mut buffer,
            &mut allocation,
            core::ptr::null_mut(),
        )
        .result()?;

        Ok((buffer, Allocation(allocation)))
    }

    unsafe fn create_buffer_with_alignment(
        &self,
        buffer_info: impl vk::Cast<Target = vk::BufferCreateInfo>,
        options: &AllocationOptions,
        min_alignment: vk::DeviceSize,
    ) -> VkResult<(vk::Buffer, Allocation)> {
        let mut info: VmaAllocationCreateInfo = options.into();
        info.pool = self.pool().0;

        let mut buffer = vk::Buffer::null();
        let mut allocation: VmaAllocation = mem::zeroed();
        vmaCreateBufferWithAlignment(
            self.allocator().0,
            buffer_info.as_ref(),
            &info,
            min_alignment,
            &mut buffer,
            &mut allocation,
            core::ptr::null_mut(),
        )
        .result()?;

        Ok((buffer, Allocation(allocation)))
    }

    unsafe fn create_image(
        &self,
        image_info: impl vk::Cast<Target = vk::ImageCreateInfo>,
        options: &AllocationOptions,
    ) -> VkResult<(vk::Image, Allocation)> {
        let mut info: VmaAllocationCreateInfo = options.into();
        info.pool = self.pool().0;

        let mut image = vk::Image::null();
        let mut allocation: VmaAllocation = mem::zeroed();
        vmaCreateImage(
            self.allocator().0,
            image_info.as_ref(),
            &info,
            &mut image,
            &mut allocation,
            core::ptr::null_mut(),
        )
        .result()?;

        Ok((image, Allocation(allocation)))
    }
}

impl Alloc for Allocator {
    fn allocator(&self) -> &Allocator {
        self
    }

    fn pool(&self) -> PoolHandle {
        PoolHandle::DEFAULT
    }
}

impl Alloc for Pool {
    fn allocator(&self) -> &Allocator {
        &self.allocator
    }

    fn pool(&self) -> PoolHandle {
        self.handle
    }
}
