# Window surface

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/05_window_surface.rs)

Since Vulkan is a platform agnostic API, it can't interface directly with the window system on its own. To establish the connection between Vulkan and the window system to present results to the screen, we need to use the WSI (Window System Integration) extensions. In this chapter we'll discuss the first one, which is `VK_KHR_surface`. It exposes a `vk::SurfaceKHR` object that represents an abstract type of surface to present rendered images to. The surface in our program will be backed by the window that we've already opened with `winit`.

The `VK_KHR_surface` extension is an instance level extension and we've actually already enabled it, because it's included in the list returned by `vk_window::get_required_instance_extensions`. The list also includes some other WSI extensions that we'll use in the next couple of chapters.

The window surface needs to be created right after the instance creation, because it can actually influence the physical device selection. The reason we postponed this is because window surfaces are part of the larger topic of render targets and presentation for which the explanation would have cluttered the basic setup. It should also be noted that window surfaces are an entirely optional component in Vulkan, if you just need off-screen rendering. Vulkan allows you to do that without hacks like creating an invisible window (necessary for OpenGL).

While we can freely import types for extensions like the struct `vk::SurfaceKHR`, we need to import the `vulkanalia` extension trait for `VK_KHR_surface` before we can call any of the Vulkan commands added by the extension. Add the following import for `vk::KhrSurfaceExtension`:

```rust,noplaypen
use vulkanalia::vk::KhrSurfaceExtension;
```

## Window surface creation

Start by adding a `surface` field in `AppData` above the other fields.

```rust,noplaypen
struct AppData {
    surface: vk::SurfaceKHR,
    // ...
}
```

Although the `vk::SurfaceKHR` object and its usage is platform agnostic, its creation isn't because it depends on window system details. For example, it needs the `HWND` and `HMODULE` handles on Windows. Therefore there is a platform-specific addition to the extension, which on Windows is called `VK_KHR_win32_surface` and is also automatically included in the list from `vk_window::get_required_instance_extensions`.

I will demonstrate how this platform specific extension can be used to create a surface on Windows, but we won't actually use it in this tutorial. `vulkanalia` has `vk_window::create_surface` that handles the platform differences for us. Still, it's good to see what it does behind the scenes before we start relying on it.

Because a window surface is a Vulkan object, it comes with a `vk::Win32SurfaceCreateInfoKHR` struct that needs to be filled in. It has two important parameters: `hinstance` and `hwnd`. These are the handles to the process and the window.

```rust,noplaypen
use winit::platform::windows::WindowExtWindows;

let info = vk::Win32SurfaceCreateInfoKHR::builder()
    .hinstance(window.hinstance())
    .hwnd(window.hwnd());
```

The `WindowExtWindows` trait is imported from `winit` because it allows us to access platform-specific methods on the `winit` `Window` struct. In this case, it permits us to get the process and window handles for the window created by `winit`.

After that the surface can be created with `create_win32_surface_khr`, which includes parameters for the surface creation details and custom allocators. Technically this is a WSI extension function, but it is so commonly used that the standard Vulkan loader includes it, so unlike other extensions you don't need to explicitly load it. However, we do need to import the `vulkanalia` extension trait for `VK_KHR_win32_surface` (`vk::KhrWin32SurfaceExtension`).

```rust,noplaypen
use vk::KhrWin32SurfaceExtension;

let surface = instance.create_win32_surface_khr(&info, None).unwrap();
```

The process is similar for other platforms like Linux, where `create_xcb_surface_khr` takes an XCB connection and window as creation details with X11.

The `vk_window::create_surface` function performs exactly this operation with a different implementation for each platform. We'll now integrate it into our program. Add a call to the function in `App::create` right before we pick a physical device.

```rust,noplaypen
unsafe fn create(window: &Window) -> Result<Self> {
    // ...
    let instance = create_instance(window, &entry, &mut data)?;
    data.surface = vk_window::create_surface(&instance, &window, &window)?;
    pick_physical_device(&instance, &mut data)?;
    // ...
}
```

The parameters are the Vulkan instance and the `winit` window. Once we have our surface, it can be destroyed in `App::destroy` using the Vulkan API:

```rust,noplaypen
unsafe fn destroy(&mut self) {
    // ...
    self.instance.destroy_surface_khr(self.data.surface, None);
    self.instance.destroy_instance(None);
}
```

Make sure that the surface is destroyed before the instance.

## Querying for presentation support

Although the Vulkan implementation may support window system integration, that does not mean that every device in the system supports it. Therefore we need to extend the `QueueFamilyIndices` struct to ensure that a device can present images to the surface we created. Since the presentation is a queue-specific feature, the problem is actually about finding a queue family that supports presenting to the surface we created.

It's actually possible that the queue families supporting drawing commands and the ones supporting presentation do not overlap. Therefore we have to take into account that there could be a distinct presentation queue by modifying the `QueueFamilyIndices` struct:

```rust,noplaypen
struct QueueFamilyIndices {
    graphics: u32,
    present: u32,
}
```

Next, we'll modify the `QueueFamilyIndices::get` method to look for a queue family that has the capability of presenting to our window surface. The function to check for that is `get_physical_device_surface_support_khr`, which takes the physical device, queue family index. and surface as parameters and returns whether presentation is supported for that combination of physical device, queue family, and surface.

Modify `QueueFamilyIndices::get` to find a presentation queue family index below where a graphics queue family index is found.

```rust,noplaypen
let mut present = None;
for (index, properties) in properties.iter().enumerate() {
    if instance.get_physical_device_surface_support_khr(
        physical_device,
        index as u32,
        data.surface,
    )? {
        present = Some(index as u32);
        break;
    }
}
```

We'll also need to add `present` to the final expression:

```rust,noplaypen
if let (Some(graphics), Some(present)) = (graphics, present) {
    Ok(Self { graphics, present })
} else {
    Err(anyhow!(SuitabilityError("Missing required queue families.")))
}
```

Note that it's very likely that these end up being the same queue family after all, but throughout the program we will treat them as if they were separate queues for a uniform approach. Nevertheless, you could add logic to explicitly prefer a physical device that supports drawing and presentation in the same queue for improved performance.

## Creating the presentation queue

The one thing that remains is modifying the logical device creation procedure to create the presentation queue and retrieve the `vk::Queue` handle. Add a field to `AppData` for the handle:

```rust,noplaypen
struct AppData {
    // ...
    present_queue: vk::Queue,
}
```

Next, we need to have multiple `vk::DeviceQueueCreateInfo` structs to create a queue from both families. An easy way to do that is to create a set of all unique queue families that are necessary for the required queues. We'll do this in the `create_logical_device` function:

```rust,noplaypen
let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

let mut unique_indices = HashSet::new();
unique_indices.insert(indices.graphics);
unique_indices.insert(indices.present);

let queue_priorities = &[1.0];
let queue_infos = unique_indices
    .iter()
    .map(|i| {
        vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(*i)
            .queue_priorities(queue_priorities)
    })
    .collect::<Vec<_>>();
```

And delete the previous `queue_infos` slice and take a reference to the `queue_infos` list for `vk::DeviceCreateInfo`:

```rust,noplaypen
let info = vk::DeviceCreateInfo::builder()
    .queue_create_infos(&queue_infos)
    .enabled_layer_names(&layers)
    .enabled_features(&features);
```

If the queue families are the same, then we only need to pass its index once. Finally, add a call to retrieve the queue handle:

```rust,noplaypen
data.present_queue = device.get_device_queue(indices.present, 0);
```

In case the queue families are the same, the two handles will most likely have the same value now. In the next chapter we're going to look at swapchains and how they give us the ability to present images to the surface.
