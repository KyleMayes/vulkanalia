# Image views

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/07_image_views.rs)

To use any `vk::Image`, including those in the swapchain, in the render pipeline we have to create a `vk::ImageView` object. An image view is quite literally a view into an image. It describes how to access the image and which part of the image to access, for example if it should be treated as a 2D texture depth texture without any mipmapping levels.

In this chapter we'll write a `create_swapchain_image_views` function that creates a basic image view for every image in the swapchain so that we can use them as color targets later on.

First add an `AppData` field to store the image views in:

```rust,noplaypen
struct AppData {
    // ...
    swapchain_image_views: Vec<vk::ImageView>,
}

```

Create the `create_swapchain_image_views` function and call it right after swapchain creation in `App::create`.

```rust,noplaypen
impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        // ...
        create_swapchain(window, &instance, &device, &mut data)?;
        create_swapchain_image_views(&device, &mut data)?;
        // ...
    }
}

unsafe fn create_swapchain_image_views(
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    Ok(())
}
```

What we next need to do is iterate over the swapchain images to create an image view for each:

```rust,noplaypen
unsafe fn create_swapchain_image_views(
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    data.swapchain_image_views = data
        .swapchain_images
        .iter()
        .map(|i| {

        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}
```

For each image view we are creating we'll first need to define the color component mapping for the image view. This allows you to swizzle the color channels around. For example, you can map all of the channels to the red channel for a monochrome texture. You can also map constant values of `0` and `1` to a channel. In our case we'll stick to the default mapping.

```rust,noplaypen
let components = vk::ComponentMapping::builder()
    .r(vk::ComponentSwizzle::IDENTITY)
    .g(vk::ComponentSwizzle::IDENTITY)
    .b(vk::ComponentSwizzle::IDENTITY)
    .a(vk::ComponentSwizzle::IDENTITY);
```

Next we will define the subresource range for the image view which describes the image's purpose and which part of the image should be accessed. Our images will be used as color targets without any mipmapping levels or multiple layers.

```rust,noplaypen
let subresource_range = vk::ImageSubresourceRange::builder()
    .aspect_mask(vk::ImageAspectFlags::COLOR)
    .base_mip_level(0)
    .level_count(1)
    .base_array_layer(0)
    .layer_count(1);
```

If you were working on a stereographic 3D application, then you would create a swapchain with multiple layers. You could then create multiple image views for each image representing the views for the left and right eyes by accessing different layers.

We can now create a `vk::ImageViewCreateInfo` struct which provides the parameters for image view creation.

```rust,noplaypen
let info = vk::ImageViewCreateInfo::builder()
    .image(*i)
    .view_type(vk::ImageViewType::_2D)
    .format(data.swapchain_format)
    .components(components)
    .subresource_range(subresource_range);
```

The `view_type` and `format` fields specify how the image data should be interpreted. The `view_type` field allows you to treat images as 1D textures, 2D textures, 3D textures, and cube maps.

Creating the image view is now a matter of calling `create_image_view`:

```rust,noplaypen
device.create_image_view(&info, None)
```

Unlike images, the image views were explicitly created by us, so we need to add a similar loop to destroy them again in `App::destroy`:

```rust,noplaypen
unsafe fn destroy(&mut self) {
    self.data.swapchain_image_views
        .iter()
        .for_each(|v| self.device.destroy_image_view(*v, None));
    // ...
}
```

An image view is sufficient to start using an image as a texture, but it's not quite ready to be used as a render target just yet. That requires one more step of indirection, known as a framebuffer. But first we'll have to set up the graphics pipeline.
