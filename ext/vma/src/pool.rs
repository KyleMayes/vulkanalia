// SPDX-License-Identifier: Apache-2.0

use alloc::sync::Arc;
use core::ffi::CStr;
use core::marker::PhantomData;
use core::mem;
use core::ptr::NonNull;

use vulkanalia::prelude::v1_0::*;
use vulkanalia::{ResultExt, VkResult};

use crate::allocator::Allocator;
use crate::flags::PoolCreateFlags;
use crate::vma::*;

#[derive(Copy, Clone, Debug)]
pub struct PoolHandle(pub(crate) VmaPool);

impl PoolHandle {
    pub const DEFAULT: Self = Self(core::ptr::null_mut());
}

unsafe impl Send for PoolHandle {}
unsafe impl Sync for PoolHandle {}

#[derive(Copy, Clone, Debug)]
pub struct PoolOptions<'a> {
    pub memory_type_index: u32,
    pub flags: PoolCreateFlags,
    pub block_size: vk::DeviceSize,
    pub min_block_count: usize,
    pub max_block_count: usize,
    pub priority: f32,
    pub min_allocation_alignment: vk::DeviceSize,
    pub next: *mut core::ffi::c_void,
    _marker: PhantomData<&'a ()>,
}

impl<'a> PoolOptions<'a> {
    pub fn push_next<T>(mut self, next: &'a mut impl vk::Cast<Target = T>) -> Self
    where
        T: vk::Extends<vk::MemoryAllocateInfo>,
    {
        self.next = vk::merge(self.next, NonNull::from(next).cast());
        self
    }
}

impl Default for PoolOptions<'_> {
    fn default() -> Self {
        Self {
            memory_type_index: 0,
            flags: PoolCreateFlags::empty(),
            block_size: 0,
            min_block_count: 0,
            max_block_count: 0,
            priority: 0.0,
            min_allocation_alignment: 0,
            next: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pool {
    pub(crate) allocator: Arc<Allocator>,
    pub(crate) handle: PoolHandle,
}

impl Allocator {
    pub fn create_pool(self: &Arc<Self>, options: &PoolOptions) -> VkResult<Pool> {
        let info = VmaPoolCreateInfo {
            memoryTypeIndex: options.memory_type_index,
            flags: options.flags.bits(),
            blockSize: options.block_size,
            minBlockCount: options.min_block_count,
            maxBlockCount: options.max_block_count,
            priority: options.priority,
            minAllocationAlignment: options.min_allocation_alignment,
            pMemoryAllocateNext: options.next,
        };

        let handle = unsafe {
            let mut pool: VmaPool = mem::zeroed();
            vmaCreatePool(self.0, &info, &mut pool).result()?;
            PoolHandle(pool)
        };

        Ok(Pool {
            allocator: self.clone(),
            handle,
        })
    }

    pub fn get_default_pool(self: &Arc<Self>) -> Pool {
        Pool {
            allocator: self.clone(),
            handle: PoolHandle::DEFAULT,
        }
    }
}

impl Pool {
    pub fn name(&self) -> Option<&CStr> {
        if self.handle.0.is_null() {
            return None;
        }

        unsafe {
            let mut name = core::ptr::null();
            vmaGetPoolName(self.allocator.0, self.handle.0, &mut name);
            if !name.is_null() {
                Some(CStr::from_ptr(name))
            } else {
                None
            }
        }
    }

    pub fn set_name(&self, name: Option<&CStr>) {
        if self.handle.0.is_null() {
            return;
        }

        let name = name.map_or(core::ptr::null(), CStr::as_ptr);
        unsafe { vmaSetPoolName(self.allocator.0, self.handle.0, name) };
    }

    pub fn get_statistics(&self) -> VkResult<VmaStatistics> {
        unsafe {
            let mut statistics: VmaStatistics = mem::zeroed();
            vmaGetPoolStatistics(self.allocator.0, self.handle.0, &mut statistics);
            Ok(statistics)
        }
    }

    pub fn calculate_statistics(&self) -> VkResult<VmaDetailedStatistics> {
        unsafe {
            let mut statistics: VmaDetailedStatistics = mem::zeroed();
            vmaCalculatePoolStatistics(self.allocator.0, self.handle.0, &mut statistics);
            Ok(statistics)
        }
    }

    pub fn check_corruption(&self) -> VkResult<()> {
        unsafe { vmaCheckPoolCorruption(self.allocator.0, self.handle.0).result() }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        unsafe { vmaDestroyPool(self.allocator.0, self.handle.0) };
        self.handle = PoolHandle::DEFAULT;
    }
}
