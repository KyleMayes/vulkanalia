# Instance

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/01_instance_creation.rs)

The very first thing you will want to do is initialize the Vulkan library by creating an *instance*. The instance is the connection between your application and the Vulkan library and creating it involves specifying some details about your application to the driver. To get started, add the following imports:

```rust,noplaypen
use anyhow::{anyhow, Result};
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::window as vk_window;
use vulkanalia::prelude::v1_0::*;
```

Here we first add the [`anyhow!`](https://docs.rs/anyhow/latest/anyhow/macro.anyhow.html) macro to our imports from `anyhow`. This macro will be used to easily construct instances of `anyhow` errors. Next, we import `LibloadingLoader` which serves as `vulkanalia`'s `libloading` integration which we will use to load the initial Vulkan commands from the Vulkan shared library. The standard name of the Vulkan shared library on your operating system (e.g., `vulkan-1.dll` on Windows) is then imported as `LIBRARY`.

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

## Driver compability (macOS)

If you are using macOS, you may be using a version of the Vulkan SDK that requires additional setup for your Vulkan application to run properly (1.3.216 or later). Attempt to run your application. If it runs successfully, you can ignore this section.

If you instead get an error about driver incompability (e.g., `The required version of Vulkan is not supported by the driver or is otherwise incompatible for implementation-specific reasons.`) you will need to enable an additional portability extension.

Replace the extension enumeration and instance creation code with the following:

```rust,noplaypen
let mut extensions = vk_window::get_required_instance_extensions(window)
    .iter()
    .map(|e| e.as_ptr())
    .collect::<Vec<_>>();

let flags = if entry
    .enumerate_instance_extension_properties(None)?
    .iter()
    .any(|e| e.extension_name == vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name)
{
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

This code automatically enables `KHR_PORTABILITY_ENUMERATION_EXTENSION` if it is supported and sets a related instance creation flag. As mentioned in the [Getting Started](https://vulkan.lunarg.com/doc/sdk/1.3.216.0/mac/getting_started.html) guide for the macOS Vulkan SDK, this extension is required to permit selection of Vulkan implementations that are not fully conformant to the Vulkan specification.

The Vulkan implementation used on macOS, [MoltenVK](https://github.com/KhronosGroup/MoltenVK), is [not fully conformant](https://www.lunarg.com/wp-content/uploads/2022/05/The-State-of-Vulkan-on-Apple-15APR2022.pdf) and therefore requires this extension to be usable.

## `Instance` vs `vk::Instance`

When we call our `^create_instance` function, what we get back is not a raw Vulkan instance as would be returned by the Vulkan command `vkCreateInstance` (`vk::Instance`). Instead what we got back is a custom type defined by `vulkanalia` which combines both a raw Vulkan instance and the commands loaded for that specific instance.

This is the `Instance` type we have been using (imported from the `vulkanalia` prelude) which should not be confused with the `vk::Instance` type which represents a raw Vulkan instance. In future chapters we will also use the `Device` type which, like `Instance`, is a pairing of a raw Vulkan device (`vk::Device`) and the commands loaded for that specific device. Fortunately we will not be using `vk::Instance` or `vk::Device` directly in this tutorial so you don't need to worry about getting them mixed up.

Because an `Instance` contains both a Vulkan instance and the associated commands, the command wrappers implemented for an `Instance` are able to provide the Vulkan instance when it is required by the underlying Vulkan command.

If you look at the documentation for the `vkDestroyInstance` command, you will see that it takes two parameters: the instance to destroy and the optional custom allocator callbacks. However, `destroy_instance` only takes the optional custom allocator callbacks because it is able to provide the raw Vulkan handle as the first parameter itself as described above.

Before continuing with the more complex steps after instance creation, it's time to evaluate our debugging options by checking out validation layers.
