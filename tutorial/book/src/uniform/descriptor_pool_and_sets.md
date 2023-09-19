# Descriptor pool and sets

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/22_descriptor_sets.rs)

The descriptor layout from the previous chapter describes the type of descriptors that can be bound. In this chapter we're going to create a descriptor set for each `vk::Buffer` resource to bind it to the uniform buffer descriptor.

## Descriptor pool

Descriptor sets can't be created directly, they must be allocated from a pool like command buffers. The equivalent for descriptor sets is unsurprisingly called a *descriptor pool*. We'll write a new function `^create_descriptor_pool` to set it up.

```rust,noplaypen
impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        // ...
        create_uniform_buffers(&instance, &device, &mut data)?;
        create_descriptor_pool(&device, &mut data)?;
        // ...
    }
}

unsafe fn create_descriptor_pool(device: &Device, data: &mut AppData) -> Result<()> {
    Ok(())
}
```

We first need to describe which descriptor types our descriptor sets are going to contain and how many of them, using `vk::DescriptorPoolSize` structures.

```rust,noplaypen
let ubo_size = vk::DescriptorPoolSize::builder()
    .type_(vk::DescriptorType::UNIFORM_BUFFER)
    .descriptor_count(data.swapchain_images.len() as u32);
```

We will allocate one of these descriptors for every frame. This pool size structure is referenced by the main `vk::DescriptorPoolCreateInfo` along with the maximum number of descriptor sets that may be allocated:

```rust,noplaypen
let pool_sizes = &[ubo_size];
let info = vk::DescriptorPoolCreateInfo::builder()
    .pool_sizes(pool_sizes)
    .max_sets(data.swapchain_images.len() as u32);
```

The structure has an optional flag similar to command pools that determines if individual descriptor sets can be freed or not: `vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET`. We're not going to touch the descriptor set after creating it, so we don't need this flag.

```rust,noplaypen
struct AppData {
    // ...
    uniform_buffers: Vec<vk::Buffer>,
    uniform_buffers_memory: Vec<vk::DeviceMemory>,
    descriptor_pool: vk::DescriptorPool,
    // ...
}
```

Add a new `AppData` field to store the handle of the descriptor pool so you can call `create_descriptor_pool` to create it.

```rust,noplaypen
data.descriptor_pool = device.create_descriptor_pool(&info, None)?;
```

The descriptor pool should be destroyed when the swapchain is recreated because it depends on the number of images:

```rust,noplaypen
unsafe fn destroy_swapchain(&mut self) {
    self.device.destroy_descriptor_pool(self.data.descriptor_pool, None);
    // ...
}
```

And recreated in `App::recreate_swapchain`:

```rust,noplaypen
unsafe fn recreate_swapchain(&mut self, window: &Window) -> Result<()> {
    // ...
    create_uniform_buffers(&self.instance, &self.device, &mut self.data)?;
    create_descriptor_pool(&self.device, &mut self.data)?;
    // ...
}
```

## Descriptor set

We can now allocate the descriptor sets themselves. Add a `create_descriptor_sets` function for that purpose:

```rust,noplaypen
impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        // ...
        create_descriptor_pool(&device, &mut data)?;
        create_descriptor_sets(&device, &mut data)?;
        // ...
    }

    unsafe fn recreate_swapchain(&mut self, window: &Window) -> Result<()> {
        // ..
        create_descriptor_pool(&self.device, &mut self.data)?;
        create_descriptor_sets(&self.device, &mut self.data)?;
        // ..
    }
}

unsafe fn create_descriptor_sets(device: &Device, data: &mut AppData) -> Result<()> {
    Ok(())
}
```

A descriptor set allocation is described with a `vk::DescriptorSetAllocateInfo` struct. You need to specify the descriptor pool to allocate from and an array of descriptor layouts that describes each of the descriptor sets you are allocating:

