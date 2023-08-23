// SPDX-License-Identifier: Apache-2.0

use std::mem;

use anyhow::Result;
use examples::{App, AppData, Example};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk;

/// The triangle vertices.
#[rustfmt::skip]
static VERTICES: [Vertex; 3] = [
    Vertex { pos: cgmath::vec2(0.0, -0.5), color: cgmath::vec3(1.0, 0.0, 0.0) },
    Vertex { pos: cgmath::vec2(0.5, 0.5), color: cgmath::vec3(0.0, 1.0, 0.0) },
    Vertex { pos: cgmath::vec2(-0.5, 0.5), color: cgmath::vec3(0.0, 0.0, 1.0) },
];

fn main() -> Result<()> {
    unsafe { App::new("Triangle", TriangleExample::default())?.run() }
}

#[derive(Copy, Clone, Debug, Default)]
struct TriangleExample {
    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
}

impl Example for TriangleExample {
    unsafe fn create_render_pass(&self, device: &Device, data: &mut AppData) -> Result<()> {
        // Attachments

        let color_attachment = vk::AttachmentDescription::builder()
            .format(data.swapchain_format)
            .samples(vk::SampleCountFlags::_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

        // Subpasses

        let color_attachment_ref = vk::AttachmentReference::builder()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

        let color_attachments = &[color_attachment_ref];
        let subpass = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(color_attachments);

        // Dependencies

        let dependency = vk::SubpassDependency::builder()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);

        // Create

        let attachments = &[color_attachment];
        let subpasses = &[subpass];
        let dependencies = &[dependency];
        let info = vk::RenderPassCreateInfo::builder()
            .attachments(attachments)
            .subpasses(subpasses)
            .dependencies(dependencies);

        data.render_pass = device.create_render_pass(&info, None)?;

        Ok(())
    }

    unsafe fn create_pipeline(&self, device: &Device, data: &mut AppData) -> Result<()> {
        // Stages

        let vert = include_bytes!("../../shaders/triangle/vert.spv");
        let frag = include_bytes!("../../shaders/triangle/frag.spv");

        let vert_shader_module = examples::create_shader_module(device, vert)?;
        let frag_shader_module = examples::create_shader_module(device, frag)?;

        let vert_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_shader_module)
            .name(b"main\0");

        let frag_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_shader_module)
            .name(b"main\0");

        // Vertex Input State

        let binding_descriptions = &[Vertex::binding_description()];
        let attribute_descriptions = &Vertex::attribute_descriptions();
        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(binding_descriptions)
            .vertex_attribute_descriptions(attribute_descriptions);

        // Input Assembly State

        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        // Viewport State

        let viewport = vk::Viewport::builder()
            .x(0.0)
            .y(0.0)
            .width(data.swapchain_extent.width as f32)
            .height(data.swapchain_extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0);

        let scissor = vk::Rect2D::builder()
            .offset(vk::Offset2D::default())
            .extent(data.swapchain_extent);

        let viewports = &[viewport];
        let scissors = &[scissor];
        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(viewports)
            .scissors(scissors);

        // Rasterization State

        let rasterization_state = vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(true);

        // Multisample State

        let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::_1);

        // Color Blend State

        let attachment = vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(vk::ColorComponentFlags::all())
            .blend_enable(false);

        let attachments = &[attachment];
        let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::COPY)
            .attachments(attachments)
            .blend_constants([0.0, 0.0, 0.0, 0.0]);

        // Layout

        let info = vk::PipelineLayoutCreateInfo::builder();
        data.pipeline_layout = device.create_pipeline_layout(&info, None)?;

        // Create

        let stages = &[vert_stage, frag_stage];
        let info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(stages)
            .vertex_input_state(&vertex_input_state)
            .input_assembly_state(&input_assembly_state)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterization_state)
            .multisample_state(&multisample_state)
            .color_blend_state(&color_blend_state)
            .layout(data.pipeline_layout)
            .render_pass(data.render_pass)
            .subpass(0)
            .base_pipeline_handle(vk::Pipeline::null());

        data.pipeline = device
            .create_graphics_pipelines(vk::PipelineCache::null(), &[info], None)?
            .0[0];

        // Cleanup

        device.destroy_shader_module(vert_shader_module, None);
        device.destroy_shader_module(frag_shader_module, None);

        Ok(())
    }

    unsafe fn record_commands(&mut self, instance: &Instance, device: &Device, data: &AppData) -> Result<()> {
        // Vertex Buffer

        if self.vertex_buffer.is_null() {
            let (buffer, memory) = examples::create_buffer(
                instance,
                device,
                data,
                mem::size_of_val(&VERTICES[..]) as u64,
                vk::BufferUsageFlags::VERTEX_BUFFER,
                vk::MemoryPropertyFlags::HOST_COHERENT,
            )?;

            examples::fill_buffer(device, buffer, memory, &VERTICES)?;

            self.vertex_buffer = buffer;
            self.vertex_buffer_memory = memory;
        }

        // Commands

        for (index, command_buffer) in data.command_buffers.iter().enumerate() {
            device.begin_command_buffer(*command_buffer, &vk::CommandBufferBeginInfo::default())?;

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
                .framebuffer(data.framebuffers[index])
                .render_area(render_area)
                .clear_values(clear_values);

            device.cmd_begin_render_pass(*command_buffer, &render_pass_begin, vk::SubpassContents::INLINE);
            device.cmd_bind_pipeline(*command_buffer, vk::PipelineBindPoint::GRAPHICS, data.pipeline);
            device.cmd_bind_vertex_buffers(*command_buffer, 0, &[self.vertex_buffer], &[0]);
            device.cmd_draw(*command_buffer, VERTICES.len() as u32, 1, 0, 0);
            device.cmd_end_render_pass(*command_buffer);

            device.end_command_buffer(*command_buffer)?;
        }

        Ok(())
    }

    unsafe fn destroy(&self, device: &Device) {
        device.destroy_buffer(self.vertex_buffer, None);
        device.free_memory(self.vertex_buffer_memory, None)
    }
}

/// A triangle vertex.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    pos: cgmath::Vector2<f32>,
    color: cgmath::Vector3<f32>,
}

impl Vertex {
    /// Gets the binding description for a triangle vertex.
    fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(mem::size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    /// Gets the attribute descriptions for a triangle vertex.
    fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        let pos = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(0)
            .build();
        let color = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(mem::size_of::<cgmath::Vector2<f32>>() as u32)
            .build();
        [pos, color]
    }
}
