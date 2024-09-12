// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use std::collections::HashMap;
use std::mem;
use std::ptr::copy_nonoverlapping as memcpy;

use anyhow::Result;
use examples::{App, AppData, Example, Vertex};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk;
use vulkanalia_vma::{self as vma, Alloc};
use winit::event::Event;
use winit::event_loop::EventLoop;
use winit::window::Window;

fn main() -> Result<()> {
    unsafe { App::new("EGUI", EguiExample::new())?.run()? };
    Ok(())
}

struct EguiExampleData {
    descriptor_set_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    sampler: vk::Sampler,
    allocator: Arc<vma::Allocator>,
    buffers: HashMap<u32, Vec<(vk::Buffer, vma::Allocation)>>,
    textures: HashMap<egui::TextureId, (vk::Image, vma::Allocation, vk::ImageView)>,
    texture_manager: vulkanalia_egui::texture::TextureManager,
}

struct EguiExample {
    context: egui::Context,
    state: Option<egui_winit::State>,
}

impl EguiExample {
    fn new() -> Self {
        Self {
            context: egui::Context::default(),
            state: None,
        }
    }
}

impl Example<EguiExampleData> for EguiExample {
    fn vert(&self) -> &[u8] {
        include_bytes!("../../shaders/egui/vert.spv")
    }

    fn frag(&self) -> &[u8] {
        include_bytes!("../../shaders/egui/frag.spv")
    }

    fn create_window(&mut self, title: &str, event_loop: &EventLoop<()>) -> Result<Window> {
        let window = egui_winit::create_window(
            &self.context,
            event_loop,
            &egui::ViewportBuilder {
                title: Some(title.into()),
                inner_size: Some(egui::Vec2::new(800.0, 600.0)),
                ..Default::default()
            },
        )?;

        self.state = Some(egui_winit::State::new(
            self.context.clone(),
            egui::ViewportId::ROOT,
            event_loop,
            Some(window.scale_factor() as f32),
            None,
        ));

        Ok(window)
    }

    fn consume_event(&mut self, window: &Window, event: &Event<()>) -> bool {
        if let Event::WindowEvent { event, .. } = event {
            self.state.as_mut().unwrap().on_window_event(window, event).consumed
        } else {
            false
        }
    }

    unsafe fn create_pipeline(&self, device: &Device, data: &mut AppData) -> Result<()> {
        // Stages

        let vert_shader_module = examples::create_shader_module(device, self.vert())?;
        let frag_shader_module = examples::create_shader_module(device, self.frag())?;

        let vert_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_shader_module)
            .name(b"main\0");

