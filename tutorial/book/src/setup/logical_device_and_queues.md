# Logical device and queues

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/04_logical_device.rs)

After selecting a physical device to use we need to set up a logical device to interface with it. The logical device creation process is similar to the instance creation process and describes the features we want to use. We also need to specify which queues to create now that we've queried which queue families are available. You can even create multiple logical devices from the same physical device if you have varying requirements.

Start by adding a new `App` field to store the logical device in:

```rust,noplaypen
struct App {
    // ...
    device: Device,
}
```

Next, add a `create_logical_device` function that is called from `App:create` and add the resulting logical device to the `App` initializer:

```rust,noplaypen
impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        // ...
        let device = create_logical_device(&entry, &instance, &mut data)?;
        Ok(Self { entry, instance, data, device })
    }
}

unsafe fn create_logical_device(
    entry: &Entry,
    instance: &Instance,
    data: &mut AppData,
) -> Result<Device> {
}
```

## Specifying the queues to be created

The creation of a logical device involves specifying a bunch of details in structs again, of which the first one will be `vk::DeviceQueueCreateInfo`. This structure describes the number of queues we want for a single queue family. Right now we're only interested in a queue with graphics capabilities.

```rust,noplaypen
let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

let queue_priorities = &[1.0];
let queue_info = vk::DeviceQueueCreateInfo::builder()
    .queue_family_index(indices.graphics)
    .queue_priorities(queue_priorities);
```

The currently available drivers will only allow you to create a small number of queues for each queue family and you don't really need more than one. That's because you can create all of the command buffers on multiple threads and then submit them all at once on the main thread with a single low-overhead call.

Vulkan lets you assign priorities to queues to influence the scheduling of command buffer execution using floating point numbers between `0.0` and `1.0`. This is required even when only creating a single queue.

## Specifying the layers to enable

The next piece of information we need to provide bears a resemblance to the `vk::InstanceCreateInfo` struct. Once again we need to specify any layers or extensions we want to enable, but this time any specified extensions are device specific rather than global.

An example of a device specific extension is `VK_KHR_swapchain`, which allows you to present rendered images from that device to windows. It is possible that there are Vulkan devices in the system that lack this ability, for example because they only support compute operations. We will come back to this extension in the swapchain chapter.

Previous implementations of Vulkan made a distinction between instance and device specific validation layers, but this is no longer the case. That means that the layer names we pass to `enabled_layer_names` later are ignored by up-to-date implementations. However, it is still a good idea to set them anyway to be compatible with older implementations.

We wont be enabling any device extensions yet, so we will just construct a list of layer names containing the validation layer if validation is enabled.

```rust,noplaypen
let layers = if VALIDATION_ENABLED {
    vec![VALIDATION_LAYER.as_ptr()]
} else {
    vec![]
};
```

## Specifying the extensions to enable

As discussed in the `Instance` chapter, certain Vulkan extensions must be enabled for applications that use Vulkan implementations that aren't fully conformant with the Vulkan specification. In that chapter, we enabled the instance extensions needed for compatibility with these non-conformant implementations. Here, we'll enable the device extension needed for the same purpose.

```rust,noplaypen
let mut extensions = vec![];

// Required by Vulkan SDK on macOS since 1.3.216.
if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
    extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
}
```

## Specifying used device features

The next information to specify is the set of device features that we'll be using. These are the features that we queried support for with `get_physical_device_features` in the previous chapter, like geometry shaders. Right now we don't need anything special, so we can simply define it and leave everything to the default values (`false`). We'll come back to this structure once we're about to start doing more interesting things with Vulkan.

```rust,noplaypen
let features = vk::PhysicalDeviceFeatures::builder();
```

## Creating the logical device

With the previous two structures, the validation layer (if enabled), and the device extensions in place, we can fill in the main `vk::DeviceCreateInfo` structure.

```rust,noplaypen
let queue_infos = &[queue_info];
let info = vk::DeviceCreateInfo::builder()
    .queue_create_infos(queue_infos)
    .enabled_layer_names(&layers)
    .enabled_extension_names(&extensions)
    .enabled_features(&features);
```

That's it, we're now ready to instantiate the logical device with a call to the appropriately named `create_device` method.

```rust,noplaypen
let device = instance.create_device(data.physical_device, &info, None)?;
```

The parameters are the physical device to interface with, the queue and usage info we just specified, and the optional allocation callbacks. Similarly to the instance creation function, this call can return errors based on enabling non-existent extensions or specifying the desired usage of unsupported features.

The device should be destroyed in `App::destroy`:

```rust,noplaypen
unsafe fn destroy(&mut self) {
    self.device.destroy_device(None);
    // ...
}
```

Logical devices don't interact directly with instances, which is why it's not included as a parameter.

## Retrieving queue handles

The queues are automatically created along with the logical device, but we don't have a handle to interface with them yet. First add a new `AppData` field to store a handle to the graphics queue:

```rust,noplaypen
struct AppData {
    // ...
    graphics_queue: vk::Queue,
}
```

Device queues are implicitly cleaned up when the device is destroyed, so we don't need to do anything in `App::destroy`.

We can use the `get_device_queue` function to retrieve queue handles for each queue family. The parameters are the logical device, queue family, and queue index. Because we're only creating a single queue from this family, we'll simply use index 0.

```rust,noplaypen
data.graphics_queue = device.get_device_queue(indices.graphics, 0);
```

Lastly, return the created logical device from `create_logical_device`:

```rust,noplaypen
Ok(device)
```

With the logical device and queue handles we can now actually start using the graphics card to do things! In the next few chapters we'll set up the resources to present results to the window system.
