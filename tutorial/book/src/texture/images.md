# Images

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/23_texture_image.rs)

The geometry has been colored using per-vertex colors so far, which is a rather limited approach. In this part of the tutorial we're going to implement texture mapping to make the geometry look more interesting. This will also allow us to load and draw basic 3D models in a future chapter.

Adding a texture to our application will involve the following steps:

* Create an image object backed by device memory
* Fill it with pixels from an image file
* Create an image sampler
* Add a combined image sampler descriptor to sample colors from the texture

We've already worked with image objects before, but those were automatically created by the swapchain extension. This time we'll have to create one by ourselves. Creating an image and filling it with data is similar to vertex buffer creation. We'll start by creating a staging resource and filling it with pixel data and then we copy this to the final image object that we'll use for rendering. Although it is possible to create a staging image for this purpose, Vulkan also allows you to copy pixels from a `vk::Buffer` to an image and the API for this is actually [faster on some hardware](https://developer.nvidia.com/vulkan-memory-management). We'll first create this buffer and fill it with pixel values, and then we'll create an image to copy the pixels to. Creating an image is not very different from creating buffers. It involves querying the memory requirements, allocating device memory and binding it, just like we've seen before.

However, there is something extra that we'll have to take care of when working with images. Images can have different *layouts* that affect how the pixels are organized in memory. Due to the way graphics hardware works, simply storing the pixels row by row may not lead to the best performance, for example. When performing any operation on images, you must make sure that they have the layout that is optimal for use in that operation. We've actually already seen some of these layouts when we specified the render pass:

* `vk::ImageLayout::PRESENT_SRC_KHR` &ndash; Optimal for presentation&nbsp;
* `vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL` &ndash; Optimal as attachment for writing colors from the fragment shader&nbsp;
* `vk::ImageLayout::TRANSFER_SRC_OPTIMAL` &ndash; Optimal as source in a transfer operation, like `cmd_copy_image_to_buffer`&nbsp;
* `vk::ImageLayout::TRANSFER_DST_OPTIMAL` &ndash; Optimal as destination in a transfer operation, like `cmd_copy_buffer_to_image`&nbsp;
* `vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL` &ndash; Optimal for sampling from a shader&nbsp;

One of the most common ways to transition the layout of an image is a *pipeline barrier*. Pipeline barriers are primarily used for synchronizing access to resources, like making sure that an image was written to before it is read, but they can also be used to transition layouts. In this chapter we'll see how pipeline barriers are used for this purpose. Barriers can additionally be used to transfer queue family ownership when using `vk::SharingMode::EXCLUSIVE`.

## Image library

There are many libraries available for loading images, and you can even write your own code to load simple formats like BMP and PPM. In this tutorial we'll be using the [`png`](https://crates.io/crates/png) crate which you should have already added to your program's dependencies.

## Loading an image

We'll need to open the image file, so add the following import:

```rust,noplaypen
use std::fs::File;
```

Create a new function `create_texture_image` where we'll load an image and upload it into a Vulkan image object. We're going to use command buffers, so it should be called after `^create_command_pool`.

```rust,noplaypen
impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        // ...
        create_command_pool(&instance, &device, &mut data)?;
        create_texture_image(&instance, &device, &mut data)?;
        // ...
    }
}

unsafe fn create_texture_image(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    Ok(())
}
```

Create a new directory `resources` next to the `shaders` directory to store texture images in. We're going to load an image called `texture.png` from that directory. I've chosen to use the following [CC0 licensed image](https://pixabay.com/en/statue-sculpture-fig-historically-1275469/) resized to 512 x 512 pixels, but feel free to pick any PNG image (with an alpha channel) you want.

![](../images/texture.png)

Loading an image with this library is really easy:

```rust,noplaypen
unsafe fn create_texture_image(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    let image = File::open("resources/texture.png")?;

    let decoder = png::Decoder::new(image);
    let mut reader = decoder.read_info()?;

    let mut pixels = vec![0;  reader.info().raw_bytes()];
    reader.next_frame(&mut pixels)?;

    let size = reader.info().raw_bytes() as u64;
    let (width, height) = reader.info().size();

    Ok(())
}
```

This code will populate the `pixels` list with the pixels data with 4 bytes per pixel which will result in a total of `width * height * 4` values. Note that the `png` crate [does not have support for converting RGB images into RGBA images (at the time of writing)](https://github.com/image-rs/image-png/issues/239) and the following code is expecting the pixel data to have an alpha channel. As a result you will need to make sure you use PNG images with alpha channels (such as the above image).

## Staging buffer

We're now going to create a buffer in host visible memory so that we can use `map_memory` and copy the pixels to it. The buffer should be in host visible memory so that we can map it and it should be usable as a transfer source so that we can copy it to an image later on:

```rust,noplaypen
let (staging_buffer, staging_buffer_memory) = create_buffer(
    instance,
    device,
    data,
    size,
    vk::BufferUsageFlags::TRANSFER_SRC,
    vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
)?;
```

We can then directly copy the pixel values that we got from the image loading library to the buffer:

```rust,noplaypen
let memory = device.map_memory(
    staging_buffer_memory,
    0,
    size,
    vk::MemoryMapFlags::empty(),
)?;

memcpy(pixels.as_ptr(), memory.cast(), pixels.len());

device.unmap_memory(staging_buffer_memory);
```

## Texture Image

Although we could set up the shader to access the pixel values in the buffer, it's better to use image objects in Vulkan for this purpose. Image objects will make it easier and faster to retrieve colors by allowing us to use 2D coordinates, for one. Pixels within an image object are known as texels and we'll use that name from this point on. Add the following new `AppData` fields:

```rust,noplaypen
struct AppData {
    // ...
    texture_image: vk::Image,
    texture_image_memory: vk::DeviceMemory,
}
```

The parameters for an image are specified in a `vk::ImageCreateInfo` struct:

```rust,noplaypen
let info = vk::ImageCreateInfo::builder()
    .image_type(vk::ImageType::_2D)
    .extent(vk::Extent3D { width, height, depth: 1 })
    .mip_levels(1)
    .array_layers(1)
    // continued...
```

The image type, specified in the `image_type` field, tells Vulkan with what kind of coordinate system the texels in the image are going to be addressed. It is possible to create 1D, 2D and 3D images. One dimensional images can be used to store an array of data or gradient, two dimensional images are mainly used for textures, and three dimensional images can be used to store voxel volumes, for example. The `extent` field specifies the dimensions of the image, basically how many texels there are on each axis. That's why `depth` must be `1` instead of `0`. Our texture will not be an array and we won't be using mipmapping for now.

```rust,noplaypen
    .format(vk::Format::R8G8B8A8_SRGB)
```

Vulkan supports many possible image formats, but we should use the same format for the texels as the pixels in the buffer, otherwise the copy operation will fail.

```rust,noplaypen
    .tiling(vk::ImageTiling::OPTIMAL)
```

The `tiling` field can have one of two values:

* `vk::ImageTiling::LINEAR` &ndash; Texels are laid out in row-major order like our `pixels` array
* `vk::ImageTiling::OPTIMAL` &ndash; Texels are laid out in an implementation defined order for optimal access

Unlike the layout of an image, the tiling mode cannot be changed at a later time. If you want to be able to directly access texels in the memory of the image, then you must use `vk::ImageTiling::LINEAR`. We will be using a staging buffer instead of a staging image, so this won't be necessary. We will be using `vk::ImageTiling::OPTIMAL` for efficient access from the shader.

```rust,noplaypen
    .initial_layout(vk::ImageLayout::UNDEFINED)
```

There are only two possible values for the `initial_layout` of an image:

* `vk::ImageLayout::UNDEFINED` &ndash; Not usable by the GPU and the very first transition will discard the texels.
* `vk::ImageLayout::PREINITIALIZED` &ndash; Not usable by the GPU, but the first transition will preserve the texels.

There are few situations where it is necessary for the texels to be preserved during the first transition. One example, however, would be if you wanted to use an image as a staging image in combination with the `vk::ImageTiling::LINEAR` layout. In that case, you'd want to upload the texel data to it and then transition the image to be a transfer source without losing the data. In our case, however, we're first going to transition the image to be a transfer destination and then copy texel data to it from a buffer object, so we don't need this property and can safely use `vk::ImageLayout::UNDEFINED`.

```rust,noplaypen
    .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST)
```

The `usage` field has the same semantics as the one during buffer creation. The image is going to be used as destination for the buffer copy, so it should be set up as a transfer destination. We also want to be able to access the image from the shader to color our mesh, so the usage should include `vk::ImageUsageFlags::SAMPLED`.

```rust,noplaypen
    .sharing_mode(vk::SharingMode::EXCLUSIVE)
```

The image will only be used by one queue family: the one that supports graphics (and therefore also) transfer operations.

```rust,noplaypen
    .samples(vk::SampleCountFlags::_1)
```

The `samples` flag is related to multisampling. This is only relevant for images that will be used as attachments, so stick to one sample. 

```rust,noplaypen
    .flags(vk::ImageCreateFlags::empty()); // Optional.
```

There are also some optional flags for images that allow controlling more advanced properties for things like sparse images. Sparse images are images where only certain regions are actually backed by memory. If you were using a 3D texture for a voxel terrain, for example, then you could use this to avoid allocating memory to store large volumes of "air" values. We won't be using it in this tutorial, so you can omit the builder method for this field which will set it to the default (an empty set of flags).

```c++
data.texture_image = device.create_image(&info, None)?;
```

The image is created using `create_image`, which doesn't have any particularly noteworthy parameters. It is possible that the `vk::Format::R8G8B8A8_SRGB` format is not supported by the graphics hardware. You should have a list of acceptable alternatives and go with the best one that is supported. However, support for this particular format is so widespread that we'll skip this step. Using different formats would also require annoying conversions. We will get back to this in the depth buffer chapter, where we'll implement such a system.

```rust,noplaypen
let requirements = device.get_image_memory_requirements(data.texture_image);

let info = vk::MemoryAllocateInfo::builder()
    .allocation_size(requirements.size)
    .memory_type_index(get_memory_type_index(
        instance,
        data,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
        requirements,
    )?);

data.texture_image_memory = device.allocate_memory(&info, None)?;

device.bind_image_memory(data.texture_image, data.texture_image_memory, 0)?;
```

Allocating memory for an image works in exactly the same way as allocating memory for a buffer. Use `get_image_memory_requirements` instead of `get_buffer_memory_requirements`, and use `bind_image_memory` instead of `bind_buffer_memory`.

This function is already getting quite large and there'll be a need to create more images in later chapters, so we should abstract image creation into a `^create_image` function, like we did for buffers. Create the function and move the image object creation and memory allocation to it:

```rust,noplaypen
unsafe fn create_image(
    instance: &Instance,
    device: &Device,
    data: &AppData,
    width: u32,
    height: u32,
    format: vk::Format,
    tiling: vk::ImageTiling,
    usage: vk::ImageUsageFlags,
    properties: vk::MemoryPropertyFlags,
) -> Result<(vk::Image, vk::DeviceMemory)> {
    let info = vk::ImageCreateInfo::builder()
        .image_type(vk::ImageType::_2D)
        .extent(vk::Extent3D {
            width,
            height,
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(1)
        .format(format)
        .tiling(tiling)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .usage(usage)
        .samples(vk::SampleCountFlags::_1)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    let image = device.create_image(&info, None)?;

    let requirements = device.get_image_memory_requirements(image);

    let info = vk::MemoryAllocateInfo::builder()
        .allocation_size(requirements.size)
        .memory_type_index(get_memory_type_index(
            instance,
            data,
            properties,
            requirements,
        )?);

    let image_memory = device.allocate_memory(&info, None)?;

    device.bind_image_memory(image, image_memory, 0)?;

    Ok((image, image_memory))
}
```

I've made the width, height, format, tiling mode, usage, and memory properties parameters, because these will all vary between the images we'll be creating throughout this tutorial.

The `create_texture_image` function can now be simplified to:

```rust,noplaypen
unsafe fn create_texture_image(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    let image = File::open("resources/texture.png")?;

    let decoder = png::Decoder::new(image);
    let mut reader = decoder.read_info()?;

    let mut pixels = vec![0;  reader.info().raw_bytes()];
    reader.next_frame(&mut pixels)?;

    let size = reader.info().raw_bytes() as u64;
    let (width, height) = reader.info().size();

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = device.map_memory(
        staging_buffer_memory,
        0,
        size,
        vk::MemoryMapFlags::empty(),
    )?;

    memcpy(pixels.as_ptr(), memory.cast(), pixels.len());

    device.unmap_memory(staging_buffer_memory);

    let (texture_image, texture_image_memory) = create_image(
        instance,
        device,
        data,
        width,
        height,
        vk::Format::R8G8B8A8_SRGB,
        vk::ImageTiling::OPTIMAL,
        vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    data.texture_image = texture_image;
    data.texture_image_memory = texture_image_memory;

    Ok(())
}
```

## Layout transitions

The function we're going to write now involves recording and executing a command buffer again, so now's a good time to move that logic into a helper function or two:

```rust,noplaypen
unsafe fn begin_single_time_commands(
    device: &Device,
    data: &AppData,
) -> Result<vk::CommandBuffer> {
    let info = vk::CommandBufferAllocateInfo::builder()
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_pool(data.command_pool)
        .command_buffer_count(1);

    let command_buffer = device.allocate_command_buffers(&info)?[0];

    let info = vk::CommandBufferBeginInfo::builder()
        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    device.begin_command_buffer(command_buffer, &info)?;

    Ok(command_buffer)
}

unsafe fn end_single_time_commands(
    device: &Device,
    data: &AppData,
    command_buffer: vk::CommandBuffer,
) -> Result<()> {
    device.end_command_buffer(command_buffer)?;

    let command_buffers = &[command_buffer];
    let info = vk::SubmitInfo::builder()
        .command_buffers(command_buffers);

    device.queue_submit(data.graphics_queue, &[info], vk::Fence::null())?;
    device.queue_wait_idle(data.graphics_queue)?;

    device.free_command_buffers(data.command_pool, &[command_buffer]);

    Ok(())
}
```

The code for these functions is based on the existing code in `copy_buffer`. You can now simplify that function to:

```rust,noplaypen
unsafe fn copy_buffer(
    device: &Device,
    data: &AppData,
    source: vk::Buffer,
    destination: vk::Buffer,
    size: vk::DeviceSize,
) -> Result<()> {
    let command_buffer = begin_single_time_commands(device, data)?;

    let regions = vk::BufferCopy::builder().size(size);
    device.cmd_copy_buffer(command_buffer, source, destination, &[regions]);

    end_single_time_commands(device, data, command_buffer)?;

    Ok(())
}
```

If we were still using buffers, then we could now write a function to record and execute `cmd_copy_buffer_to_image` to finish the job, but this command requires the image to be in the right layout first. Create a new function to handle layout transitions:

```rust,noplaypen
unsafe fn transition_image_layout(
    device: &Device,
    data: &AppData,
    image: vk::Image,
    format: vk::Format,
    old_layout: vk::ImageLayout,
    new_layout: vk::ImageLayout,
) -> Result<()> {
    let command_buffer = begin_single_time_commands(device, data)?;

    end_single_time_commands(device, data, command_buffer)?;

    Ok(())
}
```

One of the most common ways to perform layout transitions is using an *image memory barrier*. A pipeline barrier like that is generally used to synchronize access to resources, like ensuring that a write to a buffer completes before reading from it, but it can also be used to transition image layouts and transfer queue family ownership when `vk::SharingMode::EXCLUSIVE` is used. There is an equivalent *buffer memory barrier* to do this for buffers.

```rust,noplaypen
let barrier = vk::ImageMemoryBarrier::builder()
    .old_layout(old_layout)
    .new_layout(new_layout)
    // continued...
```

The first two fields specify layout transition. It is possible to use `vk::ImageLayout::UNDEFINED` as `old_layout` if you don't care about the existing contents of the image.

```rust,noplaypen
    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
```

If you are using the barrier to transfer queue family ownership, then these two fields should be the indices of the queue families. They must be set to `vk::QUEUE_FAMILY_IGNORED` if you don't want to do this (not the default value!).

```rust,noplaypen
    .image(image)
    .subresource_range(subresource)
```

The `image` and `subresource_range` specify the image that is affected and the specific part of the image. We'll need to define `subresource` before our image memory barrier:

```rust,noplaypen
let subresource = vk::ImageSubresourceRange::builder()
    .aspect_mask(vk::ImageAspectFlags::COLOR)
    .base_mip_level(0)
    .level_count(1)
    .base_array_layer(0)
    .layer_count(1);
```

Our image is not an array and does not have mipmapping levels, so only one level and layer are specified.

```rust,noplaypen
    .src_access_mask(vk::AccessFlags::empty())  // TODO
    .dst_access_mask(vk::AccessFlags::empty()); // TODO
```

Barriers are primarily used for synchronization purposes, so you must specify which types of operations that involve the resource must happen before the barrier, and which operations that involve the resource must wait on the barrier. We need to do that despite already using `queue_wait_idle` to manually synchronize. The right values depend on the old and new layout, so we'll get back to this once we've figured out which transitions we're going to use.

```rust,noplaypen
device.cmd_pipeline_barrier(
    command_buffer,
    vk::PipelineStageFlags::empty(), // TODO
    vk::PipelineStageFlags::empty(), // TODO
    vk::DependencyFlags::empty(),
    &[] as &[vk::MemoryBarrier],
    &[] as &[vk::BufferMemoryBarrier],
    &[barrier],
);
```

All types of pipeline barriers are submitted using the same function. The first parameter after the command buffer specifies in which pipeline stage the operations occur that should happen before the barrier. The second parameter specifies the pipeline stage in which operations will wait on the barrier. The pipeline stages that you are allowed to specify before and after the barrier depend on how you use the resource before and after the barrier. The allowed values are listed in [this table](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#synchronization-access-types-supported) of the specification. For example, if you're going to read from a uniform after the barrier, you would specify a usage of `vk::AccessFlags::UNIFORM_READ` and the earliest shader that will read from the uniform as pipeline stage, for example `vk::PipelineStageFlags::FRAGMENT_SHADER`. It would not make sense to specify a non-shader pipeline stage for this type of usage and the validation layers will warn you when you specify a pipeline stage that does not match the type of usage.

The fourth parameter is either an empty set of `vk::DependencyFlags` or `vk::DependencyFlags::BY_REGION`. The latter turns the barrier into a per-region condition. That means that the implementation is allowed to already begin reading from the parts of a resource that were written so far, for example.

The last three parameters reference slices of pipeline barriers of the three available types: memory barriers, buffer memory barriers, and image memory barriers like the one we're using here. Note that we're not using the `vk::Format` parameter yet, but we'll be using that one for special transitions in the depth buffer chapter.

## Copying buffer to image

Before we get back to `create_texture_image`, we're going to write one more helper function: `copy_buffer_to_image`:

```rust,noplaypen
unsafe fn copy_buffer_to_image(
    device: &Device,
    data: &AppData,
    buffer: vk::Buffer,
    image: vk::Image,
    width: u32,
    height: u32,
) -> Result<()> {
    let command_buffer = begin_single_time_commands(device, data)?;

    end_single_time_commands(device, data, command_buffer)?;

    Ok(())
}
```

Just like with buffer copies, you need to specify which part of the buffer is going to be copied to which part of the image. This happens through `vk::BufferImageCopy` structs:

```rust,noplaypen
let subresource = vk::ImageSubresourceLayers::builder()
    .aspect_mask(vk::ImageAspectFlags::COLOR)
    .mip_level(0)
    .base_array_layer(0)
    .layer_count(1);

let region = vk::BufferImageCopy::builder()
    .buffer_offset(0)
    .buffer_row_length(0)
    .buffer_image_height(0)
    .image_subresource(subresource)
    .image_offset(vk::Offset3D { x: 0, y: 0, z: 0 })
    .image_extent(vk::Extent3D { width, height, depth: 1 });
```

Most of these fields are self-explanatory. The `buffer_offset` specifies the byte offset in the buffer at which the pixel values start. The `buffer_row_length` and `buffer_image_height` fields specify how the pixels are laid out in memory. For example, you could have some padding bytes between rows of the image. Specifying `0` for both indicates that the pixels are simply tightly packed like they are in our case. The `image_subresource`, `image_offset` and `image_extent` fields indicate to which part of the image we want to copy the pixels.

Buffer to image copy operations are enqueued using the `cmd_copy_buffer_to_image` function:

```rust,noplaypen
device.cmd_copy_buffer_to_image(
    command_buffer,
    buffer,
    image,
    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
    &[region],
);
```

The fourth parameter indicates which layout the image is currently using. I'm assuming here that the image has already been transitioned to the layout that is optimal for copying pixels to. Right now we're only copying one chunk of pixels to the whole image, but it's possible to specify an array of `vk::BufferImageCopy` to perform many different copies from this buffer to the image in one operation.

## Preparing the texture image

We now have all of the tools we need to finish setting up the texture image, so we're going back to the `create_texture_image` function. The last thing we did there was creating the texture image. The next step is to copy the staging buffer to the texture image. This involves two steps:

* Transition the texture image to `vk::ImageLayout::TRANSFER_DST_OPTIMAL`&nbsp;
* Execute the buffer to image copy operation

This is easy to do with the functions we just created:

```rust,noplaypen
transition_image_layout(
    device,
    data,
    data.texture_image,
    vk::Format::R8G8B8A8_SRGB,
    vk::ImageLayout::UNDEFINED,
    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
)?;

copy_buffer_to_image(
    device,
    data,
    staging_buffer,
    data.texture_image,
    width,
    height,
)?;
```

The image was created with the `vk::ImageLayout::UNDEFINED` layout, so that one should be specified as old layout when transitioning `texture_image`. Remember that we can do this because we don't care about its contents before performing the copy operation.

To be able to start sampling from the texture image in the shader, we need one last transition to prepare it for shader access:

```rust,noplaypen
transition_image_layout(
    device,
    data,
    data.texture_image,
    vk::Format::R8G8B8A8_SRGB,
    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
    vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
)?;
```

## Transition barrier masks

If you run your application with validation layers enabled now, then you'll see that it complains about the access masks and pipeline stages in `transition_image_layout` being invalid. We still need to set those based on the layouts in the transition.

There are two transitions we need to handle:

* Undefined → transfer destination &ndash; transfer writes that don't need to wait on anything
* Transfer destination → shader reading &ndash; shader reads should wait on transfer writes, specifically the shader reads in the fragment shader, because that's where we're going to use the texture

These rules are specified using the following access masks and pipeline stages which should be added at the beginning of `transition_image_layout`:

```rust,noplaypen
let (
    src_access_mask,
    dst_access_mask,
    src_stage_mask,
    dst_stage_mask,
) = match (old_layout, new_layout) {
    (vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL) => (
        vk::AccessFlags::empty(),
        vk::AccessFlags::TRANSFER_WRITE,
        vk::PipelineStageFlags::TOP_OF_PIPE,
        vk::PipelineStageFlags::TRANSFER,
    ),
    (vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL) => (
        vk::AccessFlags::TRANSFER_WRITE,
        vk::AccessFlags::SHADER_READ,
        vk::PipelineStageFlags::TRANSFER,
        vk::PipelineStageFlags::FRAGMENT_SHADER,
    ),
    _ => return Err(anyhow!("Unsupported image layout transition!")),
};
```

Then update the `vk::ImageMemoryBarrier` struct and the `cmd_pipeline_barrier` call with the access flags and pipeline stage masks:

```rust,noplaypen
let barrier = vk::ImageMemoryBarrier::builder()
    .old_layout(old_layout)
    .new_layout(new_layout)
    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
    .image(image)
    .subresource_range(subresource)
    .src_access_mask(src_access_mask)
    .dst_access_mask(dst_access_mask);

device.cmd_pipeline_barrier(
    command_buffer,
    src_stage_mask,
    dst_stage_mask,
    vk::DependencyFlags::empty(),
    &[] as &[vk::MemoryBarrier],
    &[] as &[vk::BufferMemoryBarrier],
    &[barrier],
);
```

As you can see in the aforementioned table, transfer writes must occur in the pipeline transfer stage. Since the writes don't have to wait on anything, you may specify an empty access mask and the earliest possible pipeline stage `vk::PipelineStageFlags::TOP_OF_PIPE` for the pre-barrier operations. It should be noted that `vk::PipelineStageFlags::TRANSFER` is not a *real* stage within the graphics and compute pipelines. It is more of a pseudo-stage where transfers happen. See [the documentation](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineStageFlagBits.html) for more information and other examples of pseudo-stages.

The image will be written in the same pipeline stage and subsequently read by the fragment shader, which is why we specify shader reading access in the fragment shader pipeline stage.

If we need to do more transitions in the future, then we'll extend the function. The application should now run successfully, although there are of course no visual changes yet.

One thing to note is that command buffer submission results in implicit `vk::AccessFlags::HOST_WRITE` synchronization at the beginning. Since the `transition_image_layout` function executes a command buffer with only a single command, you could use this implicit synchronization and set `src_access_mask` to `vk::AccessFlags::empty()` if you ever needed a `vk::AccessFlags::HOST_WRITE` dependency in a layout transition. It's up to you if you want to be explicit about it or not, but I'm personally not a fan of relying on these OpenGL-like "hidden" operations.

There is actually a special type of image layout that supports all operations, `vk::ImageLayout::GENERAL`. The problem with it, of course, is that it doesn't necessarily offer the best performance for any operation. It is required for some special cases, like using an image as both input and output, or for reading an image after it has left the preinitialized layout.

All of the helper functions that submit commands so far have been set up to execute synchronously by waiting for the queue to become idle. For practical applications it is recommended to combine these operations in a single command buffer and execute them asynchronously for higher throughput, especially the transitions and copy in the `create_texture_image` function. Try to experiment with this by creating a `setup_command_buffer` that the helper functions record commands into, and add a `flush_setup_commands` to execute the commands that have been recorded so far. It's best to do this after the texture mapping works to check if the texture resources are still set up correctly.

## Cleanup

Finish the `create_texture_image` function by cleaning up the staging buffer and its memory at the end:

```rust,noplaypen
device.destroy_buffer(staging_buffer, None);
device.free_memory(staging_buffer_memory, None);
```

The main texture image is used until the end of the program:

```rust,noplaypen
unsafe fn destroy(&mut self) {
    self.destroy_swapchain();
    self.device.destroy_image(self.data.texture_image, None);
    self.device.free_memory(self.data.texture_image_memory, None);
    // ...
}
```

The image now contains the texture, but we still need a way to access it from the graphics pipeline. We'll work on that in the next chapter.
