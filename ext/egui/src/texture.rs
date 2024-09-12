// SPDX-License-Identifier: Apache-2.0

//! Texture management.

use std::sync::Arc;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use egui::{TextureFilter, TextureId, TextureOptions, TextureWrapMode, TexturesDelta};
use vulkanalia::prelude::v1_0::*;
use vulkanalia_vma::{self as vma, Alloc};

struct Texture {
    id: TextureId,
    image: vk::Image,
    memory: vk::DeviceMemory,
    image_view: vk::ImageView,
}

pub struct TextureManager {
    device: Device,
    descriptor_set_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    allocator: Arc<vma::Allocator>,
    samplers: HashMap<TextureOptions, vk::Sampler>,
    textures: HashMap<TextureId, Texture>,
}

impl TextureManager {
    pub fn new(
        device: Device,
        descriptor_set_layout: vk::DescriptorSetLayout,
        descriptor_pool: vk::DescriptorPool,
        allocator: Arc<vma::Allocator>,
    ) -> Self {
        Self {
            device,
            descriptor_set_layout,
            descriptor_pool,
            allocator,
            samplers: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn update(&mut self, delta: &TexturesDelta) -> VkResult<()> {
        Ok(())
    }

    unsafe fn get_sampler(&mut self, options: TextureOptions) -> VkResult<vk::Sampler> {
        match self.samplers.entry(options) {
            Entry::Occupied(entry) => Ok(*entry.get()),
            Entry::Vacant(entry) => Ok(*entry.insert(create_sampler(&self.device, &options)?)),
        }
    }

    pub unsafe fn destroy(&mut self) {
        for sampler in self.samplers.values() {
            self.device.destroy_sampler(*sampler, None);
        }
    }
}

unsafe fn create_sampler(device: &Device, options: &TextureOptions) -> VkResult<vk::Sampler> {
    fn get_filter(filter: TextureFilter) -> vk::Filter {
        match filter {
            TextureFilter::Linear => vk::Filter::LINEAR,
            TextureFilter::Nearest => vk::Filter::NEAREST,
        }
    }

    fn get_address_mode(wrap_mode: TextureWrapMode) -> vk::SamplerAddressMode {
        match wrap_mode {
            TextureWrapMode::ClampToEdge => vk::SamplerAddressMode::CLAMP_TO_EDGE,
            TextureWrapMode::Repeat => vk::SamplerAddressMode::REPEAT,
            TextureWrapMode::MirroredRepeat => vk::SamplerAddressMode::MIRRORED_REPEAT,
        }
    }

    let mag_filter = get_filter(options.magnification);
    let min_filter = get_filter(options.minification);
    let address_mode = get_address_mode(options.wrap_mode);

    let info = vk::SamplerCreateInfo::builder()
        .mag_filter(mag_filter)
        .min_filter(min_filter)
        .address_mode_u(address_mode)
        .address_mode_v(address_mode)
        .address_mode_w(address_mode)
        .anisotropy_enable(false)
        .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
        .min_lod(0.0)
        .max_lod(vk::LOD_CLAMP_NONE);

    device.create_sampler(&info, None)
}
