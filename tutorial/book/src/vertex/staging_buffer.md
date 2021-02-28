# Staging buffer

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/19_staging_buffer.rs)

The vertex buffer we have right now works correctly, but the memory type that allows us to access it from the CPU may not be the most optimal memory type for the graphics card itself to read from. The most optimal memory has the `vk::MemoryPropertyFlags::DEVICE_LOCAL` flag and is usually not accessible by the CPU on dedicated graphics cards. In this chapter we're going to create two vertex buffers. One *staging buffer* in CPU accessible memory to upload the data from the vertex array to, and the final vertex buffer in device local memory. We'll then use a buffer copy command to move the data from the staging buffer to the actual vertex buffer.

## Transfer queue

The buffer copy command requires a queue family that supports transfer operations, which is indicated using `vk::QueueFlags::TRANSFER`. The good news is that any queue family with `vk::QueueFlags::GRAPHICS` or `vk::QueueFlags::COMPUTE` capabilities already implicitly support `vk::QueueFlags::TRANSFER` operations. The implementation is not required to explicitly list it in `queue_flags` in those cases.

If you like a challenge, then you can still try to use a different queue family specifically for transfer operations. It will require you to make the following modifications to your program:

* Modify `QueueFamilyIndices` and `QueueFamilyIndices::get` to explicitly look for a queue family with the `vk::QueueFlags::TRANSFER` bit, but not the `vk::QueueFlags::GRAPHICS`.
* Modify `create_logical_device` to request a handle to the transfer queue
* Create a second command pool for command buffers that are submitted on the transfer queue family
* Change the `sharing_mode` of resources to be `vk::SharingMode::CONCURRENT` and specify both the graphics and transfer queue families
* Submit any transfer commands like `cmd_copy_buffer` (which we'll be using in this chapter) to the transfer queue instead of the graphics queue

It's a bit of work, but it'll teach you a lot about how resources are shared between queue families.

## Abstracting buffer creation

Because we're going to create multiple buffers in this chapter, it's a good idea to move buffer creation to a helper function. Create a new function `^create_buffer` and move the code in `create_vertex_buffer` (except mapping) to it.

```rust,noplaypen
unsafe fn create_buffer(
    instance: &Instance,
    device: &Device,
    data: &AppData,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
    properties: vk::MemoryPropertyFlags,
) -> Result<(vk::Buffer, vk::DeviceMemory)> {
    let buffer_info = vk::BufferCreateInfo::builder()
        .size(size)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    let buffer = device.create_buffer(&buffer_info, None)?;

    let requirements = device.get_buffer_memory_requirements(buffer);

    let memory_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(requirements.size)
        .memory_type_index(get_memory_type_index(
            instance,
            data,
            properties,
            requirements,
        )?);

    let buffer_memory = device.allocate_memory(&memory_info, None)?;

    device.bind_buffer_memory(buffer, buffer_memory, 0)?;

    Ok((buffer, buffer_memory))
}
```

Make sure to add parameters for the buffer size, usage and memory properties so that we can use this function to create many different types of buffers.

You can now remove the buffer creation and memory allocation code from `create_vertex_buffer` and just call `^create_buffer` instead:

```rust,noplaypen
unsafe fn create_vertex_buffer(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    let size = (size_of::<Vertex>() * VERTICES.len()) as u64;

    let (vertex_buffer, vertex_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    data.vertex_buffer = vertex_buffer;
    data.vertex_buffer_memory = vertex_buffer_memory;

    let memory = device.map_memory(
        vertex_buffer_memory,
        0,
        size,
        vk::MemoryMapFlags::empty(),
    )?;

    memcpy(VERTICES.as_ptr(), memory.cast(), VERTICES.len());

    device.unmap_memory(vertex_buffer_memory);

    Ok(())
}
```

Run your program to make sure that the vertex buffer still works properly.

## Using a staging buffer

We're now going to change `create_vertex_buffer` to only use a host visible buffer as temporary buffer and use a device local one as actual vertex buffer.

```rust,noplaypen
unsafe fn create_vertex_buffer(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    let size = (size_of::<Vertex>() * VERTICES.len()) as u64;

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

    memcpy(VERTICES.as_ptr(), memory.cast(), VERTICES.len());

    device.unmap_memory(staging_buffer_memory);

    let (vertex_buffer, vertex_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    data.vertex_buffer = vertex_buffer;
    data.vertex_buffer_memory = vertex_buffer_memory;

    Ok(())
}
```

We're now using a new `staging_buffer` with `staging_buffer_memory` for mapping and copying the vertex data. In this chapter we're going to use two new buffer usage flags:

* `vk::BufferUsageFlags::TRANSFER_SRC` &ndash; Buffer can be used as source in a memory transfer operation.
* `vk::BufferUsageFlags::TRANSFER_DST` &ndash; Buffer can be used as destination in a memory transfer operation.

The `vertex_buffer` is now allocated from a memory type that is device local, which generally means that we're not able to use `map_memory`. However, we can copy data from the `staging_buffer` to the `vertex_buffer`. We have to indicate that we intend to do that by specifying the transfer source flag for the `staging_buffer` and the transfer destination flag for the `vertex_buffer`, along with the vertex buffer usage flag.

We're now going to write a function to copy the contents from one buffer to another, called `copy_buffer`.

```rust,noplaypen
unsafe fn copy_buffer(
    device: &Device,
    data: &AppData,
    source: vk::Buffer,
    destination: vk::Buffer,
    size: vk::DeviceSize,
) -> Result<()> {
    Ok(())
}
```

Memory transfer operations are executed using command buffers, just like drawing commands. Therefore we must first allocate a temporary command buffer. You may wish to create a separate command pool for these kinds of short-lived buffers, because the implementation may be able to apply memory allocation optimizations. You should use the `vk::CommandPoolCreateFlags::TRANSIENT` flag during command pool generation in that case.

```rust,noplaypen
unsafe fn copy_buffer(
    device: &Device,
    data: &AppData,
    source: vk::Buffer,
    destination: vk::Buffer,
    size: vk::DeviceSize,
) -> Result<()> {
    let info = vk::CommandBufferAllocateInfo::builder()
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_pool(data.command_pool)
        .command_buffer_count(1);

    let command_buffer = device.allocate_command_buffers(&info)?[0];

    Ok(())
}
```

And immediately start recording the command buffer:

```rust,noplaypen
let info = vk::CommandBufferBeginInfo::builder()
    .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

device.begin_command_buffer(command_buffer, &info)?;
```

We're only going to use the command buffer once and wait with returning from the function until the copy operation has finished executing. It's good practice to tell the driver about our intent using `vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT`.

```rust,noplaypen
let regions = vk::BufferCopy::builder().size(size);
device.cmd_copy_buffer(command_buffer, source, destination, &[regions]);
```

Contents of buffers are transferred using the `cmd_copy_buffer` command. It takes the source and destination buffers as arguments, and an array of regions to copy. The regions are defined in `vk::BufferCopy` structs and consist of a source buffer offset, destination buffer offset and size. It is not possible to specify `vk::WHOLE_SIZE` here, unlike the `map_memory` command.

```rust,noplaypen
device.end_command_buffer(command_buffer)?;
```

This command buffer only contains the copy command, so we can stop recording right after that. Now execute the command buffer to complete the transfer:

```rust,noplaypen
let command_buffers = &[command_buffer];
let info = vk::SubmitInfo::builder()
    .command_buffers(command_buffers);

device.queue_submit(data.graphics_queue, &[info], vk::Fence::null())?;
device.queue_wait_idle(data.graphics_queue)?;
```

Unlike the draw commands, there are no events we need to wait on this time. We just want to execute the transfer on the buffers immediately. There are again two possible ways to wait on this transfer to complete. We could use a fence and wait with `wait_for_fences`, or simply wait for the transfer queue to become idle with `queue_wait_idle`. A fence would allow you to schedule multiple transfers simultaneously and wait for all of them complete, instead of executing one at a time. That may give the driver more opportunities to optimize.

```rust,noplaypen
device.free_command_buffers(data.command_pool, &[command_buffer]);
```

Don't forget to clean up the command buffer used for the transfer operation.

We can now call `copy_buffer` from the `create_vertex_buffer` function to move the vertex data to the device local buffer:

```rust,noplaypen
copy_buffer(device, data, staging_buffer, vertex_buffer, size)?;
```

After copying the data from the staging buffer to the device buffer, we should clean it up:

```rust,noplaypen
device.destroy_buffer(staging_buffer, None);
device.free_memory(staging_buffer_memory, None);
```

Run your program to verify that you're seeing the familiar triangle again. The improvement may not be visible right now, but its vertex data is now being loaded from high performance memory. This will matter when we're going to start rendering more complex geometry.

## Conclusion

It should be noted that in a real world application, you're not supposed to actually call `allocate_memory` for every individual buffer. The maximum number of simultaneous memory allocations is limited by the `max_memory_allocation_count` physical device limit, which may be as low as `4096` even on high end hardware like an NVIDIA GTX 1080. The right way to allocate memory for a large number of objects at the same time is to create a custom allocator that splits up a single allocation among many different objects by using the `offset` parameters that we've seen in many functions.

However, for this tutorial it's okay to use a separate allocation for every resource, because we won't come close to hitting any of these limits for now.
