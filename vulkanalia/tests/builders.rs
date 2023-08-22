extern crate vulkanalia;

use vulkanalia::chain::{input_chain, output_chain};
use vulkanalia::prelude::v1_0::*;

#[test]
fn test_push_next_one() {
    let mut vk1_1 = vk::PhysicalDeviceVulkan11Features::default();

    let features = vk::PhysicalDeviceFeatures2::builder()
        .push_next(&mut vk1_1)
        .build();

    let chain = unsafe { output_chain(features.next) }.collect::<Vec<_>>();
    assert_eq!(chain.len(), 1);

    assert_eq!(unsafe { chain[0].as_base_ref() }.s_type, vk1_1.s_type);
}

#[test]
fn test_push_next_many() {
    let mut vk1_1 = vk::PhysicalDeviceVulkan11Features::default();
    let mut vk1_2 = vk::PhysicalDeviceVulkan12Features::default();
    let mut vk1_3 = vk::PhysicalDeviceVulkan13Features::default();

    let features = vk::PhysicalDeviceFeatures2::builder()
        .push_next(&mut vk1_1)
        .push_next(&mut vk1_2)
        .push_next(&mut vk1_3)
        .build();

    let mut chain = unsafe { output_chain(features.next) }.collect::<Vec<_>>();
    chain.sort_by_key(|p| unsafe { p.as_base_ref() }.s_type);
    assert_eq!(chain.len(), 3);

    assert_eq!(unsafe { chain[0].as_base_ref() }.s_type, vk1_1.s_type);
    assert_eq!(unsafe { chain[1].as_base_ref() }.s_type, vk1_2.s_type);
    assert_eq!(unsafe { chain[2].as_base_ref() }.s_type, vk1_3.s_type);
}

#[test]
fn test_push_next_subchain() {
    let mut vk1_1 = vk::PhysicalDeviceVulkan11Features::default();
    let mut vk1_2 = vk::PhysicalDeviceVulkan12Features::default();
    let mut vk1_3 = vk::PhysicalDeviceVulkan13Features::default();

    let mut features = vk::PhysicalDeviceFeatures2::builder()
        .push_next(&mut vk1_1)
        .push_next(&mut vk1_2)
        .push_next(&mut vk1_3)
        .build();

    let info = vk::DeviceCreateInfo::builder()
        .push_next(&mut features)
        .build();

    let mut chain = unsafe { input_chain(info.next) }.collect::<Vec<_>>();
    chain.sort_by_key(|p| unsafe { p.as_base_ref() }.s_type);
    assert_eq!(chain.len(), 4);

    assert_eq!(unsafe { chain[0].as_base_ref() }.s_type, vk1_1.s_type);
    assert_eq!(unsafe { chain[1].as_base_ref() }.s_type, vk1_2.s_type);
    assert_eq!(unsafe { chain[2].as_base_ref() }.s_type, vk1_3.s_type);
    assert_eq!(unsafe { chain[3].as_base_ref() }.s_type, features.s_type);
}
