# Framebuffers

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/13_framebuffers.rs)

We've talked a lot about framebuffers in the past few chapters and we've set up the render pass to expect a single framebuffer with the same format as the swapchain images, but we haven't actually created any yet.

The attachments specified during render pass creation are bound by wrapping them into a `vk::Framebuffer` object. A framebuffer object references all of the `vk::ImageView` objects that represent the attachments. In our case that will be only a single one: the color attachment. However, the image that we have to use for the attachment depends on which image the swapchain returns when we retrieve one for presentation. That means that we have to create a framebuffer for all of the images in the swapchain and use the one that corresponds to the retrieved image at drawing time.

To that end, create another `Vec` field in `AppData` to hold the framebuffers:

```rust,noplaypen
struct AppData {
    // ...
    framebuffers: Vec<vk::Framebuffer>,
}
```

We'll create the objects for this array in a new function `create_framebuffers` that is called from `App::create` right after creating the graphics pipeline:

```rust,noplaypen
impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        // ...
        create_pipeline(&device, &mut data)?;
        create_framebuffers(&device, &mut data)?;
        // ...
    }
}

unsafe fn create_framebuffers(device: &Device, data: &mut AppData) -> Result<()> {
    Ok(())
}
```

Start by mapping over the swapchain image views:

```rust,noplaypen
unsafe fn create_framebuffers(device: &Device, data: &mut AppData) -> Result<()> {
    data.framebuffers = data
        .swapchain_image_views
        .iter()
        .map(|i| {

        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}
```

We'll then create a framebuffer for each image view:

```rust,noplaypen
let attachments = &[*i];
let create_info = vk::FramebufferCreateInfo::builder()
    .render_pass(data.render_pass)
    .attachments(attachments)
    .width(data.swapchain_extent.width)
    .height(data.swapchain_extent.height)
    .layers(1);

device.create_framebuffer(&create_info, None)
```

As you can see, creation of framebuffers is quite straightforward. We first need to specify with which `render_pass` the framebuffer needs to be compatible. You can only use a framebuffer with the render passes that it is compatible with, which roughly means that they use the same number and type of attachments.

The `attachments` field specifies the `vk::ImageView` objects that should be bound to the respective attachment descriptions in the render pass `attachment` array.

The `width` and `height` parameters are self-explanatory and `layers` refers to the number of layers in image arrays. Our swapchain images are single images, so the number of layers is `1`.

We should delete the framebuffers before the image views and render pass that they are based on, but only after we've finished rendering:

```rust,noplaypen
unsafe fn destroy(&mut self) {
    self.data.framebuffers
        .iter()
        .for_each(|f| self.device.destroy_framebuffer(*f, None));
    // ...
}
```

We've now reached the milestone where we have all of the objects that are required for rendering. In the next chapter we're going to write the first actual drawing commands.
