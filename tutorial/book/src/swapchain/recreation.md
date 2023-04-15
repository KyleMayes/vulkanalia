# Recreation

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/16_swapchain_recreation.rs)

The application we have now successfully draws a triangle, but there are some circumstances that it isn't handling properly yet. It is possible for the window surface to change such that the swapchain is no longer compatible with it. One of the reasons that could cause this to happen is the size of the window changing. We have to catch these events and recreate the swapchain.

## Recreating the swapchain

Create a new `App::recreate_swapchain` method that calls `create_swapchain` and all of the creation functions for the objects that depend on the swapchain or the window size.

```rust,noplaypen
unsafe fn recreate_swapchain(&mut self, window: &Window) -> Result<()> {
    self.device.device_wait_idle()?;
    create_swapchain(window, &self.instance, &self.device, &mut self.data)?;
    create_swapchain_image_views(&self.device, &mut self.data)?;
    create_render_pass(&self.instance, &self.device, &mut self.data)?;
    create_pipeline(&self.device, &mut self.data)?;
    create_framebuffers(&self.device, &mut self.data)?;
    create_command_buffers(&self.device, &mut self.data)?;
    self.data
        .images_in_flight
        .resize(self.data.swapchain_images.len(), vk::Fence::null());
    Ok(())
}
```

We first call `device_wait_idle`, because just like in the last chapter, we shouldn't touch resources that may still be in use. Obviously, the first thing we'll have to do is recreate the swapchain itself. The image views need to be recreated because they are based directly on the swapchain images. The render pass needs to be recreated because it depends on the format of the swapchain images. It is rare for the swapchain image format to change during an operation like a window resize, but it should still be handled. Viewport and scissor rectangle size is specified during graphics pipeline creation, so the pipeline also needs to be rebuilt. It is possible to avoid this by using dynamic state for the viewports and scissor rectangles. Then, the framebuffers and command buffers also directly depend on the swapchain images. Lastly we resize our list of fences for the swapchain images since there is a possibility that there might be a different number of swapchain images after recreation.

To make sure that the old versions of these objects are cleaned up before recreating them, we should move some of the cleanup code to a separate method that we can call from the `App::recreate_swapchain` method after waiting for the device to be idle. Let's call it `App::destroy_swapchain`:

```rust,noplaypen
unsafe fn recreate_swapchain(&mut self, window: &Window) -> Result<()> {
    self.device.device_wait_idle()?;
    self.destroy_swapchain();
    // ...
}

unsafe fn destroy_swapchain(&mut self) {

}
```

We'll move the cleanup code of all objects that are recreated as part of a swapchain refresh from `App::destroy` to `App::destroy_swapchain`:

```rust,noplaypen
unsafe fn destroy(&mut self) {
    self.destroy_swapchain();

    self.data.in_flight_fences
        .iter()
        .for_each(|f| self.device.destroy_fence(*f, None));
    self.data.render_finished_semaphores
        .iter()
        .for_each(|s| self.device.destroy_semaphore(*s, None));
    self.data.image_available_semaphores
        .iter()
        .for_each(|s| self.device.destroy_semaphore(*s, None));
    self.device.destroy_command_pool(self.data.command_pool, None);
    self.device.destroy_device(None);
    self.instance.destroy_surface_khr(self.data.surface, None);

    if VALIDATION_ENABLED {
        self.instance.destroy_debug_utils_messenger_ext(self.data.messenger, None);
    }

    self.instance.destroy_instance(None);
}

unsafe fn destroy_swapchain(&mut self) {
    self.data.framebuffers
        .iter()
        .for_each(|f| self.device.destroy_framebuffer(*f, None));
    self.device.free_command_buffers(self.data.command_pool, &self.data.command_buffers);
    self.device.destroy_pipeline(self.data.pipeline, None);
    self.device.destroy_pipeline_layout(self.data.pipeline_layout, None);
    self.device.destroy_render_pass(self.data.render_pass, None);
    self.data.swapchain_image_views
        .iter()
        .for_each(|v| self.device.destroy_image_view(*v, None));
    self.device.destroy_swapchain_khr(self.data.swapchain, None);
}
```

We could recreate the command pool from scratch, but that is rather wasteful. Instead I've opted to clean up the existing command buffers with the `free_command_buffers` command. This way we can reuse the existing pool to allocate the new command buffers.

That's all it takes to recreate the swapchain! However, the disadvantage of this approach is that we need to stop all rendering before creating the new swapchain. It is possible to create a new swapchain while drawing commands on an image from the old swapchain are still in-flight. You need to pass the previous swapchain to the `old_swapchain` field in the `vk::SwapchainCreateInfoKHR` struct and destroy the old swapchain as soon as you've finished using it.

