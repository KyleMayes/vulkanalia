# Command buffers

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/14_command_buffers.rs)

Commands in Vulkan, like drawing operations and memory transfers, are not executed directly using function calls. You have to record all of the operations you want to perform in command buffer objects. The advantage of this is that all of the hard work of setting up the drawing commands can be done in advance and in multiple threads. After that, you just have to tell Vulkan to execute the commands in the main loop.

## Command pools

We have to create a command pool before we can create command buffers. Command pools manage the memory that is used to store the buffers and command buffers are allocated from them. Add a new `AppData` field to store a `vk::CommandPool`:

```rust,noplaypen
struct AppData {
    // ...
    command_pool: vk::CommandPool,
}
```

Then create a new function `^create_command_pool` and call it from `App::create` after the framebuffers were created.

```rust,noplaypen
impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        // ...
        create_framebuffers(&device, &mut data)?;
        create_command_pool(&instance, &device, &mut data)?;
        // ...
    }
}

unsafe fn create_command_pool(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    Ok(())
}
```

Command pool creation only takes two parameters:

```rust,noplaypen
let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

let info = vk::CommandPoolCreateInfo::builder()
    .flags(vk::CommandPoolCreateFlags::empty()) // Optional.
    .queue_family_index(indices.graphics);
```

Command buffers are executed by submitting them on one of the device queues, like the graphics and presentation queues we retrieved. Each command pool can only allocate command buffers that are submitted on a single type of queue. We're going to record commands for drawing, which is why we've chosen the graphics queue family.

There are three possible flags for command pools:

