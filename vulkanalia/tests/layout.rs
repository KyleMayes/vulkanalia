// SPDX-License-Identifier: Apache-2.0

use std::mem::{offset_of, size_of};

use vulkanalia::vk;

#[test]
fn acceleration_structure_instance_matches_the_vulkan_abi() {
    type Instance = vk::AccelerationStructureInstanceKHR;

    assert_eq!(size_of::<Instance>(), 64);
    assert_eq!(offset_of!(Instance, transform), 0);
    assert_eq!(offset_of!(Instance, bitfields0), 48);
    assert_eq!(offset_of!(Instance, bitfields1), 52);
    assert_eq!(offset_of!(Instance, acceleration_structure_reference), 56);
}
