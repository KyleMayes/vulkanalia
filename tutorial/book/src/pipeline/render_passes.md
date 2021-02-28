# Render passes

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/11_render_passes.rs)

Before we can finish creating the pipeline, we need to tell Vulkan about the framebuffer attachments that will be used while rendering. We need to specify how many color and depth buffers there will be, how many samples to use for each of them and how their contents should be handled throughout the rendering operations. All of this information is wrapped in a *render pass* object, for which we'll create a new `^create_render_pass` function. Call this function from `App::create` before `create_pipeline`.

```rust,noplaypen
impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        // ...
        create_render_pass(&instance, &device, &mut data)?;
        create_pipeline(&device, &mut data)?;
        // ...
    }
}

unsafe fn create_render_pass(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    Ok(())
}
```

## Attachment description

In our case we'll have just a single color buffer attachment represented by one of the images from the swapchain. This will be represented by a `vk::AttachmentDescription` which we will build in `^create_render_pass`.

```rust,noplaypen
let color_attachment = vk::AttachmentDescription::builder()
    .format(data.swapchain_format)
    .samples(vk::SampleCountFlags::_1)
    // continued...
```

The `format` of the color attachment should match the format of the swapchain images, and we're not doing anything with multisampling yet, so we'll stick to 1 sample.

```rust,noplaypen
    .load_op(vk::AttachmentLoadOp::CLEAR)
    .store_op(vk::AttachmentStoreOp::STORE)
```

The `load_op` and `store_op` determine what to do with the data in the attachment before rendering and after rendering. We have the following choices for `load_op`:

* `vk::AttachmentLoadOp::LOAD` &ndash; Preserve the existing contents of the attachment
* `vk::AttachmentLoadOp::CLEAR` &ndash; Clear the values to a constant at the start
* `vk::AttachmentLoadOp::DONT_CARE` &ndash; Existing contents are undefined; we don't care about them

In our case we're going to use the clear operation to clear the framebuffer to black before drawing a new frame. There are only two possibilities for the `store_op`:

* `vk::AttachmentStoreOp::STORE` &ndash; Rendered contents will be stored in memory and can be read later
* `vk::AttachmentStoreOp::DONT_CARE` &ndash; Contents of the framebuffer will be undefined after the rendering operation

We're interested in seeing the rendered triangle on the screen, so we're going with the store operation here.

```rust,noplaypen
    .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
    .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
```

The `load_op` and `store_op` apply to color and depth data, and `stencil_load_op` / `stencil_store_op` apply to stencil data. Our application won't do anything with the stencil buffer, so the results of loading and storing are irrelevant.

```rust,noplaypen
    .initial_layout(vk::ImageLayout::UNDEFINED)
    .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);
```

Textures and framebuffers in Vulkan are represented by `vk::Image` objects with a certain pixel format, however the layout of the pixels in memory can change based on what you're trying to do with an image.

Some of the most common layouts are:

* `vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL` &ndash; Images used as color attachment
* `vk::ImageLayout::PRESENT_SRC_KHR` &ndash; Images to be presented in the swapchain
* `vk::ImageLayout::TRANSFER_DST_OPTIMAL` &ndash; Images to be used as destination for a memory copy operation

We'll discuss this topic in more depth in the texturing chapter, but what's important to know right now is that images need to be transitioned to specific layouts that are suitable for the operation that they're going to be involved in next.

The `initial_layout` specifies which layout the image will have before the render pass begins. The `final_layout` specifies the layout to automatically transition to when the render pass finishes. Using `vk::ImageLayout::UNDEFINED` for `initial_layout` means that we don't care what previous layout the image was in. The caveat of this special value is that the contents of the image are not guaranteed to be preserved, but that doesn't matter since we're going to clear it anyway. We want the image to be ready for presentation using the swapchain after rendering, which is why we use `vk::ImageLayout::PRESENT_SRC_KHR` as `final_layout`.

## Subpasses and attachment references

A single render pass can consist of multiple subpasses. Subpasses are subsequent rendering operations that depend on the contents of framebuffers in previous passes, for example a sequence of post-processing effects that are applied one after another. If you group these rendering operations into one render pass, then Vulkan is able to reorder the operations and conserve memory bandwidth for possibly better performance. For our very first triangle, however, we'll stick to a single subpass.

Every subpass references one or more of the attachments that we've described using the structure in the previous sections. These references are themselves `vk::AttachmentReference` structs that look like this:

```rust,noplaypen
let color_attachment_ref = vk::AttachmentReference::builder()
    .attachment(0)
    .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
```

The `attachment` parameter specifies which attachment to reference by its index in the attachment descriptions array. Our array consists of a single `vk::AttachmentDescription`, so its index is `0`. The `layout` specifies which layout we would like the attachment to have during a subpass that uses this reference. Vulkan will automatically transition the attachment to this layout when the subpass is started. We intend to use the attachment to function as a color buffer and the `vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL` layout will give us the best performance, as its name implies.

The subpass is described using a `vk::SubpassDescription` structure:

```rust,noplaypen
let color_attachments = &[color_attachment_ref];
let subpass = vk::SubpassDescription::builder()
    .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
    .color_attachments(color_attachments);
```

Vulkan may also support compute subpasses in the future, so we have to be explicit about this being a graphics subpass. Then we specify the reference to the color attachment.

The index of the attachment in this array is directly referenced from the fragment shader with the `layout(location = 0) out vec4 outColor` directive!

The following other types of attachments can be referenced by a subpass:

* `input_attachments` &ndash; Attachments that are read from a shader
* `resolve_attachments` &ndash; Attachments used for multisampling color attachments
* `depth_stencil_attachment` &ndash; Attachment for depth and stencil data
* `preserve_attachments` &ndash; Attachments that are not used by this subpass, but for which the data must be preserved

## Render pass

Now that the attachment and a basic subpass referencing it have been described, we can create the render pass itself. Create a new class member variable to hold the `vk::RenderPass` object right above the `pipeline_layout` field in `AppData`:

```rust,noplaypen
struct AppData {
    // ...
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
}
```

The render pass object can then be created by filling in the `vk::RenderPassCreateInfo` structure with an array of attachments and subpasses. The `vk::AttachmentReference` objects reference attachments using the indices of this array.

```rust,noplaypen
let attachments = &[color_attachment];
let subpasses = &[subpass];
let info = vk::RenderPassCreateInfo::builder()
    .attachments(attachments)
    .subpasses(subpasses);

data.render_pass = device.create_render_pass(&info, None)?;
```

Just like the pipeline layout, the render pass will be referenced throughout the program, so it should only be cleaned up at the end in `App::destroy`:

```rust,noplaypen
unsafe fn destroy(&mut self) {
    self.device.destroy_pipeline_layout(self.data.pipeline_layout, None);
    self.device.destroy_render_pass(self.data.render_pass, None);
    // ...
}
```

That was a lot of work, but in the next chapter it all comes together to finally create the graphics pipeline object!
