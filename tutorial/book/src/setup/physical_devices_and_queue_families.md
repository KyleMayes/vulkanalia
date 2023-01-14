# Physical devices and queue families

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/03_physical_device_selection.rs)

After initializing the Vulkan library through an `Instance` we need to look for and select a graphics card in the system that supports the features we need. In fact we can select any number of graphics cards and use them simultaneously, but in this tutorial we'll stick to the first graphics card that suits our needs.

We'll add a `pick_physical_device` function which will accomplish this task and write the physical device and related information to the `AppData` instance. This function and the functions it calls will use a custom error type (`SuitabilityError`) to signal that a physical device does not satisfy the requirements of the application. This error type will use the `thiserror` crate to automatically implement all the necessary boilerplate for an error type.

```rust,noplaypen
use thiserror::Error;

impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        // ...
        pick_physical_device(&instance, &mut data)?;
        Ok(Self { entry, instance, data })
    }
}

#[derive(Debug, Error)]
#[error("Missing {0}.")]
pub struct SuitabilityError(pub &'static str);

unsafe fn pick_physical_device(instance: &Instance, data: &mut AppData) -> Result<()> {
    Ok(())
}
```

The graphics card that we'll end up selecting will be stored in a `vk::PhysicalDevice` handle that is added as a new field to the `AppData` struct. This object will be implicitly destroyed when the `Instance` is destroyed, so we won't need to do anything new in the `App::destroy` method.

```rust,noplaypen
struct AppData {
    // ...
    physical_device: vk::PhysicalDevice,
}
```

## Device suitability

We'll need a way to determine whether a physical device meets our needs. We'll start by creating a function that returns a `SuitabilityError` if a supplied physical device does not support everything we require:

```rust,noplaypen
unsafe fn check_physical_device(
    instance: &Instance,
    data: &AppData,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    Ok(())
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

As an example, let's say we consider our application only usable for dedicated graphics cards that support geometry shaders. Then the `check_physical_device` function might look like this:

```rust,noplaypen
unsafe fn check_physical_device(
    instance: &Instance,
    data: &AppData,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    let properties = instance.get_physical_device_properties(physical_device);
    if properties.device_type != vk::PhysicalDeviceType::DISCRETE_GPU {
        return Err(anyhow!(SuitabilityError("Only discrete GPUs are supported.")));
    }

    let features = instance.get_physical_device_features(physical_device);
    if features.geometry_shader != vk::TRUE {
        return Err(anyhow!(SuitabilityError("Missing geometry shader support.")));
    }

    Ok(())
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
    unsafe fn get(
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
            Err(anyhow!(SuitabilityError("Missing required queue families.")))
        }
    }
}
```

The queue properties returned by `get_physical_device_queue_family_properties` contains various details about the queue families supported by the physical device, including the type of operations supported and the number of queues that can be created based on that family. Here we are looking for the first queue family that supports graphics operations as indicated by `vk::QueueFlags::GRAPHICS`.

Now that we have this fancy queue family lookup method, we can use it as a check in the `check_physical_device` function to ensure the device can process the commands we want to use:

```rust,noplaypen
unsafe fn check_physical_device(
    instance: &Instance,
    data: &AppData,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    QueueFamilyIndices::get(instance, data, physical_device)?;
    Ok(())
}
```

Lastly we can iterate over the physical devices and pick the first that satisfies our requirements as indicated by `check_physical_device`. To do this, update `pick_physical_device` to look like the following:

```rust,noplaypen
unsafe fn pick_physical_device(instance: &Instance, data: &mut AppData) -> Result<()> {
    for physical_device in instance.enumerate_physical_devices()? {
        let properties = instance.get_physical_device_properties(physical_device);

        if let Err(error) = check_physical_device(instance, data, physical_device) {
            warn!("Skipping physical device (`{}`): {}", properties.device_name, error);
        } else {
            info!("Selected physical device (`{}`).", properties.device_name);
            data.physical_device = physical_device;
            return Ok(());
        }
    }

    Err(anyhow!("Failed to find suitable physical device."))
}
```

Great, that's all we need for now to find the right physical device! The next step is to create a logical device to interface with it.
