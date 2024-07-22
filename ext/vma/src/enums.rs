// SPDX-License-Identifier: Apache-2.0

use crate::vma::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum MemoryUsage {
    #[default]
    Unknown,
    Auto,
    AutoPreferDevice,
    AutoPreferHost,
    GpuLazilyAllocated,
}

impl From<MemoryUsage> for VmaMemoryUsage {
    fn from(value: MemoryUsage) -> Self {
        use MemoryUsage::*;
        match value {
            Unknown => VmaMemoryUsage::UNKNOWN,
            Auto => VmaMemoryUsage::AUTO,
            AutoPreferDevice => VmaMemoryUsage::AUTO_PREFER_DEVICE,
            AutoPreferHost => VmaMemoryUsage::AUTO_PREFER_HOST,
            GpuLazilyAllocated => VmaMemoryUsage::GPU_LAZILY_ALLOCATED,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DefragmentationMoveOperation {
    Copy,
    Ignore,
    Destroy,
}

#[allow(deprecated)]
impl From<DefragmentationMoveOperation> for VmaDefragmentationMoveOperation {
    fn from(value: DefragmentationMoveOperation) -> Self {
        use DefragmentationMoveOperation::*;
        match value {
            Copy => VmaDefragmentationMoveOperation::COPY,
            Ignore => VmaDefragmentationMoveOperation::IGNORE,
            Destroy => VmaDefragmentationMoveOperation::DESTROY,
        }
    }
}
