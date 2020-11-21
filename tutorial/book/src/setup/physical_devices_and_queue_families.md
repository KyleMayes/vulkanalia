# Physical devices and queue families

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/03_physical_device_selection.rs)

After initializing the Vulkan library through an `Instance` we need to look for and select a graphics card in the system that supports the features we need. In fact we can select any number of graphics cards and use them simultaneously, but in this tutorial we'll stick to the first graphics card that suits our needs.

We'll add a `pick_physical_device` function which will accomplish this task and write the physical device and related information to the `AppData` instance. With this in place we can construct and populate an `AppData` instance in our `App::create` method.

```rust,noplaypen
impl App {
    fn create(window: &Window) -> Result<Self> {
        // ...
        pick_physical_device(&instance, &mut data)?;
        Ok(Self { entry, instance, data })
    }
}

fn pick_physical_device(instance: &Instance, data: &mut AppData) -> Result<()> {
    Ok(())
}
```

The graphics card that we'll end up selecting will be stored in a `vk::PhysicalDevice` handle that is added as a new field to the `AppData` struct. This object will be implicitly destroyed when the `Instance` is destroyed, so we won't need to do anything new in the `App::destroy` method.

```rust,noplaypen
struct AppData {
    physical_device: vk::PhysicalDevice,
}
```

## Device suitability

We'll need a way to determine whether a physical device meets our needs. We'll start by creating a function that returns whether a supplied physical device supports everything we require:

```rust,noplaypen
fn check_physical_device(
    instance: &Instance,
    data: &AppData,
    physical_device: vk::PhysicalDevice,
) -> Result<bool> {
    Ok(true)
}
```

To evaluate whether a physical device meets our needs we can start by querying for some details. Basic device properties like the name, type, and supported Vulkan version can be queried using `get_physical_device_properties`:

```rust,noplaypen
let properties = instance
    .get_physical_device_properties(physical_device);
```

The support for optional features like texture compression, 64 bit floats, and multi-viewport rendering (useful for VR) can be queried using `get_physical_device_features`:

```rust,noplaypen
let features = instance
    .get_physical_device_features(physical_device);
```

There are more details that can be queried from devices that we'll discuss later concerning device memory and queue families (see the next section).

As an example, let's say we consider our application only usable for dedicated graphics cards that support geometry shaders. Then the `check_physical_device` function would look like this:

```rust,noplaypen
fn check_physical_device(
    instance: &Instance,
    data: &AppData,
    physical_device: vk::PhysicalDevice,
) -> Result<bool> {
    let properties = instance.get_physical_device_properties(physical_device);
    let features = instance.get_physical_device_features(physical_device);
    Ok(
        properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
            && features.geometry_shader == vk::TRUE,
    )
}
```

Instead of just checking if a device is suitable or not and going with the first one, you could also give each device a score and pick the highest one. That way you could favor a dedicated graphics card by giving it a higher score, but fall back to an integrated GPU if that's the only available one. You could also just display the names of the choices and allow the user to select.

Next we'll discuss the first real required feature.

## Queue families

It has been briefly touched upon before that almost every operation in Vulkan, anything from drawing to uploading textures, requires commands to be submitted to a queue. There are different types of queues that originate from different queue families and each family of queues allows only a subset of commands. For example, there could be a queue family that only allows processing of compute commands or one that only allows memory transfer related commands.

We need to check which queue families are supported by the device and which one of these supports the commands that we want to use. For that purpose we'll add a new struct `QueueFamilyIndices` that stores the indices of the queue families we need.

Right now we are only going to look for a queue that supports graphics commands, so the struct and its implementation will look like this:

```rust,noplaypen
#[derive(Copy, Clone, Debug)]
struct QueueFamilyIndices {
    graphics: u32,
}

impl QueueFamilyIndices {
    fn get(
        instance: &Instance,
        data: &AppData,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Self> {
        let properties = instance
            .get_physical_device_queue_family_properties(physical_device);

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        if let Some(graphics) = graphics {
            Ok(Self { graphics })
        } else {
            Err(anyhow!("Failed to get required queue family indices."))
        }
    }
}
```

The queue properties returned by `get_physical_device_queue_family_properties` contains various details about the queue families supported by the physical device, including the type of operations supported and the number of queues that can be created based on that family. Here we are looking for the first queue family that supports graphics operations as indicated by `vk::QueueFlags::GRAPHICS`.

Now that we have this fancy queue family lookup method, we can use it as a check in the `check_physical_device` function to ensure the device can process the commands we want to use:

```rust,noplaypen
fn check_physical_device(
    instance: &Instance,
    data: &AppData,
    physical_device: vk::PhysicalDevice,
) -> Result<bool> {
    QueueFamilyIndices::get(instance, data, physical_device)?;
    Ok(true)
}
```

Lastly we can iterate over the physical devices and pick the first that satisfies our requirements as indicated by `check_physical_device`. To do this, add the following code to `pick_physical_device`:

```rust,noplaypen
data.physical_device = instance
    .enumerate_physical_devices()?
    .into_iter()
    .find(|pd| match check_physical_device(instance, data, *pd) {
        Ok(suitable) => suitable,
        Err(e) => {
            warn!("Failed to check physical device suitability: {}", e);
            false
        }
    })
    .ok_or_else(|| anyhow!("Failed to find suitable physical device."))?;
```

Great, that's all we need for now to find the right physical device! The next step is to create a logical device to interface with it.
