# Conclusion

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/12_graphics_pipeline_complete.rs)

We can now combine all of the structures and objects from the previous chapters to create the graphics pipeline! Here's the types of objects we have now, as a quick recap:

* Shader stages &ndash; the shader modules that define the functionality of the programmable stages of the graphics pipeline
* Fixed-function state &ndash; all of the structures that define the fixed-function stages of the pipeline, like input assembly, rasterizer, viewport and color blending
* Pipeline layout &ndash; the uniform and push values referenced by the shader that can be updated at draw time
* Render pass &ndash; the attachments referenced by the pipeline stages and their usage

All of these combined fully define the functionality of the graphics pipeline, so we can now begin filling in the `vk::GraphicsPipelineCreateInfo` structure at the end of the `create_pipeline` function (but before the shader modules are destroyed). But before the calls to `DeviceV1_0:::destroy_shader_module` because these are still to be used during the creation.

```rust,noplaypen
let stages = &[vert_stage, frag_stage];
let info = vk::GraphicsPipelineCreateInfo::builder()
    .stages(stages)
    // continued...
```

We start by providing an array of the `vk::PipelineShaderStageCreateInfo` structs.

```rust,noplaypen
    .vertex_input_state(&vertex_input_state)
    .input_assembly_state(&input_assembly_state)
    .viewport_state(&viewport_state)
    .rasterization_state(&rasterization_state)
    .multisample_state(&multisample_state)
    .color_blend_state(&color_blend_state)
```

Then we reference all of the structures describing the fixed-function stage.

```rust,noplaypen
    .layout(data.pipeline_layout)
```

After that comes the pipeline layout, which is a Vulkan handle rather than a struct reference.

```rust,noplaypen
    .render_pass(data.render_pass)
    .subpass(0);
```

And finally we have the reference to the render pass and the index of the sub pass where this graphics pipeline will be used. It is also possible to use other render passes with this pipeline instead of this specific instance, but they have to be *compatible* with `render_pass`. The requirements for compatibility are described [here](https://www.khronos.org/registry/vulkan/specs/1.2/html/vkspec.html#renderpass-compatibility), but we won't be using that feature in this tutorial.

```rust,noplaypen
    .base_pipeline_handle(vk::Pipeline::null()) // Optional.
    .base_pipeline_index(-1)                    // Optional.
```

There are actually two more parameters: `base_pipeline_handle` and `base_pipeline_index`. Vulkan allows you to create a new graphics pipeline by deriving from an existing pipeline. The idea of pipeline derivatives is that it is less expensive to set up pipelines when they have much functionality in common with an existing pipeline and switching between pipelines from the same parent can also be done quicker. You can either specify the handle of an existing pipeline with `base_pipeline_handle` or reference another pipeline that is about to be created by index with `base_pipeline_index`. Right now there is only a single pipeline, so we'll simply specify a null handle and an invalid index. These values are only used if the `vk::PipelineCreateFlags::DERIVATIVE` flag is also specified in the `flags` field of `vk::GraphicsPipelineCreateInfo`.

Now prepare for the final step by creating a field in `AppData` to hold the `vk::Pipeline` object:

```rust,noplaypen
struct AppData {
    // ...
    pipeline: vk::Pipeline,
}
```

And finally create the graphics pipeline:

```rust,noplaypen
data.pipeline = device.create_graphics_pipelines(
    vk::PipelineCache::null(), &[info], None)?.0[0];
```

The `create_graphics_pipelines` function actually has more parameters than the usual object creation functions in Vulkan. It is designed to take multiple `vk::GraphicsPipelineCreateInfo` objects and create multiple `vk::Pipeline` objects in a single call.

The first parameter, for which we've passed the `vk::PipelineCache::null()` argument, references an optional `vk::PipelineCache` object. A pipeline cache can be used to store and reuse data relevant to pipeline creation across multiple calls to `create_graphics_pipelines` and even across program executions if the cache is stored to a file. This makes it possible to significantly speed up pipeline creation at a later time.

The graphics pipeline is required for all common drawing operations, so it should also only be destroyed at the end of the program in `App::destroy`:

```rust,noplaypen
unsafe fn destroy(&mut self) {
    self.device.destroy_pipeline(self.data.pipeline, None);
    // ...
}
```

Now run your program to confirm that all this hard work has resulted in a successful pipeline creation! We are already getting quite close to seeing something pop up on the screen. In the next couple of chapters we'll set up the actual framebuffers from the swapchain images and prepare the drawing commands.
