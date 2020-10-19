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

### Step 3 - Window surface and swap chain

Unless you're only interested in offscreen rendering, you will need to create a window to present rendered images to. Windows can be created with the native platform APIs or libraries like [GLFW](http://www.glfw.org/), [SDL](https://www.libsdl.org/), or the [`winit`](https://github.com/rust-windowing/winit) crate. We will be using the `winit` crate in this tutorial, but more about that in the next chapter.

We need two more components to actually render to a window: a window surface (`VkSurfaceKHR`) and a swap chain (`VkSwapchainKHR`). Note the `KHR` postfix, which means that these objects are part of a Vulkan extension. The Vulkan API itself is completely platform agnostic, which is why we need to use the standardized WSI (Window System Interface) extension to interact with the window manager. The surface is a cross-platform abstraction over windows to render to and is generally instantiated by providing a reference to the native window handle, for example `HWND` on Windows. However, `vulkanalia` has optional integration with the `winit` crate which we will be leveraging to handle the platform-specific details of creating a window and associated surface for us.

The swap chain is a collection of render targets. Its basic purpose is to ensure that the image that we're currently rendering to is different from the one that is currently on the screen. This is important to make sure that only complete images are shown. Every time we want to draw a frame we have to ask the swap chain to provide us with an image to render to. When we've finished drawing a frame, the image is returned to the swap chain for it to be presented to the screen at some point. The number of render targets and conditions for presenting finished images to the screen depends on the present mode. Common present modes are  double buffering (vsync) and triple buffering. We'll look into these in the swap chain creation chapter.

Some platforms allow you to render directly to a display without interacting with any window manager through the `VK_KHR_display` and `VK_KHR_display_swapchain` extensions. These allow you to create a surface that represents the entire screen and could be used to implement your own window manager, for example.

### Step 4 - Image views and framebuffers

To draw to an image acquired from the swap chain, we have to wrap it into a `VkImageView` and `VkFramebuffer`. An image view references a specific part of an image to be used, and a framebuffer references image views that are to be used for color, depth and stencil targets. Because there could be many different images in the swap chain, we'll preemptively create an image view and framebuffer for each of them and select the right one at draw time.

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

Because the image in the framebuffer depends on which specific image the swap chain will give us, we need to record a command buffer for each possible image and select the right one at draw time. The alternative would be to record the command buffer again every frame, which is not as efficient.

### Step 8 - Main loop

Now that the drawing commands have been wrapped into a command buffer, the main loop is quite straightforward. We first acquire an image from the swap chain with `vkAcquireNextImageKHR`. We can then select the appropriate command buffer for that image and execute it with `vkQueueSubmit`. Finally, we return the image to the swap chain for presentation to the screen with `vkQueuePresentKHR`.

Operations that are submitted to queues are executed asynchronously. Therefore we have to use synchronization objects like semaphores to ensure a correct order of execution. Execution of the draw command buffer must be set up to wait on image acquisition to finish, otherwise it may occur that we start rendering to an image that is still being read for presentation on the screen. The `vkQueuePresentKHR` call in turn needs to wait for rendering to be finished, for which we'll use a second semaphore that is signaled after rendering completes.

### Summary

This whirlwind tour should give you a basic understanding of the work ahead for drawing the first triangle. A real-world program contains more steps, like allocating vertex buffers, creating uniform buffers and uploading texture images that will be covered in subsequent chapters, but we'll start simple because Vulkan has enough of a steep learning curve as it is. Note that we'll cheat a bit by initially embedding the vertex coordinates in the vertex shader instead of using a vertex buffer. That's because managing vertex buffers requires some familiarity with command buffers first.

So in short, to draw the first triangle we need to:

* Create a `VkInstance`
* Select a supported graphics card (`VkPhysicalDevice`)
* Create a `VkDevice` and `VkQueue` for drawing and presentation
* Create a window, window surface and swap chain
* Wrap the swap chain images into `VkImageView`
* Create a render pass that specifies the render targets and usage
* Create framebuffers for the render pass
* Set up the graphics pipeline
* Allocate and record a command buffer with the draw commands for every possible swap chain image
* Draw frames by acquiring images, submitting the right draw command buffer and returning the images back to the swap chain

It's a lot of steps, but the purpose of each individual step will be made very simple and clear in the upcoming chapters. If you're confused about the relation of a single step compared to the whole program, you should refer back to this chapter.

## API concepts

