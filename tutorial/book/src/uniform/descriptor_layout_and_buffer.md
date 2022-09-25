# Descriptor layout and buffer

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/21_descriptor_layout.rs) | [shader.vert](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/shaders/21/shader.vert) | [shader.frag](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/shaders/21/shader.frag)

We're now able to pass arbitrary attributes to the vertex shader for each vertex, but what about global variables? We're going to move on to 3D graphics from this chapter on and that requires a model-view-projection matrix. We could include it as vertex data, but that's a waste of memory and it would require us to update the vertex buffer whenever the transformation changes. The transformation could easily change every single frame.

The right way to tackle this in Vulkan is to use *resource descriptors*. A descriptor is a way for shaders to freely access resources like buffers and images. We're going to set up a buffer that contains the transformation matrices and have the vertex shader access them through a descriptor. Usage of descriptors consists of three parts:

* Specify a descriptor layout during pipeline creation
* Allocate a descriptor set from a descriptor pool
* Bind the descriptor set during rendering

The *descriptor layout* specifies the types of resources that are going to be accessed by the pipeline, just like a render pass specifies the types of attachments that will be accessed. A *descriptor set* specifies the actual buffer or image resources that will be bound to the descriptors, just like a framebuffer specifies the actual image views to bind to render pass attachments. The descriptor set is then bound for the drawing commands just like the vertex buffers and framebuffer.

There are many types of descriptors, but in this chapter we'll work with uniform buffer objects (UBO). We'll look at other types of descriptors in future chapters, but the basic process is the same. Let's say we have the data we want the vertex shader to have in a struct like this:

```rust,noplaypen
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct UniformBufferObject {
    model: glm::Mat4,
    view: glm::Mat4,
    proj: glm::Mat4,
}
```

Then we can copy the data to a `vk::Buffer` and access it through a uniform buffer object descriptor from the vertex shader like this:

```glsl
layout(binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

// ...

void main() {
    gl_Position = ubo.proj * ubo.view * ubo.model * vec4(inPosition, 0.0, 1.0);
    fragColor = inColor;
}
```

We're going to update the model, view and projection matrices every frame to make the rectangle from the previous chapter spin around in 3D.

## Vertex shader

Modify the vertex shader to include the uniform buffer object like it was specified above. I will assume that you are familiar with MVP transformations. If you're not, see [the resource](https://www.opengl-tutorial.org/beginners-tutorials/tutorial-3-matrices/) mentioned in the first chapter.

```glsl
#version 450

layout(binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) in vec2 inPosition;
layout(location = 1) in vec3 inColor;

layout(location = 0) out vec3 fragColor;

void main() {
    gl_Position = ubo.proj * ubo.view * ubo.model * vec4(inPosition, 0.0, 1.0);
    fragColor = inColor;
}
```

Note that the order of the `uniform`, `in` and `out` declarations doesn't matter. The `binding` directive is similar to the `location` directive for attributes. We're going to reference this binding in the descriptor layout. The line with `gl_Position` is changed to use the transformations to compute the final position in clip coordinates. Unlike the 2D triangles, the last component of the clip coordinates may not be `1`, which will result in a division when converted to the final normalized device coordinates on the screen. This is used in perspective projection as the *perspective division* and is essential for making closer objects look larger than objects that are further away.

## Descriptor set layout

The next step is to define the UBO on the Rust side and to tell Vulkan about this descriptor in the vertex shader.

```rust,noplaypen
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct UniformBufferObject {
    model: glm::Mat4,
    view: glm::Mat4,
    proj: glm::Mat4,
}
```

We can exactly match the definition in the shader using data types in GLM. The data in the matrices is binary compatible with the way the shader expects it, so we can later just copy a `UniformBufferObject` to a `vk::Buffer`.

We need to provide details about every descriptor binding used in the shaders for pipeline creation, just like we had to do for every vertex attribute and its `location` index. We'll set up a new function to define all of this information called `^create_descriptor_set_layout`. It should be called right before pipeline creation, because we're going to need it there.

```rust,noplaypen
impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        // ...
        create_descriptor_set_layout(&device, &mut data)?;
        create_pipeline(&device, &mut data)?;
        // ...
    }
}


unsafe fn create_descriptor_set_layout(
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    Ok(())
}
```

Every binding needs to be described through a `vk::DescriptorSetLayoutBinding` struct.

```rust,noplaypen
unsafe fn create_descriptor_set_layout(
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    let ubo_binding = vk::DescriptorSetLayoutBinding::builder()
        .binding(0)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::VERTEX);

    Ok(())
}
```

The first two fields specify the `binding` used in the shader and the type of descriptor, which is a uniform buffer object. It is possible for the shader variable to represent an array of uniform buffer objects, and `descriptor_count` specifies the number of values in the array. This could be used to specify a transformation for each of the bones in a skeleton for skeletal animation, for example. Our MVP transformation is in a single uniform buffer object, so we're using a `descriptor_count` of `1`.

We also need to specify in which shader stages the descriptor is going to be referenced. The `stage_flags` field can be a combination of `vk::ShaderStageFlags` values or the value `vk::ShaderStageFlags::ALL_GRAPHICS`. In our case, we're only referencing the descriptor from the vertex shader.

