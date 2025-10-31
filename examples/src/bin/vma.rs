// SPDX-License-Identifier: Apache-2.0

#![allow(unsafe_op_in_unsafe_fn)]

use std::mem;
use std::ptr::copy_nonoverlapping as memcpy;

use anyhow::Result;
use cgmath::{vec2, vec4};
use examples::{App, AppData, Example, Vertex};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk;
use vulkanalia_vma::{self as vma, Alloc};

/// The triangle vertices.
#[rustfmt::skip]
static VERTICES: [Vertex; 3] = [
    Vertex { pos: vec2(0.0, -0.5), color: vec4(1.0, 0.0, 0.0, 1.0) },
    Vertex { pos: vec2(0.5, 0.5), color: vec4(0.0, 1.0, 0.0, 1.0) },
    Vertex { pos: vec2(-0.5, 0.5), color: vec4(0.0, 0.0, 1.0, 0.0) },
];

fn main() -> Result<()> {
    unsafe { App::new("VMA", VmaExample)?.run()? };
    Ok(())
}

struct VmaExampleData {
    allocator: vma::Allocator,
    vertex_buffer: (vk::Buffer, vma::Allocation),
}

#[derive(Default)]
struct VmaExample;

impl Example<VmaExampleData> for VmaExample {
    unsafe fn setup(&mut self, instance: &Instance, device: &Device, data: &AppData) -> Result<VmaExampleData> {
        let allocator_options = vma::AllocatorOptions::new(instance, device, data.physical_device);
        let allocator = vma::Allocator::new(&allocator_options)?;

        let (vertex_buffer, vertex_buffer_allocation) = allocator.create_buffer(
            vk::BufferCreateInfo::builder()
                .size(mem::size_of_val(&VERTICES[..]) as u64)
                .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
                .sharing_mode(vk::SharingMode::EXCLUSIVE),
            &vma::AllocationOptions {
                flags: vma::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE,
                ..Default::default()
            },
        )?;

        let dst = allocator.map_memory(vertex_buffer_allocation)?;
        memcpy(VERTICES.as_ptr(), dst.cast(), VERTICES.len());
        allocator.unmap_memory(vertex_buffer_allocation);

        Ok(VmaExampleData {
            allocator,
            vertex_buffer: (vertex_buffer, vertex_buffer_allocation),
        })
    }

    unsafe fn cleanup(&self, _: &Instance, _: &Device, _: &AppData, custom_data: &VmaExampleData) {
        custom_data
            .allocator
            .destroy_buffer(custom_data.vertex_buffer.0, custom_data.vertex_buffer.1);
    }

    unsafe fn record(
        &mut self,
        _: &Instance,
        device: &Device,
        data: &AppData,
        custom_data: &VmaExampleData,
        framebuffer: vk::Framebuffer,
        command_buffer: vk::CommandBuffer,
    ) -> Result<()> {
        device.begin_command_buffer(command_buffer, &vk::CommandBufferBeginInfo::default())?;

        let clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.4, 0.6, 0.9, 0.0],
            },
        };

        let render_area = vk::Rect2D::builder()
            .offset(vk::Offset2D::default())
            .extent(data.swapchain_extent);

        let clear_values = &[clear_value];
        let render_pass_begin = vk::RenderPassBeginInfo::builder()
            .render_pass(data.render_pass)
            .framebuffer(framebuffer)
            .render_area(render_area)
            .clear_values(clear_values);

        device.cmd_begin_render_pass(command_buffer, &render_pass_begin, vk::SubpassContents::INLINE);
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, data.pipeline);
        device.cmd_bind_vertex_buffers(command_buffer, 0, &[custom_data.vertex_buffer.0], &[0]);
        device.cmd_draw(command_buffer, VERTICES.len() as u32, 1, 0, 0);
        device.cmd_end_render_pass(command_buffer);

        device.end_command_buffer(command_buffer)?;

        Ok(())
    }
}