The Vulkan API is defined in terms of the C programming language. This API is defined in the Vulkan API Registry which is [an XML file](https://github.com/KhronosGroup/Vulkan-Docs/blob/main/xml/vk.xml) which serves the purpose of being a machine readable definition of the Vulkan API.

The Vulkan [C and C++ headers](https://github.com/KhronosGroup/Vulkan-Headers) that are part of the Vulkan SDK you will be installing in the next chapter are generated from this Vulkan API Registry. However, we will not be using these headers, directly or indirectly, because `vulkanalia` includes a Rust interface to the Vulkan API generated from the Vulkan API registry.

Underneath `vulkanalia` is the [`vulkanalia-sys`](https://docs.rs/vulkanalia-sys) crate which defines the raw types (commands, enums, bitmasks, structs, etc.) defined by the Vulkan API Registry. These raw types are re-exported from the `vulkanalia` crate in the [`vk`](https://docs.rs/vulkanalia/latest/vulkanalia/vk/index.html) module along with some other types generated from the Vulkan API Registry which allow for somewhat simpler and less error-prone usage of the Vulkan API from Rust.

### Type Names

Because Rust has support for namespaces unlike C, the `vulkanalia` API omits the parts of Vulkan type names that are used for namespacing purposes in C. Types such as structs, unions, and enums lose their `Vk` prefix (e.g., `VkInstance` becomes `vk::Instance` where `vk` is the the module of the same name in `vulkanalia`).

### Enums

Vulkan enums are modeled as structs and the enum values are modeled as associated constants on the struct. Rust enums are not used to model Vulkan enums because use of Rust enums in FFI can lead to [undefined behavior](https://github.com/rust-lang/rust/issues/36927).

Since we don't need to worry about name conflicts between the enum values for different enums (or different libraries), the portions of the enum value names used for namespacing purposes are omitted.

For example, the `VK_OBJECT_TYPE_INSTANCE` enum value is the `INSTANCE` value for the `VkObjectType` enum. In `vulkanalia`, this enum value becomes [`vk::ObjectType::INSTANCE`](https://docs.rs/vulkanalia/latest/vulkanalia/vk/struct.ObjectType.html#associatedconstant.INSTANCE).

### Bitmasks

Vulkan bitmasks are modeled as structs with the bitflags as associated constants (similarly to enums) which are generated by the `bitflags!` macro from the [`bitflags`](https://github.com/bitflags/bitflags) crate.

Like with enums values, the portions of bitmask names used for namespacing purposes are omitted.

For example, the `VK_BUFFER_USAGE_TRANSFER_SRC_BIT` bitflag is the `TRANSFER_SRC` bitflag for the `VkBufferUsageFlags` bitmask. In `vulkanalia`, this becomes [`vk::BufferUsageFlags::TRANSFER_SRC`](https://docs.rs/vulkanalia/latest/vulkanalia/vk/struct.BufferUsageFlags.html#associatedconstant.TRANSFER_SRC).

### Commands

The raw Vulkan commands are defined in `vulkanalia` with the `PFN_` (pointer to function) prefix. So [`vk::PFN_vkEnumerateInstanceExtensionProperties`](https://docs.rs/vulkanalia/latest/vulkanalia/vk/type.PFN_vkEnumerateInstanceExtensionProperties.html) refers to a function pointer for the `vkEnumerateInstanceExtensionProperties` Vulkan command.

If we want to actually call these Vulkan commands, we first need to load them. Vulkan commands are loaded by and stored in four structs, the contents of which are determined by how those commands are loaded. Only two Vulkan commands are directly loaded from a Vulkan shared library, `vkGetInstanceProcAddr` and `vkGetDeviceProcAddr`. These commands are then used to load all of the other Vulkan commands. The four structs are (note that Vulkan instances and devices are a topic that will be covered in future chapters):

 * [`vk::StaticCommands`](https://docs.rs/vulkanalia/latest/vulkanalia/vk/struct.StaticCommands.html) &ndash; the Vulkan commands listed above which are loaded directly from a Vulkan shared library
 * [`vk::EntryCommands`](https://docs.rs/vulkanalia/latest/vulkanalia/vk/struct.EntryCommands.html) &ndash; the Vulkan commands loaded using `vkGetInstanceProcAddr` and a null Vulkan instance (i.e., Vulkan commands not tied to a particular Vulkan instance)
 * [`vk::InstanceCommands`](https://docs.rs/vulkanalia/latest/vulkanalia/vk/struct.InstanceCommands.html) &ndash; the Vulkan commands loaded using `vkGetInstanceProcAddr` and a valid Vulkan instance
 * [`vk::DeviceCommands`](https://docs.rs/vulkanalia/latest/vulkanalia/vk/struct.DeviceCommands.html) &ndash; the Vulkan commands loaded using `vkGetDeviceProcAddr` and a valid Vulkan device

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

For those new to the Vulkan API, a look at the [documentation](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateDeviceExtensionProperties.html) for this command will be more illuminating. The description of the behavior of this command in the documentation suggests that using this command to list the available extensions will be a multi-step process:

 1. Call the command to get the number of extensions
 2. Allocate a buffer that can contain the specified number of extensions
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

The Rust signature for the command wrapper looks like this:

```rust,noplaypen
fn enumerate_instance_extension_properties(
    &self,
    layer_name: Option<&[u8]>,
) -> VkResult<Vec<ExtensionProperties>>;
```

These command wrappers simplify usage of Vulkan commands by handling the need to call some commands twice such as in this case as well as capturing the fallibility of the underlying command by returning a (Rust) `Result` ([`VkResult<T>`](https://docs.rs/vulkanalia/latest/vulkanalia/type.VkResult.html) in this case is a type alias for `Result<T, vk::Result>`). Note that the `layer_name` argument is optional, a fact which is defined by the Vulkan API Registry but cannot be indicated by a C function signature alone.

You likely noticed the `&self` parameter in the above command wrapper. These command wrappers are defined in traits which are implemented for types exposed by `vulkanalia`. The traits can be separated into two categories: version traits and extension traits. The version traits offer command wrappers for the commands which are a standard part of Vulkan whereas the extension traits offer command wrappers for the commands which are defined as part of Vulkan extensions.

For example, the above command wrapper is in the [`vk::EntryV1_0`](https://docs.rs/vulkanalia/latest/vulkanalia/vk/trait.EntryV1_0.html) trait since it is a standard Vulkan command that is part of Vulkan 1.0 and is not dependent on a valid Vulkan instance or version (as described in the previous section).

A Vulkan device command that was added in Vulkan 1.2 would be in the [`vk::DeviceV1_2`](https://docs.rs/vulkanalia/latest/vulkanalia/vk/trait.DeviceV1_2.html) trait. [`vk::KhrSurfaceExtension`](https://docs.rs/vulkanalia/latest/vulkanalia/vk/trait.KhrSurfaceExtension.html) is an example of an extension trait that we will be using in future chapters to call Vulkan commands that are defined as part of the [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_surface.html) extension.

These traits are defined for types which contain both the loaded commands and the required Vulkan instance or device (if any). These types have been lovingly hand-crafted and are not part of the generated Vulkan bindings in the `vk` module of `vulkanalia`. They will be used in future chapters and are the `Entry`, `Instance`, and `Device` structs and can be found in the `Structs` section of the [`vulkanalia` documentation](https://docs.rs/vulkanalia).

### Builders

The Vulkan API heavily utilizes structs as parameters for Vulkan commands. Many of these structs have a field which indicates the type of the struct. In the C API (called from C++ in this example), this field (`sType`) would need to be set explicitly, for example:

```c++
std::vector<const char*> extensions{/* 3 extension names */};

VkInstanceCreateInfo info;
info.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
info.enabledExtensionCount = static_cast<uint32_t>(extensions.size());
info.ppEnabledExtensionNames = extensions.data();

VkInstance instance;
vkCreateInstance(&info, NULL, &instance);
```

`vulkanalia` retains this approach but makes it a bit easier to work with while also preventing certain classes of errors. `vulkanalia` has builder structs which simplify the construction of Vulkan structs by populating the struct with defaults which will include the appropriate value for structure type field when present. These builder structs also simplify the setting of linked fields that represent arrays such as `enabledExtensionCount` and `ppEnabledExtensionNames` in the above code. Using `vulkanalia` the above code would become:

```rust,noplaypen
let extensions = &[/* 3 extensions */];

let info = vk::InstanceCreateInfo::builder()
    .enabled_extension_names(extensions)
    .build();

let instance = entry.create_instance(&info, None).unwrap();
```

Here we create a builder struct instance ([`vk::InstanceCreateInfoBuilder`](https://docs.rs/vulkanalia/latest/vulkanalia/vk/struct.InstanceCreateInfoBuilder.html)) for the [`vk::InstanceCreateInfo`](https://docs.rs/vulkanalia/latest/vulkanalia/vk/struct.InstanceCreateInfo.html) struct that will be initially be populated with default values which includes setting the `s_type` field to `vk::StructureType::INSTANCE_CREATE_INFO`. Then we set the enabled extension names (which will set both the `enabled_extension_count` and `enabled_extension_name` fields). Then we can build our `vk::InstanceCreateInfo` struct and pass it to the Vulkan command wrapper.

However, the above Rust code involves a certain degree of danger. The builder structs have lifetimes which enforce that the references stored in them live at least as long as the struct. In the above example, this means that the Rust compiler will make sure that the value passed to the `enabled_extension_names` method lives at least as long as the builder struct. However, as soon as we call `build()` to get the raw Vulkan struct these lifetimes are discarded which means the Rust compiler can't prevent us from attempting to access a value that has been dropped.

The following code will (hopefully) crash since the temporary `Vec` passed to the `enabled_extension_names` value will have been dropped by the time we call the Vulkan command with our `vk::InstanceCreateInfo` struct:

```rust,noplaypen
let info = vk::InstanceCreateInfo::builder()
    .enabled_extension_names(&vec![/* 3 extensions */])
    .build();

let instance = entry.create_instance(&info, None).unwrap();
```

Fortunately, `vulkanalia` has a solution for this. Simply don't call `build()` and instead pass the builder struct to the command wrapper instead! Anywhere a Vulkan struct is expected in a command wrapper you can instead provide the associated builder struct. If you remove the `build()` call from the above code the Rust compiler will be able to use the lifetimes on the builder struct to reject this incorrect code with `error[E0716]: temporary value dropped while borrowed`.

### Preludes

`vulkanalia` offers [prelude modules](https://docs.rs/vulkanalia/latest/vulkanalia/prelude/index.html) that expose the basic types needed to use the crate. One prelude module is available per Vulkan version and each will expose the relevant command traits along with other very frequently used types:

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