```rust,noplaypen
let layouts = vec![data.descriptor_set_layout; data.swapchain_images.len()];
let info = vk::DescriptorSetAllocateInfo::builder()
    .descriptor_pool(data.descriptor_pool)
    .set_layouts(&layouts);
```

In our case we will create one descriptor set for each swapchain image, all with the same layout. Unfortunately we do need all the copies of the layout because the next function expects an array matching the number of sets.

Add an `AppData` field to hold the descriptor set handles:

```rust,noplaypen
struct AppData {
    // ...
    descriptor_pool: vk::DescriptorPool,
    descriptor_sets: Vec<vk::DescriptorSet>,
    // ...
}
```

And then allocate them with `allocate_descriptor_sets`:

```rust,noplaypen
data.descriptor_sets = device.allocate_descriptor_sets(&info)?;
```

You don't need to explicitly clean up descriptor sets, because they will be automatically freed when the descriptor pool is destroyed. The call to `allocate_descriptor_sets` will allocate descriptor sets, each with one uniform buffer descriptor.

The descriptor sets have been allocated now, but the descriptors within still need to be configured. We'll now add a loop to populate every descriptor:

```rust,noplaypen
for i in 0..data.swapchain_images.len() {

}
```

Descriptors that refer to buffers, like our uniform buffer descriptor, are configured with a `vk::DescriptorBufferInfo` struct. This structure specifies the buffer and the region within it that contains the data for the descriptor.

```rust,noplaypen
for i in 0..data.swapchain_images.len() {
    let info = vk::DescriptorBufferInfo::builder()
        .buffer(data.uniform_buffers[i])
        .offset(0)
        .range(size_of::<UniformBufferObject>() as u64);
}
```

If you're overwriting the whole buffer, like we are in this case, then it is is also possible to use the `vk::WHOLE_SIZE` value for the range. The configuration of descriptors is updated using the `update_descriptor_sets` function, which takes an array of `vk::WriteDescriptorSet` structs as parameter.

```rust,noplaypen
let buffer_info = &[info];
let ubo_write = vk::WriteDescriptorSet::builder()
    .dst_set(data.descriptor_sets[i])
    .dst_binding(0)
    .dst_array_element(0)
    // continued...
```

The first two fields specify the descriptor set to update and the binding. We gave our uniform buffer binding index `0`. Remember that descriptors can be arrays, so we also need to specify the first index in the array that we want to update. We're not using an array, so the index is simply `0`.

```rust,noplaypen
    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
```

We need to specify the type of descriptor again. It's possible to update multiple descriptors at once in an array, starting at index `dst_array_element`.

```rust,noplaypen
    .buffer_info(buffer_info);
```

The last field references an array with `descriptor_count` structs that actually configure the descriptors. It depends on the type of descriptor which one of the three you actually need to use. The `buffer_info` field is used for descriptors that refer to buffer data, `image_info` is used for descriptors that refer to image data, and `texel_buffer_view` is used for descriptors that refer to buffer views. Our descriptor is based on buffers, so we're using `buffer_info`.

```rust,noplaypen
device.update_descriptor_sets(&[ubo_write], &[] as &[vk::CopyDescriptorSet]);
```

The updates are applied using `update_descriptor_sets`. It accepts two kinds of arrays as parameters: an array of `vk::WriteDescriptorSet` and an array of `vk::CopyDescriptorSet`. The latter can be used to copy descriptors to each other, as its name implies.

## Using descriptor sets

We now need to update the `create_command_buffers` function to actually bind the right descriptor set for each swapchain image to the descriptors in the shader with `cmd_bind_descriptor_sets`. This needs to be done before the `cmd_draw_indexed` call:

```rust,noplaypen
device.cmd_bind_descriptor_sets(
    *command_buffer,
    vk::PipelineBindPoint::GRAPHICS,
    data.pipeline_layout,
    0,
    &[data.descriptor_sets[i]],
    &[],
);
device.cmd_draw_indexed(*command_buffer, INDICES.len() as u32, 1, 0, 0, 0);
```