        let frag_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_shader_module)
            .name(b"main\0");

        // Vertex Input State

        let binding_description = vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(mem::size_of::<egui::epaint::Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX);

        let pos = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(0);
        let uv = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(mem::size_of::<egui::Pos2>() as u32);
        let color = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(2)
            .format(vk::Format::R8G8B8A8_UNORM)
            .offset((2 * mem::size_of::<egui::Pos2>()) as u32);

        let binding_descriptions = &[binding_description];
        let attribute_descriptions = &[pos, uv, color];
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
            .cull_mode(vk::CullModeFlags::NONE)
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

        // Push Constant Ranges

        let vert_push_constant_range = vk::PushConstantRange::builder()
            .stage_flags(vk::ShaderStageFlags::VERTEX)
            .offset(0)
            .size((2 * mem::size_of::<f32>()) as u32);

        // Layout

        let push_constant_ranges = &[vert_push_constant_range];
        let layout_info = vk::PipelineLayoutCreateInfo::builder().push_constant_ranges(push_constant_ranges);

        data.pipeline_layout = device.create_pipeline_layout(&layout_info, None)?;

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

    unsafe fn setup(&mut self, instance: &Instance, device: &Device, data: &AppData) -> Result<EguiExampleData> {
        let sampler_binding = vk::DescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT);

        let bindings = &[sampler_binding];
        let info = vk::DescriptorSetLayoutCreateInfo::builder().bindings(bindings);

        let descriptor_set_layout = device.create_descriptor_set_layout(&info, None)?;

        let sampler_size = vk::DescriptorPoolSize::builder()
            .type_(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(data.swapchain_images.len() as u32);

        let pool_sizes = &[sampler_size];
        let info = vk::DescriptorPoolCreateInfo::builder()
            .pool_sizes(pool_sizes)
            .max_sets(data.swapchain_images.len() as u32);

        let descriptor_pool = device.create_descriptor_pool(&info, None)?;

        let info = vk::SamplerCreateInfo::builder()
            .mag_filter(vk::Filter::LINEAR)
            .min_filter(vk::Filter::LINEAR)
            .address_mode_u(vk::SamplerAddressMode::REPEAT)
            .address_mode_v(vk::SamplerAddressMode::REPEAT)
            .address_mode_w(vk::SamplerAddressMode::REPEAT)
            .anisotropy_enable(true)
            .max_anisotropy(16.0)
            .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(vk::CompareOp::ALWAYS)
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .min_lod(0.0)
            .max_lod(vk::LOD_CLAMP_NONE)
            .mip_lod_bias(0.0);

        let sampler = device.create_sampler(&info, None)?;

        let allocator_options = vma::AllocatorOptions::new(instance, device, data.physical_device);
        let allocator = Arc::new(vma::Allocator::new(&allocator_options)?);

        Ok(EguiExampleData {
            descriptor_set_layout,
            descriptor_pool,
            sampler,
            allocator: allocator.clone(),
            buffers: HashMap::new(),
            textures: HashMap::new(),
            texture_manager: vulkanalia_egui::texture::TextureManager::new(
                device.clone(),
                descriptor_set_layout,
                descriptor_pool,
                allocator,
            ),
        })
    }

    unsafe fn cleanup(&self, _: &Instance, device: &Device, _: &AppData, mut custom_data: EguiExampleData) {
        custom_data.texture_manager.destroy();

        for (buffer, allocation) in custom_data.buffers.values().flatten() {
            custom_data.allocator.destroy_buffer(*buffer, *allocation);
        }

        for (image, allocation, image_view) in custom_data.textures.values() {
            custom_data.allocator.destroy_image(*image, *allocation);
            device.destroy_image_view(*image_view, None);
        }

        device.destroy_sampler(custom_data.sampler, None);
        device.destroy_descriptor_pool(custom_data.descriptor_pool, None);
        device.destroy_descriptor_set_layout(custom_data.descriptor_set_layout, None);
    }

    unsafe fn record(
        &mut self,
        window: &Window,
        _: &Instance,
        device: &Device,
        data: &AppData,
        custom_data: &mut EguiExampleData,
        frame_index: u32,
        framebuffer: vk::Framebuffer,
        command_buffer: vk::CommandBuffer,
    ) -> Result<()> {
        let state = self.state.as_mut().unwrap();
        let raw_input = state.take_egui_input(window);

        let full_output = self.context.run(raw_input, |context| {
            egui::CentralPanel::default().show(context, |ui| {
                ui.heading("egui + vulkanalia");
                if ui.button("Click Me!").clicked() {
                    log::info!("Clicked.");
                }
            });
        });

        custom_data.texture_manager.update(&full_output.textures_delta)?;

        for (id, delta) in full_output.textures_delta.set {
            let font = if let egui::ImageData::Font(font) = delta.image {
                font
            } else {
                continue;
            };

            let options = vma::AllocationOptions {
                flags: vma::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE,
                ..Default::default()
            };

            let texture = custom_data.allocator.create_image(
                vk::ImageCreateInfo::builder()
                    .image_type(vk::ImageType::_2D)
                    .extent(vk::Extent3D {
                        width: font.width() as u32,
                        height: font.height() as u32,
                        depth: 1,
                    })
                    .mip_levels(1)
                    .array_layers(1)
                    .format(vk::Format::R8G8B8A8_UNORM)
                    .tiling(vk::ImageTiling::LINEAR)
                    .initial_layout(vk::ImageLayout::UNDEFINED)
                    .usage(vk::ImageUsageFlags::SAMPLED)
                    .sharing_mode(vk::SharingMode::EXCLUSIVE)
                    .samples(vk::SampleCountFlags::_1),
                &options,
            )?;

            let pixels = font.srgba_pixels(None).collect::<Vec<_>>();

            let bytes = custom_data.allocator.map_memory(texture.1)?;
            memcpy(pixels.as_ptr(), bytes.cast(), pixels.len());
            custom_data.allocator.unmap_memory(texture.1);

            let subresource_range = vk::ImageSubresourceRange::builder()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1);

            let info = vk::ImageViewCreateInfo::builder()
                .image(texture.0)
                .view_type(vk::ImageViewType::_2D)
                .format(vk::Format::R8G8B8A8_UNORM)
                .subresource_range(subresource_range);

            let image_view = device.create_image_view(&info, None)?;

            custom_data.textures.insert(id, (texture.0, texture.1, image_view));
        }

        state.handle_platform_output(window, full_output.platform_output);

        let primitives = self
            .context
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        let buffers = custom_data.buffers.entry(frame_index).or_default();
        for (buffer, allocation) in buffers.drain(..) {
            custom_data.allocator.destroy_buffer(buffer, allocation);
        }

        let mut meshes: Vec<(vk::Buffer, vk::Buffer, u32)> = vec![];
        for primitive in primitives {
            use egui::epaint::Primitive::*;
            match primitive.primitive {
                Mesh(mesh) => {
                    let options = vma::AllocationOptions {
                        flags: vma::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE,
                        ..Default::default()
                    };

                    let vertices = custom_data.allocator.create_buffer(
                        vk::BufferCreateInfo::builder()
                            .size((mem::size_of::<Vertex>() * mesh.vertices.len()) as u64)
                            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
                            .sharing_mode(vk::SharingMode::EXCLUSIVE),
                        &options,
                    )?;

                    let bytes = custom_data.allocator.map_memory(vertices.1)?;
                    memcpy(mesh.vertices.as_ptr(), bytes.cast(), mesh.vertices.len());
                    custom_data.allocator.unmap_memory(vertices.1);

                    let indices = custom_data.allocator.create_buffer(
                        vk::BufferCreateInfo::builder()
                            .size((mem::size_of::<u32>() * mesh.indices.len()) as u64)
                            .usage(vk::BufferUsageFlags::INDEX_BUFFER)
                            .sharing_mode(vk::SharingMode::EXCLUSIVE),
                        &options,
                    )?;

                    let bytes = custom_data.allocator.map_memory(indices.1)?;
                    memcpy(mesh.indices.as_ptr(), bytes.cast(), mesh.indices.len());
                    custom_data.allocator.unmap_memory(indices.1);

                    buffers.push(vertices);
                    buffers.push(indices);
                    meshes.push((vertices.0, indices.0, mesh.indices.len() as u32));
                }
                _ => {}
            }
        }

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

        let screen = &[data.swapchain_extent.width as f32, data.swapchain_extent.height as f32];
        device.cmd_push_constants(
            command_buffer,
            data.pipeline_layout,
            vk::ShaderStageFlags::VERTEX,
            0,
            screen.as_slice().align_to::<u8>().1,
        );

        for (vertex_buffer, index_buffer, num_indices) in meshes {
            device.cmd_bind_vertex_buffers(command_buffer, 0, &[vertex_buffer], &[0]);
            device.cmd_bind_index_buffer(command_buffer, index_buffer, 0, vk::IndexType::UINT32);
            device.cmd_draw_indexed(command_buffer, num_indices, 1, 0, 0, 0);
        }

        device.cmd_end_render_pass(command_buffer);

        device.end_command_buffer(command_buffer)?;

        Ok(())
    }
}
