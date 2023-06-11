# Overview

This chapter will start off with an introduction of Vulkan and the problems it addresses. After that we're going to look at the ingredients that are required for the first triangle. This will give you a big picture to place each of the subsequent chapters in. We will conclude by covering the structure of the Vulkan API as implemented by `vulkanalia`.

## Origin of Vulkan

Just like the previous graphics APIs, Vulkan is designed as a cross-platform abstraction over [GPUs](https://en.wikipedia.org/wiki/Graphics_processing_unit). The problem with most of these APIs is that the era in which they were designed featured graphics hardware that was mostly limited to configurable fixed functionality. Programmers had to provide the vertex data in a standard format and were at the mercy of the GPU manufacturers with regards to lighting and shading options.

As graphics card architectures matured, they started offering more and more programmable functionality. All this new functionality had to be integrated with the existing APIs somehow. This resulted in less than ideal abstractions and a lot of guesswork on the graphics driver side to map the programmer's intent to the modern graphics architectures. That's why there are so many driver updates for improving the performance in games, sometimes by significant margins. Because of the complexity of these drivers, application developers also need to deal with inconsistencies between vendors, like the syntax that is accepted for [shaders](https://en.wikipedia.org/wiki/Shader). Aside from these new features, the past decade also saw an influx of mobile devices with powerful graphics hardware. These mobile GPUs have different architectures based on their energy and space requirements. One such example is [tiled rendering](https://en.wikipedia.org/wiki/Tiled_rendering), which would benefit from improved performance by offering the programmer more control over this functionality. Another limitation originating from the age of these APIs is limited multi-threading support, which can result in a bottleneck on the CPU side.

Vulkan solves these problems by being designed from scratch for modern graphics architectures. It reduces driver overhead by allowing programmers to clearly specify their intent using a more verbose API, and allows multiple threads to create and submit commands in parallel. It reduces inconsistencies in shader compilation by switching to a standardized byte code format with a single compiler. Lastly, it acknowledges the general purpose processing capabilities of modern graphics cards by unifying the graphics and compute functionality into a single API.

## What it takes to draw a triangle

We'll now look at an overview of all the steps it takes to render a triangle in a well-behaved Vulkan program. All of the concepts introduced here will be elaborated on in the next chapters. This is just to give you a big picture to relate all of the individual components to.

### Step 1 - Instance and physical device selection

A Vulkan application starts by setting up the Vulkan API through a `VkInstance`. An instance is created by describing your application and any API extensions you will be using. After creating the instance, you can query for Vulkan supported hardware and select one or more `VkPhysicalDevice`s to use for operations. You can query for properties like VRAM size and device capabilities to select desired devices, for example to prefer using dedicated graphics cards.

### Step 2 - Logical device and queue families

After selecting the right hardware device to use, you need to create a `VkDevice` (logical device), where you describe more specifically which `VkPhysicalDeviceFeatures` you will be using, like multi-viewport rendering and 64-bit floats. You also need to specify which queue families you would like to use. Most operations performed with Vulkan, like draw commands and memory operations, are asynchronously executed by submitting them to a `VkQueue`. Queues are allocated from queue families, where each queue family supports a specific set of operations in its queues. For example, there could be separate queue families for graphics, compute and memory transfer operations. The availability of queue families could also be used as a distinguishing factor in physical device selection. It is possible for a device with Vulkan support to not offer any graphics functionality, however all graphics cards with Vulkan support today will generally support all queue operations that we're interested in.

### Step 3 - Window surface and swapchain

Unless you're only interested in offscreen rendering, you will need to create a window to present rendered images to. Windows can be created with the native platform APIs or libraries like [GLFW](http://www.glfw.org/), [SDL](https://www.libsdl.org/), or the [`winit`](https://github.com/rust-windowing/winit) crate. We will be using the `winit` crate in this tutorial, but more about that in the next chapter.

We need two more components to actually render to a window: a window surface (`VkSurfaceKHR`) and a swapchain (`VkSwapchainKHR`). Note the `KHR` postfix, which means that these objects are part of a Vulkan extension. The Vulkan API itself is completely platform agnostic, which is why we need to use the standardized WSI (Window System Interface) extension to interact with the window manager. The surface is a cross-platform abstraction over windows to render to and is generally instantiated by providing a reference to the native window handle, for example `HWND` on Windows. However, `vulkanalia` has optional integration with the `winit` crate which we will be leveraging to handle the platform-specific details of creating a window and associated surface for us.

The swapchain is a collection of render targets. Its basic purpose is to ensure that the image that we're currently rendering to is different from the one that is currently on the screen. This is important to make sure that only complete images are shown. Every time we want to draw a frame we have to ask the swapchain to provide us with an image to render to. When we've finished drawing a frame, the image is returned to the swapchain for it to be presented to the screen at some point. The number of render targets and conditions for presenting finished images to the screen depends on the present mode. Common present modes are  double buffering (vsync) and triple buffering. We'll look into these in the swapchain creation chapter.

Some platforms allow you to render directly to a display without interacting with any window manager through the `VK_KHR_display` and `VK_KHR_display_swapchain` extensions. These allow you to create a surface that represents the entire screen and could be used to implement your own window manager, for example.

### Step 4 - Image views and framebuffers

To draw to an image acquired from the swapchain, we have to wrap it into a `VkImageView` and `VkFramebuffer`. An image view references a specific part of an image to be used, and a framebuffer references image views that are to be used for color, depth and stencil targets. Because there could be many different images in the swapchain, we'll preemptively create an image view and framebuffer for each of them and select the right one at draw time.

### Step 5 - Render passes

Render passes in Vulkan describe the type of images that are used during rendering operations, how they will be used, and how their contents should be treated. In our initial triangle rendering application, we'll tell Vulkan that we will use a single image as color target and that we want it to be cleared to a solid color right before the drawing operation. Whereas a render pass only describes the type of images, a `VkFramebuffer` actually binds specific images to these slots.

### Step 6 - Graphics pipeline

The graphics pipeline in Vulkan is set up by creating a `VkPipeline` object. It describes the configurable state of the graphics card, like the viewport size and depth buffer operation and the programmable state using `VkShaderModule` objects. The `VkShaderModule` objects are created from shader byte code. The driver also needs to know which render targets will be used in the pipeline, which we specify by referencing the render pass.

One of the most distinctive features of Vulkan compared to existing APIs, is that almost all configuration of the graphics pipeline needs to be set in advance. That means that if you want to switch to a different shader or slightly change your vertex layout, then you need to entirely recreate the graphics pipeline. That means that you will have to create many `VkPipeline` objects in advance for all the different combinations you need for your rendering operations. Only some basic configuration, like viewport size and clear color, can be changed dynamically. All of the state also needs to be described explicitly, there is no default color blend state, for example.

The good news is that because you're doing the equivalent of ahead-of-time compilation versus just-in-time compilation, there are more optimization opportunities for the driver and runtime performance is more predictable, because large state changes like switching to a different graphics pipeline are made very explicit.

### Step 7 - Command pools and command buffers

As mentioned earlier, many of the operations in Vulkan that we want to execute, like drawing operations, need to be submitted to a queue. These operations first need to be recorded into a `VkCommandBuffer` before they can be submitted. These command buffers are allocated from a `VkCommandPool` that is associated with a specific queue family. To draw a simple triangle, we need to record a command buffer with the following operations:

* Begin the render pass
* Bind the graphics pipeline
* Draw 3 vertices
* End the render pass

Because the image in the framebuffer depends on which specific image the swapchain will give us, we need to record a command buffer for each possible image and select the right one at draw time. The alternative would be to record the command buffer again every frame, which is not as efficient.

### Step 8 - Main loop

Now that the drawing commands have been wrapped into a command buffer, the main loop is quite straightforward. We first acquire an image from the swapchain with `vkAcquireNextImageKHR`. We can then select the appropriate command buffer for that image and execute it with `vkQueueSubmit`. Finally, we return the image to the swapchain for presentation to the screen with `vkQueuePresentKHR`.

Operations that are submitted to queues are executed asynchronously. Therefore we have to use synchronization objects like semaphores to ensure a correct order of execution. Execution of the draw command buffer must be set up to wait on image acquisition to finish, otherwise it may occur that we start rendering to an image that is still being read for presentation on the screen. The `vkQueuePresentKHR` call in turn needs to wait for rendering to be finished, for which we'll use a second semaphore that is signaled after rendering completes.

### Summary

This whirlwind tour should give you a basic understanding of the work ahead for drawing the first triangle. A real-world program contains more steps, like allocating vertex buffers, creating uniform buffers and uploading texture images that will be covered in subsequent chapters, but we'll start simple because Vulkan has enough of a steep learning curve as it is. Note that we'll cheat a bit by initially embedding the vertex coordinates in the vertex shader instead of using a vertex buffer. That's because managing vertex buffers requires some familiarity with command buffers first.

So in short, to draw the first triangle we need to:

* Create a `VkInstance`
* Select a supported graphics card (`VkPhysicalDevice`)
* Create a `VkDevice` and `VkQueue` for drawing and presentation
* Create a window, window surface and swapchain
* Wrap the swapchain images into `VkImageView`
* Create a render pass that specifies the render targets and usage
* Create framebuffers for the render pass
* Set up the graphics pipeline
* Allocate and record a command buffer with the draw commands for every possible swapchain image
* Draw frames by acquiring images, submitting the right draw command buffer and returning the images back to the swapchain

It's a lot of steps, but the purpose of each individual step will be made very simple and clear in the upcoming chapters. If you're confused about the relation of a single step compared to the whole program, you should refer back to this chapter.

## API concepts

The Vulkan API is defined in terms of the C programming language. The canonical version of the Vulkan API is defined in the Vulkan API Registry which is [an XML file](https://github.com/KhronosGroup/Vulkan-Docs/blob/main/xml/vk.xml) which serves as a machine readable definition of the Vulkan API.

The [Vulkan headers](https://github.com/KhronosGroup/Vulkan-Headers) that are part of the Vulkan SDK you will be installing in the next chapter are generated from this Vulkan API Registry. However, we will not be using these headers, directly or indirectly, because `vulkanalia` includes a Rust interface to the Vulkan API generated from the Vulkan API registry that is independent of the C interface provided by the Vulkan SDK.

The foundation of `vulkanalia` is the [`vulkanalia-sys`](https://docs.rs/vulkanalia-sys) crate which defines the raw types (commands, enums, bitmasks, structs, etc.) defined by the Vulkan API Registry. These raw types are re-exported from the `vulkanalia` crate in the [`vk`](https://docs.rs/vulkanalia/%VERSION%/vulkanalia/vk/index.html) module along with some other items generated from the Vulkan API Registry which serve as the thin wrapper around the Vulkan API previously mentioned in the introduction.

### Type Names

Because Rust has support for namespaces unlike C, the `vulkanalia` API omits the parts of Vulkan type names that are used for namespacing purposes in C. More specifically, Vulkan types such as structs, unions, and enums lose their `Vk` prefix. For example, the `VkInstanceCreateInfo` struct becomes the [`InstanceCreateInfo`](https://docs.rs/vulkanalia/%VERSION%/vulkanalia/vk/struct.InstanceCreateInfo.html) struct in `vulkanalia` and can be found in the previously mentioned [`vk`](https://docs.rs/vulkanalia/%VERSION%/vulkanalia/vk/index.html) module.

Going forward, this tutorial will refer to the Vulkan types defined by `vulkanalia` using the `vk::` module prefix to make it clear the type represents something generated from the Vulkan API Registry.

These type names will each be links to the `vulkanalia` documentation for the referenced type. The `vulkanalia` documentation for Vulkan types will also contain a link to the Vulkan specification for the type which you can use to learn more about the purpose and usage of the type.

A few type name examples:

* `vk::Instance`&nbsp;
* `vk::InstanceCreateInfo`&nbsp;
* `vk::InstanceCreateFlags`&nbsp;

### Enums

`vulkanalia` models Vulkan enums as structs and models variants as associated constants for these structs. Rust enums are not used for Vulkan enums because the use of Rust enums in FFI can lead to [undefined behavior](https://github.com/rust-lang/rust/issues/36927).

Since associated constants are namespaced to the struct they are for, we don't need to worry about name conflicts between the values of different Vulkan enums (or enums from other libraries) like we would in C. So like with type names, `vulkanalia` omits the parts of variant names used for namespacing purposes.

For example, the `VK_OBJECT_TYPE_INSTANCE` variant is the `INSTANCE` value for the `VkObjectType` enum. In `vulkanalia`, this variant becomes `vk::ObjectType::INSTANCE`.

### Bitmasks

`vulkanalia` models Vulkan bitmasks as structs and models bitflags as associated constants for these structs. These structs and associated constants are generated by the `bitflags!` macro from the [`bitflags`](https://github.com/bitflags/bitflags) crate.

Like with variants, the parts of bitmask names used for namespacing purposes are omitted.

For example, the `VK_BUFFER_USAGE_TRANSFER_SRC_BIT` bitflag is the `TRANSFER_SRC` bitflag for the `VkBufferUsageFlags` bitmask. In `vulkanalia`, this becomes `vk::BufferUsageFlags::TRANSFER_SRC`.

### Commands

The types for raw Vulkan commands like `vkCreateInstance` are defined in `vulkanalia` as function pointer type aliases with the `PFN_` (pointer to function) prefix. So the `vulkanalia` type alias for `vkCreateInstance` is `vk::PFN_vkCreateInstance`.

These function pointer types are not enough on their own to call Vulkan commands, we first need to load the commands described by these types. The Vulkan specification has a [detailed description](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#initialization-functionpointers) of how this is done, but I will present a simplified version here.

The first Vulkan command to load is `vkGetInstanceProcAddr`. This command is loaded in a platform-specific manner, but `vulkanalia` provides an optional integration with [`libloading`](https://crates.io/crates/libloading) that we will be using in this tutorial to load this command from a Vulkan shared library. `vkGetInstanceProcAddr` can be then used to load the other Vulkan commands we want to call.

However, there may be multiple versions of Vulkan commands available depending on the Vulkan implementations on your system. For example, if your system has both a dedicated NVIDIA GPU and an integrated Intel GPU, there may be separate implementations of device-specific Vulkan commands like `allocate_memory` for each device. In cases like this, `vkGetInstanceProcAddr` will return a command that will dispatch calls to the appropriate device-specific command depending on the device in use.

To avoid the runtime overhead of this dispatch, the `vkGetDeviceProcAddr` command can be used to directly load these device-specific Vulkan commands. This command is loaded in the same manner as `vkGetInstanceProcAddr`.

We will be calling dozens of Vulkan commands in this tutorial. Fortunately we won't have to load them manually, `vulkanalia` provides structs which can be used to easily load all the Vulkan commands in one of four categories:

* `vk::StaticCommands` &ndash; The Vulkan commands loaded in a platform-specific manner that can then used to load the other commands (i.e., `vkGetInstanceProcAddr` and `vkGetDeviceProcAddr`)
* `vk::EntryCommands` &ndash; The Vulkan commands loaded using `vkGetInstanceProcAddr` and a null Vulkan instance. These commands are not tied to a specific Vulkan instance and are used to query instance support and create instances
* `vk::InstanceCommands` &ndash; The Vulkan commands loaded using `vkGetInstanceProcAddr` and a valid Vulkan instance. These commands are tied to a specific Vulkan instance and, among other things, are used to query device support and create devices
* `vk::DeviceCommands` &ndash; The Vulkan commands loaded using `vkGetDeviceProcAddr` and a valid Vulkan device. These commands are tied to a specific Vulkan device and expose most of the functionality you would expect from a graphics API

These structs allow you to easily load and call raw Vulkan commands from Rust, but `vulkanalia` offers wrappers around the raw Vulkan commands which make calling them from Rust easier and less error-prone.

### Command wrappers

An example of a typical Vulkan command signature looks like this in C:

```c
VkResult vkEnumerateInstanceExtensionProperties(
    const char* pLayerName,
    uint32_t* pPropertyCount,
    VkExtensionProperties* pProperties
);
```

Someone who is familiar with the conventions of the Vulkan API could quickly see how this command is supposed to be used from this signature alone despite it not including some key information.

For those new to the Vulkan API, a look at the [documentation](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceExtensionProperties.html) for this command will likely be more illuminating. The description of the behavior of this command in the documentation suggests that using this command to list the available extensions for the Vulkan instance will be a multi-step process:

 1. Call the command to get the number of extensions
 2. Allocate a buffer that can contain the outputted number of extensions
 3. Call the command again to populate the buffer with the extensions

So in C++, this might look like this (ignoring the result of the command for simplicity):

```c++
// 1.
uint32_t pPropertyCount;
vkEnumerateInstanceExtensionProperties(NULL, &pPropertyCount, NULL);

// 2.
std::vector<VkExtensionProperties> pProperties{pPropertyCount};

// 3.
vkEnumerateInstanceExtensionProperties(NULL, &pPropertyCount, pProperties.data());
```

The Rust signature of the wrapper for `vkEnumerateInstanceExtensionProperties` looks like this:

```rust,noplaypen
unsafe fn enumerate_instance_extension_properties(
    &self,
    layer_name: Option<&[u8]>,
) -> VkResult<Vec<ExtensionProperties>>;
```

This command wrapper makes the usage of `vkEnumerateInstanceExtensionProperties` from Rust easier, less error-prone, and more idiomatic in several ways:

* The optionality of the `layer_name` parameter is encoded in the function signature. That this parameter is optional is not captured in the C function signature, one would need to check the Vulkan specification for this information
* The fallibility of the command is modelled by returning a `Result` ([`VkResult<T>`](https://docs.rs/vulkanalia/%VERSION%/vulkanalia/type.VkResult.html) is a type alias for `Result<T, vk::ErrorCode>`). This allows us to take advantage of Rust's strong error handling capabilities as well as be warned by the compiler if we neglect to check the result of a fallible command
* The command wrapper handles the three step process described above internally and returns a `Vec` containing the extension properties

Note that command wrappers are still `unsafe` because while `vulkanalia` can eliminate certain classes of errors (e.g., passing a null layer name to this command), there are still plenty of things that can go horribly wrong and cause fun things like segfaults. You can always check the `Valid Usage` section of the Vulkan documentation for a command to see the invariants that need to upheld to call that command validly.

You likely noticed the `&self` parameter in the above command wrapper. These command wrappers are defined in traits which are implemented for types exposed by `vulkanalia`. These traits can be separated into two categories: version traits and extension traits. The version traits offer command wrappers for the commands which are a standard part of Vulkan whereas the extension traits offer command wrappers for the commands which are defined as part of Vulkan extensions.

For example, `enumerate_instance_extension_properties` is in the `vk::EntryV1_0` trait since it is a non-extension Vulkan command that is part of Vulkan 1.0 and not dependent on a Vulkan instance or device. A Vulkan command like `cmd_draw_indirect_count` that was added in Vulkan 1.2 and is dependent on a Vulkan device would be in the `vk::DeviceV1_2` trait.

`vk::KhrSurfaceExtension` is an example of an extension trait that we will be using in future chapters to call Vulkan commands like `destroy_surface_khr` that are defined in the `VK_KHR_surface` extension.

These version and extension traits are defined for types which contain both the loaded commands and the required Vulkan instance or device (if any). These types have been lovingly hand-crafted and are not part of the generated Vulkan bindings in the `vk` module of `vulkanalia`. They will be used in future chapters and are the `Entry`, `Instance`, and `Device` structs.

Going forward, this tutorial will continue to refer to these command wrappers directly by name as in this section (e.g., `create_instance`). You can visit the `vulkanalia` documentation for the command wrapper for more information like which trait the command wrapper is defined in.

### Builders

The Vulkan API heavily utilizes structs as parameters for Vulkan commands. The Vulkan structs used as command parameters have a field which indicates the type of the struct. In the C API, this field (`sType`) would need to be set explicitly. For example, here we are populating an instance of `VkInstanceCreateInfo` and then using it to call `vkCreateInstance` in C++:

```c++
std::vector<const char*> extensions{/* 3 extension names */};

VkInstanceCreateInfo info;
info.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
info.enabledExtensionCount = static_cast<uint32_t>(extensions.size());
info.ppEnabledExtensionNames = extensions.data();

VkInstance instance;
vkCreateInstance(&info, NULL, &instance);
```

You can still populate parameter structs in this manner when using `vulkanalia`, but `vulkanalia` provides builders which simplify the construction of these parameter structs. The `vulkanalia` builder for `vk::InstanceCreateInfo` is `vk::InstanceCreateInfoBuilder`. Using this builder the above code would become:

```rust,noplaypen
let extensions = &[/* 3 extension names */];

let info = vk::InstanceCreateInfo::builder()
    .enabled_extension_names(extensions)
    .build();

let instance = entry.create_instance(&info, None).unwrap();
```

Note the following differences:

* A value is not provided for the `s_type` field. This is because the builder provides the correct value for this field (`vk::StructureType::INSTANCE_CREATE_INFO`) automatically
* A value is not provided for the `enabled_extension_count` field. This is because the `enabled_extension_names` builder method uses the length of the provided slice to set this field automatically

However, the above Rust code involves a certain degree of danger. The builders have lifetimes which enforce that the references stored in them live at least as long as the builders themselves. In the above example, this means that the Rust compiler will make sure that the slice passed to the `enabled_extension_names` method lives at least as long as the builder. However, as soon as we call `.build()` to get the underlying `vk::InstanceCreateInfo` struct the builder lifetimes are discarded. This means that the Rust compiler can no longer prevent us from shooting ourselves in the foot if we try to dereference a pointer to a slice that no longer exists.

The following code will (hopefully) crash since the temporary `Vec` passed to `enabled_extension_names` will have been dropped by the time we call `create_instance` with our `vk::InstanceCreateInfo` struct:

```rust,noplaypen
let info = vk::InstanceCreateInfo::builder()
    .enabled_extension_names(&vec![/* 3 extension names */])
    .build();

let instance = entry.create_instance(&info, None).unwrap();
```

Fortunately, `vulkanalia` has a solution for this. Simply don't call `build()` and instead pass the builder to the command wrapper instead! Anywhere a Vulkan struct is expected in a command wrapper you can instead provide the associated builder. If you remove the `build()` call from the above code the Rust compiler will be able to use the lifetimes on the builder to reject this bad code with `error[E0716]: temporary value dropped while borrowed`.

### Preludes

`vulkanalia` offers [prelude modules](https://docs.rs/vulkanalia/%VERSION%/vulkanalia/prelude/index.html) that expose the basic types needed to use the crate. One prelude module is available per Vulkan version and each will expose the relevant command traits along with other very frequently used types:

```rust,noplaypen
// Vulkan 1.0
use vulkanalia::prelude::v1_0::*;

// Vulkan 1.1
use vulkanalia::prelude::v1_1::*;

// Vulkan 1.2
use vulkanalia::prelude::v1_2::*;
```

## Validation layers

As mentioned earlier, Vulkan is designed for high performance and low driver overhead. Therefore it will include very limited error checking and debugging capabilities by default. The driver will often crash instead of returning an error code if you do something wrong, or worse, it will appear to work on your graphics card and completely fail on others.

Vulkan allows you to enable extensive checks through a feature known as *validation layers*. Validation layers are pieces of code that can be inserted between the API and the graphics driver to do things like running extra checks on function parameters and tracking memory management problems. The nice thing is that you can enable them during development and then completely disable them when releasing your application for zero overhead. Anyone can write their own validation layers, but the Vulkan SDK by LunarG provides a standard set of validation layers that we'll be using in this tutorial. You also need to register a callback function to receive debug messages from the layers.

Because Vulkan is so explicit about every operation and the validation layers are so extensive, it can actually be a lot easier to find out why your screen is black compared to OpenGL and Direct3D!
