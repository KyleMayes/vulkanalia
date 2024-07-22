// SPDX-License-Identifier: Apache-2.0

use bitflags::bitflags;
use vulkanalia::vk::Flags;

use crate::vma::*;

macro_rules! vmaflags {
    (
        $(#[$outer:meta])*
        pub struct $name:ident($source:ident) {
            $($(#[$inner:meta])* $variant:ident = $value:ident), *,
        }
    ) => {
        bitflags! {
            #[repr(transparent)]
            #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
            pub struct $name: Flags {
                $($(#[$inner])* const $variant = <$source>::$value as Flags;)*
            }
        }
    }
}

vmaflags! {
    pub struct AllocatorCreateFlags(VmaAllocatorCreateFlagBits) {
        EXTERNALLY_SYNCHRONIZED = EXTERNALLY_SYNCHRONIZED_BIT,
        KHR_DEDICATED_ALLOCATION = KHR_DEDICATED_ALLOCATION_BIT,
        KHR_BIND_MEMORY2 = KHR_BIND_MEMORY2_BIT,
        EXT_MEMORY_BUDGET = EXT_MEMORY_BUDGET_BIT,
        AMD_DEVICE_COHERENT_MEMORY = AMD_DEVICE_COHERENT_MEMORY_BIT,
        BUFFER_DEVICE_ADDRESS = BUFFER_DEVICE_ADDRESS_BIT,
        EXT_MEMORY_PRIORITY = EXT_MEMORY_PRIORITY_BIT,
        KHR_MAINTENANCE4 = KHR_MAINTENANCE4_BIT,
        KHR_MAINTENANCE5 = KHR_MAINTENANCE5_BIT,
    }
}

vmaflags! {
    pub struct AllocationCreateFlags(VmaAllocationCreateFlagBits) {
        DEDICATED_MEMORY = DEDICATED_MEMORY_BIT,
        NEVER_ALLOCATE = NEVER_ALLOCATE_BIT,
        MAPPED = MAPPED_BIT,
        USER_DATA_COPY_STRING = USER_DATA_COPY_STRING_BIT,
        UPPER_ADDRESS = UPPER_ADDRESS_BIT,
        DONT_BIND = DONT_BIND_BIT,
        WITHIN_BUDGET = WITHIN_BUDGET_BIT,
        CAN_ALIAS = CAN_ALIAS_BIT,
        HOST_ACCESS_SEQUENTIAL_WRITE = HOST_ACCESS_SEQUENTIAL_WRITE_BIT,
        HOST_ACCESS_RANDOM = HOST_ACCESS_RANDOM_BIT,
        HOST_ACCESS_ALLOW_TRANSFER_INSTEAD = HOST_ACCESS_ALLOW_TRANSFER_INSTEAD_BIT,
        STRATEGY_MIN_MEMORY = STRATEGY_MIN_MEMORY_BIT,
        STRATEGY_MIN_TIME = STRATEGY_MIN_TIME_BIT,
        STRATEGY_MIN_OFFSET = STRATEGY_MIN_OFFSET_BIT,
    }
}

impl AllocationCreateFlags {
    #[inline]
    pub fn strategy() -> Self {
        Self::from_bits_truncate(VmaAllocationCreateFlagBits::STRATEGY_MASK as Flags)
    }
}

vmaflags! {
    pub struct PoolCreateFlags(VmaPoolCreateFlagBits) {
        IGNORE_BUFFER_IMAGE_GRANULARITY = IGNORE_BUFFER_IMAGE_GRANULARITY_BIT,
        LINEAR_ALGORITHM = LINEAR_ALGORITHM_BIT,
    }
}

impl PoolCreateFlags {
    #[inline]
    pub fn algorithm() -> Self {
        Self::from_bits_truncate(VmaPoolCreateFlagBits::ALGORITHM_MASK as Flags)
    }
}

vmaflags! {
    pub struct VirtualAllocationCreateFlags(VmaVirtualAllocationCreateFlagBits) {
        UPPER_ADDRESS = UPPER_ADDRESS_BIT,
        STRATEGY_MIN_MEMORY = STRATEGY_MIN_MEMORY_BIT,
        STRATEGY_MIN_TIME = STRATEGY_MIN_TIME_BIT,
        STRATEGY_MIN_OFFSET = STRATEGY_MIN_OFFSET_BIT,
    }
}

impl VirtualAllocationCreateFlags {
    #[inline]
    pub fn strategy() -> Self {
        Self::from_bits_truncate(VmaVirtualAllocationCreateFlagBits::STRATEGY_MASK as Flags)
    }
}

vmaflags! {
    pub struct VirtualBlockCreateFlags(VmaVirtualBlockCreateFlagBits) {
        LINEAR_ALGORITHM = LINEAR_ALGORITHM_BIT,
    }
}

impl VirtualBlockCreateFlags {
    #[inline]
    pub fn algorithm() -> Self {
        Self::from_bits_truncate(VmaVirtualBlockCreateFlagBits::ALGORITHM_MASK as Flags)
    }
}

vmaflags! {
    pub struct DefragmentationFlags(VmaDefragmentationFlagBits) {
        ALGORITHM_FAST = ALGORITHM_FAST_BIT,
        ALGORITHM_BALANCED = ALGORITHM_BALANCED_BIT,
        ALGORITHM_FULL = ALGORITHM_FULL_BIT,
        ALGORITHM_EXTENSIVE = ALGORITHM_EXTENSIVE_BIT,
    }
}

impl DefragmentationFlags {
    #[inline]
    pub fn algorithm() -> Self {
        Self::from_bits_truncate(VmaDefragmentationFlagBits::ALGORITHM_MASK as Flags)
    }
}
