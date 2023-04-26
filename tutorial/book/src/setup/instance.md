# Instance

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/01_instance_creation.rs)

The very first thing you will want to do is initialize the Vulkan library by creating an *instance*. The instance is the connection between your application and the Vulkan library and creating it involves specifying some details about your application to the driver. To get started, add the following imports:

```rust,noplaypen
use anyhow::{anyhow, Result};
use log::*;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::window as vk_window;
use vulkanalia::prelude::v1_0::*;
```

Here we first add the [`anyhow!`](https://docs.rs/anyhow/latest/anyhow/macro.anyhow.html) macro to our imports from `anyhow`. This macro will be used to easily construct instances of `anyhow` errors. Then, we import `log::*` so we can use the logging macros from the `log` crate. Next, we import `LibloadingLoader` which serves as `vulkanalia`'s `libloading` integration which we will use to load the initial Vulkan commands from the Vulkan shared library. The standard name of the Vulkan shared library on your operating system (e.g., `vulkan-1.dll` on Windows) is then imported as `LIBRARY`.

Next we import `vulkanalia`'s window integration as `vk_window` which in this chapter we will use to enumerate the global Vulkan extensions required to render to a window. In a future chapter we will also use `vk_window` to link our Vulkan instance with our `winit` window.

Lastly we import the Vulkan 1.0 prelude from `vulkanalia` which will provide all of the other Vulkan-related imports we will need for this and future chapters.

Now, to create an instance we'll next have to fill in a struct with some information about our application. This data is technically optional, but it may provide some useful information to the driver in order to optimize our specific application (e.g., because it uses a well-known graphics engine with certain special behavior). This struct is called `vk::ApplicationInfo` and we'll create it in a new function called `^create_instance` that takes our window and a Vulkan entry point (which we will create later) and returns a Vulkan instance:

```rust,noplaypen
unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance> {
    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"Vulkan Tutorial\0")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"No Engine\0")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));
}
```

A lot of information in Vulkan is passed through structs instead of function parameters and we'll have to fill in one more struct to provide sufficient information for creating an instance. This next struct is not optional and tells the Vulkan driver which global extensions and validation layers we want to use. Global here means that they apply to the entire program and not a specific device, which will become clear in the next few chapters. First we'll need to use `vulkanalia`'s window integration to enumerate the required global extensions and convert them into null-terminated C strings (`*const c_char`):

```rust,noplaypen
let extensions = vk_window::get_required_instance_extensions(window)
    .iter()
    .map(|e| e.as_ptr())
    .collect::<Vec<_>>();
```

With our list of required global extensions in hand we can create and return a Vulkan instance using the Vulkan entry point passed into this function:

```rust,noplaypen
let info = vk::InstanceCreateInfo::builder()
    .application_info(&application_info)
    .enabled_extension_names(&extensions);

Ok(entry.create_instance(&info, None)?)
```

As you'll see, the general pattern that object creation function parameters in Vulkan follow is:

* Reference to struct with creation info
* Optional reference to custom allocator callbacks, always `None` in this tutorial

Now that we have a function to create Vulkan instances from entry points, we next need to create a Vulkan entry point. This entry point will load the Vulkan commands used to query instance support and create instances. But before we do that, let's add some fields to our `App` struct to store the Vulkan entry point and instance we will be creating:

```rust,noplaypen
struct App {
    entry: Entry,
    instance: Instance,
}
```

To populate these fields, update the `App::create` method to the following:

```rust,noplaypen
unsafe fn create(window: &Window) -> Result<Self> {
    let loader = LibloadingLoader::new(LIBRARY)?;
    let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
    let instance = create_instance(window, &entry)?;
    Ok(Self { entry, instance })
}
```

Here we first create a Vulkan function loader which will be used to load the initial Vulkan commands from the Vulkan shared library. Next we create the Vulkan entry point using the function loader which will load all of the commands we need to manage Vulkan instances. Lastly we are now able to call our `^create_instance` function with the Vulkan entry point.

## Cleaning up

The `Instance` should only be destroyed right before the program exits. It can be destroyed in the `App::destroy` method using `destroy_instance`:

```rust,noplaypen
unsafe fn destroy(&mut self) {
    self.instance.destroy_instance(None);
}
```

Like the Vulkan commands used to create objects, the commands used to destroy objects also take an optional reference to custom allocator callbacks. So like before, we pass `None` to indicate we are content with the default allocation behavior.

## Non-conformant Vulkan implementations

Not every platform is so fortunate to have an implementation of the Vulkan API that fully conforms to the Vulkan specification. On such a platform, there may be standard Vulkan features that are not available and/or there may be significant differences between the actual behavior of a Vulkan application using that non-conformant implementation and what the Vulkan specification says that application should behave.

Since version 1.3.216 of the Vulkan SDK, applications that use a non-conformant Vulkan implementation must enable some additional Vulkan extensions. These compatibility extensions have the primary purpose of forcing the developer to acknowledge that their application is using a non-conformant implementation of Vulkan and that they should not expect everything to be as the Vulkan specification says it should be.

This tutorial will be utilizing these compatibility Vulkan extensions so that your application can run even on platforms that lack a fully conforming Vulkan implementation.

However, you might ask "Why are we doing this? Do we really need to worry about supporting niche platforms in an introductory Vulkan tutorial?" As it turns out, the not-so-niche macOS is among those platforms that lack a fully-conformant Vulkan implementation.

As was mentioned in the introduction, Apple has their own low-level graphics API, [Metal](https://en.wikipedia.org/wiki/Metal_(API)). The Vulkan implementation that is provided as part of the Vulkan SDK for macOS ([MoltenVK](https://moltengl.com/)) is a layer that sits in-between your application and Metal and translates the Vulkan API calls your application makes into Metal API calls. Because MoltenVK is [not fully conformant with the Vulkan specification](https://www.lunarg.com/wp-content/uploads/2022/05/The-State-of-Vulkan-on-Apple-15APR2022.pdf), you will need to enable the compatibility Vulkan extensions we've been talking about to support macOS. 

As an aside, while MoltenVK is not fully-conformant, you shouldn't encounter any issues caused by deviations from the Vulkan specification while following this tutorial on macOS.

## Enabling compatibility extensions

> **Note:** Even if you are not following this tutorial on a macOS, some of the code added in this section is referenced in the remainder of this tutorial so you can't just skip it!

We'll want to check if the version of Vulkan we are using is equal to or greather than the version of Vulkan that introduced the compatibility extension requirement. With this goal in mind, we'll first add an additional import:

```rust,noplaypen
use vulkanalia::Version;
```

With this new import in place, we'll define a constant for the minimum version:

```rust,noplaypen
const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);
```

Replace the extension enumeration and instance creation code with the following:

```rust,noplaypen
let mut extensions = vk_window::get_required_instance_extensions(window)
    .iter()
    .map(|e| e.as_ptr())
    .collect::<Vec<_>>();

// Required by Vulkan SDK on macOS since 1.3.216.
let flags = if 
    cfg!(target_os = "macos") && 
    entry.version()? >= PORTABILITY_MACOS_VERSION
{
    info!("Enabling extensions for macOS portability.");
    extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
    extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
    vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
} else {
    vk::InstanceCreateFlags::empty()
};

let info = vk::InstanceCreateInfo::builder()
    .application_info(&application_info)
    .enabled_extension_names(&extensions)
    .flags(flags);
```

This code enables `KHR_PORTABILITY_ENUMERATION_EXTENSION` if your application is being compiled for a platform that lacks a non-conformant Vulkan implementation (just checking for macOS here) and the Vulkan version meets or exceeds the minimum version we just defined.

This code also enables `KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION` under the same conditions. This extension is needed to enable the `KHR_PORTABILITY_SUBSET_EXTENSION` device extension which will be added later in the tutorial when we set up a logical device.

## `Instance` vs `vk::Instance`

When we call our `^create_instance` function, what we get back is not a raw Vulkan instance as would be returned by the Vulkan command `vkCreateInstance` (`vk::Instance`). Instead what we got back is a custom type defined by `vulkanalia` which combines both a raw Vulkan instance and the commands loaded for that specific instance.

This is the `Instance` type we have been using (imported from the `vulkanalia` prelude) which should not be confused with the `vk::Instance` type which represents a raw Vulkan instance. In future chapters we will also use the `Device` type which, like `Instance`, is a pairing of a raw Vulkan device (`vk::Device`) and the commands loaded for that specific device. Fortunately we will not be using `vk::Instance` or `vk::Device` directly in this tutorial so you don't need to worry about getting them mixed up.

Because an `Instance` contains both a Vulkan instance and the associated commands, the command wrappers implemented for an `Instance` are able to provide the Vulkan instance when it is required by the underlying Vulkan command.

If you look at the documentation for the `vkDestroyInstance` command, you will see that it takes two parameters: the instance to destroy and the optional custom allocator callbacks. However, `destroy_instance` only takes the optional custom allocator callbacks because it is able to provide the raw Vulkan handle as the first parameter itself as described above.

Before continuing with the more complex steps after instance creation, it's time to evaluate our debugging options by checking out validation layers.