There is also an `immutable_samplers` field which is only relevant for image sampling related descriptors, which we'll look at later. You can leave this to its default value.

All of the descriptor bindings are combined into a single `vk::DescriptorSetLayout` object. Define a new `AppData` field above `pipeline_layout`:

```rust,noplaypen
struct AppData {
    // ...
    descriptor_set_layout: vk::DescriptorSetLayout,
    pipeline_layout: vk::PipelineLayout,
    // ...
}
```

We can then create it using `create_descriptor_set_layout`. This function accepts a simple `vk::DescriptorSetLayoutCreateInfo` with the array of bindings:

```rust,noplaypen
let bindings = &[ubo_binding];
let info = vk::DescriptorSetLayoutCreateInfo::builder()
    .bindings(bindings);

data.descriptor_set_layout = device.create_descriptor_set_layout(&info, None)?;
```

We need to specify the descriptor set layout during pipeline creation to tell Vulkan which descriptors the shaders will be using. Descriptor set layouts are specified in the pipeline layout object. Modify the `vk::PipelineLayoutCreateInfo` to reference the layout object:

```rust,noplaypen
let set_layouts = &[data.descriptor_set_layout];
let layout_info = vk::PipelineLayoutCreateInfo::builder()
    .set_layouts(set_layouts);
```

You may be wondering why it's possible to specify multiple descriptor set layouts here, because a single one already includes all of the bindings. We'll get back to that in the next chapter, where we'll look into descriptor pools and descriptor sets.

The descriptor layout should stick around while we may create new graphics pipelines i.e. until the program ends:

```rust,noplaypen
unsafe fn destroy(&mut self) {
    self.destroy_swapchain();
    self.device.destroy_descriptor_set_layout(self.data.descriptor_set_layout, None);
    // ...
}
```

## Uniform buffer

In the next chapter we'll specify the buffer that contains the UBO data for the shader, but we need to create this buffer first. We're going to copy new data to the uniform buffer every frame, so it doesn't really make any sense to have a staging buffer. It would just add extra overhead in this case and likely degrade performance instead of improving it.

We should have multiple buffers, because multiple frames may be in flight at the same time and we don't want to update the buffer in preparation of the next frame while a previous one is still reading from it! We could either have a uniform buffer per frame or per swapchain image. However, since we need to refer to the uniform buffer from the command buffer that we have per swapchain image, it makes the most sense to also have a uniform buffer per swapchain image.

To that end, add new `AppData` fields for `uniform_buffers`, and `uniform_buffers_memory`:

```rust,noplaypen
struct AppData {
    // ...
    index_buffer: vk::Buffer,
    index_buffer_memory: vk::DeviceMemory,
    uniform_buffers: Vec<vk::Buffer>,
    uniform_buffers_memory: Vec<vk::DeviceMemory>,
    // ...
}
```

Similarly, create a new function `create_uniform_buffers` that is called after `create_index_buffer` and allocates the buffers:

```rust,noplaypen
impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        // ...
        create_vertex_buffer(&instance, &device, &mut data)?;
        create_index_buffer(&instance, &device, &mut data)?;
        create_uniform_buffers(&instance, &device, &mut data)?;
        // ...
    }
}

unsafe fn create_uniform_buffers(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    data.uniform_buffers.clear();
    data.uniform_buffers_memory.clear();

    for _ in 0..data.swapchain_images.len() {
        let (uniform_buffer, uniform_buffer_memory) = create_buffer(
            instance,
            device,
            data,
            size_of::<UniformBufferObject>() as u64,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        )?;

        data.uniform_buffers.push(uniform_buffer);
        data.uniform_buffers_memory.push(uniform_buffer_memory);
    }

    Ok(())
}
```

We're going to write a separate function that updates the uniform buffer with a new transformation every frame, so there will be no `map_memory` here. The uniform data will be used for all draw calls, so the buffer containing it should only be destroyed when we stop rendering. Since it also depends on the number of swapchain images, which could change after a recreation, we'll clean it up in `destroy_swapchain`:

```rust,noplaypen
unsafe fn destroy_swapchain(&mut self) {
    self.data.uniform_buffers
        .iter()
        .for_each(|b| self.device.destroy_buffer(*b, None));
    self.data.uniform_buffers_memory
        .iter()
        .for_each(|m| self.device.free_memory(*m, None));
    // ...
}
```

This means that we also need to recreate it in `recreate_swapchain`:

```rust,noplaypen
unsafe fn recreate_swapchain(&mut self, window: &Window) -> Result<()> {
    // ...
    create_framebuffers(&self.device, &mut self.data)?;
    create_uniform_buffers(&self.instance, &self.device, &mut self.data)?;
    create_command_buffers(&self.device, &mut self.data)?;
    Ok(())
}
```

## Updating uniform data

Create a new method `App::update_uniform_buffer` and add a call to it from the `App::render` method right after we wait for the fence for the acquired swapchain image to be signalled:

