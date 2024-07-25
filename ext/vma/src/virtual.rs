// SPDX-License-Identifier: Apache-2.0

use core::mem;

use vulkanalia::prelude::v1_0::*;
use vulkanalia::ResultExt;

use crate::flags::{VirtualAllocationCreateFlags, VirtualBlockCreateFlags};
use crate::vma::*;

#[derive(Copy, Clone, Debug)]
pub struct VirtualBlockOptions {
    pub size: vk::DeviceSize,
    pub flags: VirtualBlockCreateFlags,
}

#[repr(transparent)]
#[derive(Debug)]
pub struct VirtualBlock(pub(crate) VmaVirtualBlock);

impl VirtualBlock {
    pub fn new(options: &VirtualBlockOptions) -> VkResult<Self> {
        let info = VmaVirtualBlockCreateInfo {
            size: options.size,
            flags: options.flags.bits(),
            pAllocationCallbacks: core::ptr::null(),
        };

        unsafe {
            let mut block: VmaVirtualBlock = mem::zeroed();
            vmaCreateVirtualBlock(&info, &mut block).result()?;
            Ok(Self(block))
        }
    }

    pub fn allocate(
        &self,
        options: &VirtualAllocationOptions,
    ) -> VkResult<(VirtualAllocation, vk::DeviceSize)> {
        let info = VmaVirtualAllocationCreateInfo {
            size: options.size,
            alignment: options.alignment,
            flags: options.flags.bits(),
            pUserData: core::ptr::null_mut(),
        };

        unsafe {
            let mut allocation: VmaVirtualAllocation = mem::zeroed();
            let mut offset = 0;
            vmaVirtualAllocate(self.0, &info, &mut allocation, &mut offset).result()?;
            Ok((VirtualAllocation(allocation), offset))
        }
    }

    pub fn free(&self, allocation: VirtualAllocation) {
        unsafe { vmaVirtualFree(self.0, allocation.0) };
    }

    pub fn clear(&self) {
        unsafe { vmaClearVirtualBlock(self.0) };
    }

    pub fn get_allocation_info(&self, allocation: VirtualAllocation) -> VmaVirtualAllocationInfo {
        unsafe {
            let mut info: VmaVirtualAllocationInfo = mem::zeroed();
            vmaGetVirtualAllocationInfo(self.0, allocation.0, &mut info);
            info
        }
    }
}

impl Drop for VirtualBlock {
    fn drop(&mut self) {
        unsafe { vmaDestroyVirtualBlock(self.0) };
        self.0 = core::ptr::null_mut();
    }
}

unsafe impl Send for VirtualBlock {}
unsafe impl Sync for VirtualBlock {}

#[derive(Copy, Clone, Debug)]
pub struct VirtualAllocationOptions {
    pub size: vk::DeviceSize,
    pub alignment: vk::DeviceSize,
    pub flags: VirtualAllocationCreateFlags,
}

#[repr(transparent)]
#[derive(Debug)]
pub struct VirtualAllocation(pub(crate) VmaVirtualAllocation);

unsafe impl Send for VirtualAllocation {}
unsafe impl Sync for VirtualAllocation {}