## Suboptimal or out-of-date swapchain

Now we just need to figure out when swapchain recreation is necessary and call our new `App::recreate_swapchain` method. Luckily, Vulkan will usually just tell us that the swapchain is no longer adequate during presentation. The `acquire_next_image_khr` and `queue_present_khr` commands can return the following special values to indicate this.

* `vk::ErrorCode::OUT_OF_DATE_KHR` &ndash; The swapchain has become incompatible with the surface and can no longer be used for rendering. Usually happens after a window resize.
* `vk::SuccessCode::SUBOPTIMAL_KHR` &ndash; The swapchain can still be used to successfully present to the surface, but the surface properties are no longer matched exactly.

```rust,noplaypen
let result = self.device.acquire_next_image_khr(
    self.data.swapchain,
    u64::max_value(),
    self.data.image_available_semaphores[self.frame],
    vk::Fence::null(),
);

let image_index = match result {
    Ok((image_index, _)) => image_index as usize,
    Err(vk::ErrorCode::OUT_OF_DATE_KHR) => return self.recreate_swapchain(window),
    Err(e) => return Err(anyhow!(e)),
};
```

If the swapchain turns out to be out of date when attempting to acquire an image, then it is no longer possible to present to it. Therefore we should immediately recreate the swapchain and try again in the next `App::render` call.

You could also decide to do that if the swapchain is suboptimal, but I've chosen to proceed anyway in that case because we've already acquired an image. Since `vk::SuccessCode::SUBOPTIMAL_KHR` is considered a success code rather than an error code, it will be handled by the `Ok` arm in the `match` block.

```rust,noplaypen
let result = self.device.queue_present_khr(self.data.present_queue, &present_info);

let changed = result == Ok(vk::SuccessCode::SUBOPTIMAL_KHR)
    || result == Err(vk::ErrorCode::OUT_OF_DATE_KHR);

if changed {
    self.recreate_swapchain(window)?;
} else if let Err(e) = result {
    return Err(anyhow!(e));
}
```

The `queue_present_khr` function returns the same values with the same meaning. In this case we will also recreate the swapchain if it is suboptimal, because we want the best possible result.

## Handling resizes explicitly

Although many drivers and platforms trigger `vk::ErrorCode::OUT_OF_DATE_KHR` automatically after a window resize, it is not guaranteed to happen. That's why we'll add some extra code to also handle resizes explicitly. First add a new field to the `App` struct to track whether a resize has happpened:

```rust,noplaypen
struct App {
    // ...
    resized: bool,
}
```

Don't forget to initialize this new field to `false` in `App::create`. The `App::render` method should then be modified to also check for this flag after calling `queue_present_khr`:

```rust,noplaypen
let result = self.device.queue_present_khr(self.data.present_queue, &present_info);

let changed = result == Ok(vk::SuccessCode::SUBOPTIMAL_KHR)
    || result == Err(vk::ErrorCode::OUT_OF_DATE_KHR);

if self.resized || changed {
    self.resized = false;
    self.recreate_swapchain(window)?;
} else if let Err(e) = result {
    return Err(anyhow!(e));
}
```

It is important to do this after `queue_present_khr` to ensure that the semaphores are in a consistent state, otherwise a signalled semaphore may never be properly waited upon. Now to actually detect resizes we can add an arm to our window event `match` block in `main`:

```rust,noplaypen
match event {
    // ...
    Event::WindowEvent { event: WindowEvent::Resized(_), .. } => app.resized = true,
    // ...
}
```

Now try to run the program and resize the window to see if the framebuffer is indeed resized properly with the window.

## Handling minimization

There is another case where a swapchain may become out of data and that is a special kind of window resizing: window minimization. This case is special because it will result in a framebuffer size of `0`. In this tutorial we will handle that by not rendering frames while the window is minimized:

```rust,noplaypen
let mut app = unsafe { App::create(&window)? };
let mut destroying = false;
let mut minimized = false;
event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Poll;
    match event {
        Event::MainEventsCleared if !destroying && !minimized =>
            unsafe { app.render(&window) }.unwrap(),
        Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
            if size.width == 0 || size.height == 0 {
                minimized = true;
            } else {
                minimized = false;
                app.resized = true;
            }
        }
        // ...
    }
});
```

Congratulations, you've now finished your very first well-behaved Vulkan program! In the next chapter we're going to get rid of the hardcoded vertices in the vertex shader and actually use a vertex buffer.