Unlike vertex and index buffers, descriptor sets are not unique to graphics pipelines. Therefore we need to specify if we want to bind descriptor sets to the graphics or compute pipeline. The next parameter is the layout that the descriptors are based on. The next two parameters specify the index of the first descriptor set and the array of sets to bind. We'll get back to this in a moment. The last parameter specifies an array of offsets that are used for dynamic descriptors. We'll look at these in a future chapter.

If you run your program now, then you'll notice that unfortunately nothing is visible. The problem is that because of the Y-flip we did in the projection matrix, the vertices are now being drawn in counter-clockwise order instead of clockwise order. This causes backface culling to kick in and prevents any geometry from being drawn. Go to the `^create_pipeline` function and modify the `front_face` in `vk::PipelineRasterizationStateCreateInfo` to correct this:

```rust,noplaypen
    .cull_mode(vk::CullModeFlags::BACK)
    .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
```

Run your program again and you should now see the following:

![](../images/spinning_quad.png)

The rectangle has changed into a square because the projection matrix now corrects for aspect ratio. The `App::update_uniform_buffer` method takes care of screen resizing, so we don't need to recreate the descriptor set in `App::recreate_swapchain`.

## Alignment requirements

One thing we've glossed over so far is how exactly the data in the Rust structure should match with the uniform definition in the shader. It seems obvious enough to simply use the same types in both:

```rust,noplaypen
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct UniformBufferObject {
    model: Mat4,
    view: Mat4,
    proj: Mat4,
}
```

```glsl
layout(binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;
```

However, that's not all there is to it. For example, try modifying the struct and shader to look like this:

```rust,noplaypen
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct UniformBufferObject {
    foo: Vec2,
    model: Mat4,
    view: Mat4,
    proj: Mat4,
}
```

```glsl
layout(binding = 0) uniform UniformBufferObject {
    vec2 foo;
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;
```

Recompile your shader and your program and run it and you'll find that the colorful square you worked so far has disappeared! That's because we haven't taken into account the *alignment requirements*.

Vulkan expects the data in your structure to be aligned in memory in a specific way, for example:

* Scalars have to be aligned by N (= 4 bytes given 32 bit floats).
* A `vec2` must be aligned by 2N (= 8 bytes)
* A `vec3` or `vec4` must be aligned by 4N (= 16 bytes)
* A nested structure must be aligned by the base alignment of its members rounded up to a multiple of 16.
* A `mat4` matrix must have the same alignment as a `vec4`.

You can find the full list of alignment requirements in [the specification](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/html/chap14.html#interfaces-resources-layout).

Our original shader with just three `mat4` fields already met the alignment requirements. As each `mat4` is 4 x 4 x 4 = 64 bytes in size, `model` has an offset of `0`, `view` has an offset of 64 and `proj` has an offset of 128. All of these are multiples of 16 and that's why it worked fine.

The new structure starts with a `vec2` which is only 8 bytes in size and therefore throws off all of the offsets. Now `model` has an offset of `8`, `view` an offset of `72` and `proj` an offset of `136`, none of which are multiples of 16. Unfortunately Rust does not have great support for controlling the alignment of fields in structs, but we can use some manual padding to fix the alignment issues:

```rust,noplaypen
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct UniformBufferObject {
    foo: Vec2,
    _padding: [u8; 8],
    model: Mat4,
    view: Mat4,
    proj: Mat4,
}
```

If you now compile and run your program again you should see that the shader correctly receives its matrix values once again.

## Multiple descriptor sets

As some of the structures and function calls hinted at, it is actually possible to bind multiple descriptor sets simultaneously. You need to specify a descriptor layout for each descriptor set when creating the pipeline layout. Shaders can then reference specific descriptor sets like this:

```glsl
layout(set = 0, binding = 0) uniform UniformBufferObject { ... }
```

You can use this feature to put descriptors that vary per-object and descriptors that are shared into separate descriptor sets. In that case you avoid rebinding most of the descriptors across draw calls which is potentially more efficient.