```rust,noplaypen
impl App {
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        // ...

        if !self.data.images_in_flight[image_index as usize].is_null() {
            self.device.wait_for_fences(
                &[self.data.images_in_flight[image_index as usize]],
                true,
                u64::max_value(),
            )?;
        }

        self.data.images_in_flight[image_index as usize] =
            self.data.in_flight_fences[self.frame];

        self.update_uniform_buffer(image_index)?;

        // ...
    }

    unsafe fn update_uniform_buffer(&self, image_index: usize) -> Result<()> {
        Ok(())
    }
}
```

It is important that the uniform buffer is not updated until after this fence is signalled!

As a quick refresher on the usage of fences as introduced in the [`Rendering and Presentation` chapter](../drawing/rendering_and_presentation.html#frames-in-flight), we are using fences so that the GPU can notify the CPU once it is done processing a previously submitted frame. These notifications are used for two purposes: to prevent the CPU from submitting more frames when there are already `MAX_FRAMES_IN_FLIGHT` unfinished frames submitted to the GPU and also to ensure the CPU doesn't alter or delete resources like uniform buffers or command buffers while they are still being used by the GPU to process a frame.

Our uniform buffers are associated with our swapchain images, so we need to be sure that any previous frame that rendered to the acquired swapchain image is complete before we can safely update the uniform buffer. By only updating the uniform buffer after the GPU has notified the CPU that this is the case we can safely do whatever we want with the uniform buffer.

Going back to `App::update_uniform_buffer`, this method will generate a new transformation every frame to make the geometry spin around. We need to add an import to implement this functionality:

```rust,noplaypen
use std::time::Instant;
```

The `Instant` struct will allow us to do precise timekeeping. We'll use this to make sure that the geometry rotates 90 degrees per second regardless of frame rate. Add a field to `App` to track the time the application started and initialize the field to `Instant::now()` in `App::create`:

```rust,noplaypen
struct App {
    // ...
    start: Instant,
}
```

We can now use that field to determine how many seconds it has been since the application started:

```rust,noplaypen
unsafe fn update_uniform_buffer(&self, image_index: usize) -> Result<()> {
    let time = self.start.elapsed().as_secs_f32();

    Ok(())
}
```

We will now define the model, view and projection transformations in the uniform buffer object. The model rotation will be a simple rotation around the Z-axis using the `time` variable:

```rust,noplaypen
let model = glm::rotate(
    &glm::identity(),
    time * glm::radians(&glm::vec1(90.0))[0],
    &glm::vec3(0.0, 0.0, 1.0),
);
```

The `glm::rotate` function takes an existing transformation, rotation angle and rotation axis as parameters. The `glm::identity()` constructor returns an identity matrix. Using a rotation angle of `time * glm::radians(&glm::vec1(90.0))[0]` accomplishes the purpose of rotating 90 degrees per second.

```rust,noplaypen
let view = glm::look_at(
    &glm::vec3(2.0, 2.0, 2.0),
    &glm::vec3(0.0, 0.0, 0.0),
    &glm::vec3(0.0, 0.0, 1.0),
);
```

For the view transformation I've decided to look at the geometry from above at a 45 degree angle. The `glm::look_at` function takes the eye position, center position and up axis as parameters.

```rust,noplaypen
let mut proj = glm::perspective(
    self.data.swapchain_extent.width as f32 / self.data.swapchain_extent.height as f32,
    glm::radians(&glm::vec1(45.0))[0],
    0.1,
    10.0,
);
```

I've chosen to use a perspective projection with a 45 degree vertical field-of-view. The other parameters are the aspect ratio, near and far view planes. It is important to use the current swapchain extent to calculate the aspect ratio to take into account the new width and height of the window after a resize.

```rust,noplaypen
proj[(1, 1)] *= -1.0;
```

GLM was originally designed for OpenGL, where the Y coordinate of the clip coordinates is inverted. The easiest way to compensate for that is to flip the sign on the scaling factor of the Y axis in the projection matrix. If you don't do this, then the image will be rendered upside down.

```rust,noplaypen
let ubo = UniformBufferObject { model, view, proj };
```

Lastly we combine our matrices into a uniform buffer object.

All of the transformations are defined now, so we can copy the data in the uniform buffer object to the current uniform buffer. This happens in exactly the same way as we did for vertex buffers, except without a staging buffer:

```rust,noplaypen
let memory = self.device.map_memory(
    self.data.uniform_buffers_memory[image_index],
    0,
    size_of::<UniformBufferObject>() as u64,
    vk::MemoryMapFlags::empty(),
)?;

memcpy(&ubo, memory.cast(), 1);

self.device.unmap_memory(self.data.uniform_buffers_memory[image_index]);
```

Using a UBO this way is not the most efficient way to pass frequently changing values to the shader. A more efficient way to pass a small buffer of data to shaders are *push constants*. We may look at these in a future chapter.

If you run the program now, you'll get errors about unbound descriptor sets from the validation layer and nothing will be rendered. In the next chapter we'll look at these descriptor sets, which will actually bind the `vk::Buffer`s to the uniform buffer descriptors so that the shader can access this transformation data and get our program in running order again.
