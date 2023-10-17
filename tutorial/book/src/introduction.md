# Introduction

This tutorial is an adaptation of <https://vulkan-tutorial.com> to use Rust instead of C++. The majority of the credit for this tutorial should go the author of the original tutorial ([Alexander Overvoorde](https://github.com/Overv)) and the [other contributors](https://github.com/Overv/VulkanTutorial/graphs/contributors).

This tutorial also includes several additional chapters that are original creations of the author of this adapted tutorial (starting with the `Push Constants` chapter). These chapters introduce important Vulkan concepts and features that will be useful in almost any Vulkan application. However, as noted in the disclaimer for those chapters, they should be considered experimental.

## About

This tutorial will teach you the basics of using the [Vulkan](https://www.khronos.org/vulkan/) graphics and compute API. Vulkan is a new API by the [Khronos group](https://www.khronos.org/) (known for OpenGL) that provides a much better abstraction of modern graphics cards. This new interface allows you to better describe what your application intends to do, which can lead to better performance and less surprising driver behavior compared to existing APIs like [OpenGL](https://en.wikipedia.org/wiki/OpenGL) and [Direct3D](https://en.wikipedia.org/wiki/Direct3D). The ideas behind Vulkan are similar to those of [Direct3D 12](https://en.wikipedia.org/wiki/Direct3D#Direct3D_12) and [Metal](https://en.wikipedia.org/wiki/Metal_(API)), but Vulkan has the advantage of being cross-platform and allows you to develop for Windows, Linux and Android at the same time (and iOS and macOS via [MoltenVK](https://github.com/KhronosGroup/MoltenVK)).

However, the price you pay for these benefits is that you have to work with a significantly more verbose API. Every detail related to the graphics API needs to be set up from scratch by your application, including initial frame buffer creation and memory management for objects like buffers and texture images. The graphics driver will do a lot less hand holding, which means that you will have to do more work in your application to ensure correct behavior.

The takeaway message here is that Vulkan is not for everyone. It is targeted at programmers who are enthusiastic about high performance computer graphics, and are willing to put some work in. If you are more interested in game development, rather than computer graphics, then you may wish to stick to OpenGL or Direct3D, which will not be deprecated in favor of Vulkan anytime soon. Another alternative is to use an engine like [Unreal Engine](https://en.wikipedia.org/wiki/Unreal_Engine#Unreal_Engine_4) or [Unity](https://en.wikipedia.org/wiki/Unity_(game_engine)), which will be able to use Vulkan while exposing a much higher level API to you.

With that out of the way, let's cover some prerequisites for following this tutorial:

* A graphics card and driver compatible with Vulkan ([NVIDIA](https://developer.nvidia.com/vulkan-driver), [AMD](http://www.amd.com/en-us/innovations/software-technologies/technologies-gaming/vulkan), [Intel](https://software.intel.com/en-us/blogs/2016/03/14/new-intel-vulkan-beta-1540204404-graphics-driver-for-windows-78110-1540))
* Experience with Rust
* Rust 1.65 or later
* Some existing experience with 3D computer graphics

This tutorial will not assume knowledge of OpenGL or Direct3D concepts, but it does require you to know the basics of 3D computer graphics. It will not explain the math behind perspective projection, for example. See [this online book](https://paroj.github.io/gltut/) for a great introduction of computer graphics concepts. Some other great computer graphics resources are:

* [Ray tracing in one weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
* [Physically Based Rendering book](http://www.pbr-book.org/)
* Vulkan being used in a real engine in the open-source [Quake](https://github.com/Novum/vkQuake) and [DOOM 3](https://github.com/DustinHLand/vkDOOM3)

If you want a C++ tutorial instead, see the original tutorial:<br/><https://vulkan-tutorial.com>

This tutorial uses the [`vulkanalia`](https://github.com/KyleMayes/vulkanalia) crate to provide access to the Vulkan API from Rust. `vulkanalia` provides raw bindings to the Vulkan API as well as a thin wrapper over said bindings to make them easier and more idiomatic to use from Rust (more on this in the next chapter). This means that while you should never have any difficulty in determining exactly how your Rust programs are interacting with the Vulkan API, you will be shielded from little of the danger and verbosity of the Vulkan API.

If you want a Rust Vulkan tutorial that uses a crate which provides a safe and relatively concise wrapper around the Vulkan API ([`vulkano`](https://vulkano.rs)), see this tutorial:<br/><https://github.com/bwasty/vulkan-tutorial-rs>

## Tutorial structure

We'll start with an overview of how Vulkan works and the work we'll have to do to get the first triangle on the screen. The purpose of all the smaller steps will make more sense after you've understood their basic role in the whole picture. Next, we'll set up the development environment with the [Vulkan SDK](https://lunarg.com/vulkan-sdk/).

After that we'll implement all of the basic components of a Vulkan program that are necessary to render your first triangle. Each chapter will follow roughly the following structure:

* Introduce a new concept and its purpose
* Use all of the relevant API calls to integrate it into your program
* Abstract parts of it into helper functions

Although each chapter is written as a follow-up on the previous one, it is also possible to read the chapters as standalone articles introducing a certain Vulkan feature. That means that the site is also useful as a reference. All of the Vulkan functions and types are linked to the either the Vulkan specification or to the `vulkanalia` documentation, so you can click them to learn more. Vulkan is still a fairly young API, so there may be some shortcomings in the specification itself. You are encouraged to submit feedback to [this Khronos repository](https://github.com/KhronosGroup/Vulkan-Docs).

As mentioned before, the Vulkan API has a rather verbose API with many parameters to give you maximum control over the graphics hardware. This causes basic operations like creating a texture to take a lot of steps that have to be repeated every time. Therefore we'll be creating our own collection of helper functions throughout the tutorial.

Every chapter will also start with a link to the final code for that chapter. You can refer to it if you have any doubts about the structure of the code, or if you're dealing with a bug and want to compare.

This tutorial is intended to be a community effort. Vulkan is still a fairly new API and best practices have been fully established. If you have any type of feedback on the tutorial and site itself, then please don't hesitate to submit an issue or pull request to the [GitHub repository](https://github.com/KyleMayes/vulkanalia).

After you've gone through the ritual of drawing your very first Vulkan powered triangle onscreen, we'll start expanding the program to include linear transformations, textures and 3D models.

If you've played with graphics APIs before, then you'll know that there can be a lot of steps until the first geometry shows up on the screen. There are many of these initial steps in Vulkan, but you'll see that each of the individual steps is easy to understand and does not feel redundant. It's also important to keep in mind that once you have that boring looking triangle, drawing fully textured 3D models does not take that much extra work, and each step beyond that point is much more rewarding.

If you encounter any problems while following the tutorial, check the FAQ to see if your problem and its solution is already listed there. Next, you might find someone who had the same problem (if it is not Rust-specific) in the comment section for the corresponding chapter in the [original tutorial](https://vulkan-tutorial.com/).