* `vk::CommandPoolCreateFlags::TRANSIENT` &ndash; Hint that command buffers are rerecorded with new commands very often (may change memory allocation behavior)
* `vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER` &ndash; Allow command buffers to be rerecorded individually, without this flag they all have to be reset together
* `vk::CommandPoolCreateFlags::PROTECTED` &ndash; Creates "protected" command buffers which are stored in ["protected" memory](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/html/vkspec.html#memory-protected-access-rules) where Vulkan prevents unauthorized operations from accessing the memory

We will only record the command buffers at the beginning of the program and then execute them many times in the main loop and we don't need to protect our triangle with DRM, so we're not going to use any of these flags.

```rust,noplaypen
data.command_pool = device.create_command_pool(&info, None)?;
```

Commands will be used throughout the program to draw things on the screen, so the pool should only be destroyed at the end:

```rust,noplaypen
unsafe fn destroy(&mut self) {
    self.device.destroy_command_pool(self.data.command_pool, None);
    // ...
}
```

## Command buffer allocation

We can now start allocating command buffers and recording drawing commands in them. Because one of the drawing commands involves binding the right `vk::Framebuffer`, we'll actually have to record a command buffer for every image in the swapchain once again. To that end, create a list of `vk::CommandBuffer` objects as an `AppData` field. Command buffers will be automatically freed when their command pool is destroyed, so we don't need any explicit cleanup.

```rust,noplaypen
struct AppData {
    // ...
    command_buffers: Vec<vk::CommandBuffer>,
}
```

We'll now start working on a `create_command_buffers` function that allocates and records the commands for each swapchain image.

```rust,noplaypen
impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        // ...
        create_command_pool(&instance, &device, &mut data)?;
        create_command_buffers(&device, &mut data)?;
        // ...
    }
}

unsafe fn create_command_buffers(device: &Device, data: &mut AppData) -> Result<()> {
    Ok(())
}
```

Command buffers are allocated with the `allocate_command_buffers` function, which takes a `vk::CommandBufferAllocateInfo` struct as parameter that specifies the command pool and number of buffers to allocate:

```rust,noplaypen
let allocate_info = vk::CommandBufferAllocateInfo::builder()
    .command_pool(data.command_pool)
    .level(vk::CommandBufferLevel::PRIMARY)
    .command_buffer_count(data.framebuffers.len() as u32);

data.command_buffers = device.allocate_command_buffers(&allocate_info)?;
```

The `level` parameter specifies if the allocated command buffers are primary or secondary command buffers.

* `vk::CommandBufferLevel::PRIMARY` &ndash; Can be submitted to a queue for execution, but cannot be called from other command buffers.
* `vk::CommandBufferLevel::SECONDARY` &ndash; Cannot be submitted directly, but can be called from primary command buffers.

We won't make use of the secondary command buffer functionality here, but you can imagine that it's helpful to reuse common operations from primary command buffers.

## Starting command buffer recording

We begin recording a command buffer by calling `begin_command_buffer` with a small `vk::CommandBufferBeginInfo` structure as argument that specifies some details about the usage of this specific command buffer.

```rust,noplaypen
for (i, command_buffer) in data.command_buffers.iter().enumerate() {
    let inheritance = vk::CommandBufferInheritanceInfo::builder();

    let info = vk::CommandBufferBeginInfo::builder()
        .flags(vk::CommandBufferUsageFlags::empty()) // Optional.
        .inheritance_info(&inheritance);             // Optional.

    device.begin_command_buffer(*command_buffer, &info)?;
}
```

The `flags` parameter specifies how we're going to use the command buffer. The following values are available:

* `vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT` &ndash; The command buffer will be rerecorded right after executing it once.
* `vk::CommandBufferUsageFlags::RENDER_PASS_CONTINUE` &ndash; This is a secondary command buffer that will be entirely within a single render pass.
* `vk::CommandBufferUsageFlags::SIMULTANEOUS_USE` &ndash; The command buffer can be resubmitted while it is also already pending execution.

None of these flags are applicable for us right now.

The `inheritance_info` parameter is only relevant for secondary command buffers. It specifies which state to inherit from the calling primary command buffers.

If the command buffer was already recorded once, then a call to `begin_command_buffer` will implicitly reset it. It's not possible to append commands to a buffer at a later time.

## Starting a render pass

Before we can start a render pass we'll need to build some parameters.

```rust,noplaypen
let render_area = vk::Rect2D::builder()
    .offset(vk::Offset2D::default())
    .extent(data.swapchain_extent);
```

Here we define the size of the render area. The render area defines where shader loads and stores will take place during the execution of the render pass. The pixels outside this region will have undefined values. It should match the size of the attachments for best performance.

```rust,noplaypen
let color_clear_value = vk::ClearValue {
    color: vk::ClearColorValue {
        float32: [0.0, 0.0, 0.0, 1.0],
    },
};
```

Next we define a clear value that will be used to clear the framebuffer at the beginning of the render pass (because we used `vk::AttachmentLoadOp::CLEAR` when creating the render pass). `vk::ClearValue` is a union that can be used to set clear values for color attachments or for depth/stencil attachments. Here we are setting the `color` field with a `vk::ClearColorValue` union with 4 `f32`s that define a black clear color with 100% opacity.

Drawing starts by beginning the render pass with `cmd_begin_render_pass`. The render pass is configured using some parameters in a `vk::RenderPassBeginInfo` struct.

```rust,noplaypen
let clear_values = &[color_clear_value];
let info = vk::RenderPassBeginInfo::builder()
    .render_pass(data.render_pass)
    .framebuffer(data.framebuffers[i])
    .render_area(render_area)
    .clear_values(clear_values);
```

The first parameters are the render pass itself and the attachments to bind. We created a framebuffer for each swapchain image that specifies it as color attachment. Then we provide the previously constructed render area and clear value.

```rust,noplaypen
device.cmd_begin_render_pass(
    *command_buffer, &info, vk::SubpassContents::INLINE);
```

The render pass can now begin. All of the functions that record commands can be recognized by their `cmd_` prefix. They all return `()`, so there is no need for error handling until we've finished recording.

The first parameter for every command is always the command buffer to record the command to. The second parameter specifies the details of the render pass we've just provided. The final parameter controls how the drawing commands within the render pass will be provided. It can have one of two values:

* `vk::SubpassContents::INLINE` &ndash; The render pass commands will be embedded in the primary command buffer itself and no secondary command buffers will be executed.
* `vk::SubpassContents::SECONDARY_COMMAND_BUFFERS` &ndash; The render pass commands will be executed from secondary command buffers.

We will not be using secondary command buffers, so we'll go with the first option.

## Basic drawing commands

We can now bind the graphics pipeline:

```rust,noplaypen
device.cmd_bind_pipeline(
    *command_buffer, vk::PipelineBindPoint::GRAPHICS, data.pipeline);
```

The second parameter specifies if the pipeline object is a graphics or compute pipeline. We've now told Vulkan which operations to execute in the graphics pipeline and which attachment to use in the fragment shader, so all that remains is telling it to draw the triangle:

```rust,noplaypen
device.cmd_draw(*command_buffer, 3, 1, 0, 0);
```

The actual drawing function is a bit anticlimactic, but it's so simple because of all the information we specified in advance. It has the following parameters, aside from the command buffer:

* `vertex_count` &ndash; Even though we don't have a vertex buffer, we technically still have 3 vertices to draw.
* `instance_count` &ndash; Used for instanced rendering, use `1` if you're not doing that.
* `first_vertex` &ndash; Used as an offset into the vertex buffer, defines the lowest value of `gl_VertexIndex`.
* `first_instance` &ndash; Used as an offset for instanced rendering, defines the lowest value of `gl_InstanceIndex`.

## Finishing up

The render pass can now be ended:

```rust,noplaypen
device.cmd_end_render_pass(*command_buffer);
```

And we've finished recording the command buffer:

```rust,noplaypen
device.end_command_buffer(*command_buffer)?;
```

In the next chapter we'll write the code for the main loop, which will acquire an image from the swapchain, execute the right command buffer and return the finished image to the swapchain.
