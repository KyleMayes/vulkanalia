// SPDX-License-Identifier: Apache-2.0

use std::mem;

use anyhow::Result;
use cgmath::{vec2, vec4};
use examples::{App, AppData, Example, Vertex};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk;

/// The triangle vertices.
#[rustfmt::skip]
static VERTICES: [Vertex; 3] = [
    Vertex { pos: vec2(0.0, -0.5), color: vec4(1.0, 0.0, 0.0, 1.0) },
    Vertex { pos: vec2(0.5, 0.5), color: vec4(0.0, 1.0, 0.0, 1.0) },
    Vertex { pos: vec2(-0.5, 0.5), color: vec4(0.0, 0.0, 1.0, 0.0) },
];

fn main() -> Result<()> {
    unsafe { App::new("Triangle", TriangleExample)?.run()? };
    Ok(())
}

struct TriangleExampleData {
    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
}

#[derive(Default)]
struct TriangleExample;

impl Example<TriangleExampleData> for TriangleExample {
    unsafe fn setup(&mut self, instance: &Instance, device: &Device, data: &AppData) -> Result<TriangleExampleData> {
        let (vertex_buffer, vertex_buffer_memory) = examples::create_buffer(
            instance,
            device,
            data,
            mem::size_of_val(&VERTICES[..]) as u64,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        examples::fill_buffer(device, vertex_buffer, vertex_buffer_memory, &VERTICES)?;

        Ok(TriangleExampleData {
            vertex_buffer,
            vertex_buffer_memory,
        })
    }

    unsafe fn cleanup(&self, _: &Instance, device: &Device, _: &AppData, custom_data: &TriangleExampleData) {
        device.destroy_buffer(custom_data.vertex_buffer, None);
        device.free_memory(custom_data.vertex_buffer_memory, None);
    }

    unsafe fn record(
        &mut self,
        _: &Instance,
        device: &Device,
        data: &AppData,
        custom_data: &TriangleExampleData,
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
        device.cmd_bind_vertex_buffers(command_buffer, 0, &[custom_data.vertex_buffer], &[0]);
        device.cmd_draw(command_buffer, VERTICES.len() as u32, 1, 0, 0);
        device.cmd_end_render_pass(command_buffer);

        device.end_command_buffer(command_buffer)?;

        Ok(())
    }
}
