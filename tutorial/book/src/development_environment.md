# Development environment

In this chapter we'll set up your environment for developing Vulkan applications by installing the Vulkan SDK for your operating system. This tutorial assumes you already have a working Rust (1.65+) development environment.

## Cargo project

First let's create our Cargo project:

`cargo new vulkan-tutorial`

After this command has executed, you'll have a folder called `vulkan-tutorial` containing a minimal Cargo project which produces a Rust executable.

Open the `Cargo.toml` file in the folder and add these dependencies in the pre-existing `[dependencies]` section:

```toml
anyhow = "1"
log = "0.4"
cgmath = "0.18"
png = "0.17"
pretty_env_logger = "0.5"
thiserror = "1"
tobj = { version = "3", features = ["log"] }
vulkanalia = { version = "=0.24.0", features = ["libloading", "provisional", "window"] }
winit = "0.29"
```

* `anyhow` &ndash; used for simple error handling
* `log` &ndash; used for logging statements
* `cgmath` &ndash; used as a Rust replacement for [GLM](https://glm.g-truc.net/0.9.9/index.html) (graphics math library)
* `png` &ndash; used to load PNGs to use as textures
* `pretty_env_logger` &ndash; used to print our logs to the console
* `thiserror` &ndash; used to define custom errors types without boilerplate
* `tobj` &ndash; used to load 3D models in the [Wavefront .obj format](https://en.wikipedia.org/wiki/Wavefront_.obj_file)
* `vulkanalia` &ndash; used to call the Vulkan API
* `winit` &ndash; used to create a window to render to

## Vulkan SDK

The most important component you'll need for developing Vulkan applications is the SDK. It includes the headers, standard validation layers, debugging tools and a loader for the Vulkan functions. The loader looks up the functions in the driver at runtime, similarly to GLEW for OpenGL - if you're familiar with that.

### Windows

The SDK can be downloaded from the [LunarG website](https://vulkan.lunarg.com/) using the buttons at the bottom of the page. You don't have to create an account, but it will give you access to some additional documentation that may be useful to you.

![](./images/vulkan_sdk_download_buttons.png)

Proceed through the installation and pay attention to the install location of the SDK. The first thing we'll do is verify that your graphics card and driver properly support Vulkan. Go to the directory where you installed the SDK, open the `Bin` directory and run the `vkcube.exe` demo. You should see the following:

![](./images/cube_demo.png)

If you receive an error message then ensure that your drivers are up-to-date, include the Vulkan runtime and that your graphics card is supported. See the [introduction chapter](introduction.html) for links to drivers from the major vendors.

There is another program in this directory that will be useful for development. The `glslangValidator.exe` and `glslc.exe` programs will be used to compile shaders from the human-readable [GLSL](https://en.wikipedia.org/wiki/OpenGL_Shading_Language) to bytecode. We'll cover this in depth in the [shader modules chapter](pipeline/shader_modules.html). The `Bin` directory also contains the binaries of the Vulkan loader and the validation layers, while the `Lib` directory contains the libraries.

Feel free to explore the other files, but we won't need them for this tutorial.

### Linux

These instructions will be aimed at Ubuntu users, but you may be able to follow along by changing the `apt` commands to the package manager commands that are appropriate for you.

The most important components you'll need for developing Vulkan applications on Linux are the Vulkan loader, validation layers, and a couple of command-line utilities to test whether your machine is Vulkan-capable:

* `sudo apt install vulkan-tools` &ndash; Command-line utilities, most importantly `vulkaninfo` and `vkcube`. Run these to confirm your machine supports Vulkan.
* `sudo apt install libvulkan-dev` &ndash; Installs Vulkan loader. The loader looks up the functions in the driver at runtime, similarly to GLEW for OpenGL - if you're familiar with that.
* `sudo apt install vulkan-validationlayers-dev` &ndash; Installs the standard validation layers. These are crucial when debugging Vulkan applications, and we'll discuss them in an upcoming chapter.

If installation was successful, you should be all set with the Vulkan portion. Remember to run `vkcube` and ensure you see the following pop up in a window:

![](./images/cube_demo_nowindow.png)

If you receive an error message then ensure that your drivers are up-to-date, include the Vulkan runtime and that your graphics card is supported. See the [introduction chapter](introduction.html) for links to drivers from the major vendors.

### macOS

The SDK can be downloaded from the [LunarG website](https://vulkan.lunarg.com/) using the buttons at the bottom of the page. You don't have to create an account, but it will give you access to some additional documentation that may be useful to you.

![](./images/vulkan_sdk_download_buttons.png)

The SDK version for MacOS internally uses [MoltenVK](https://moltengl.com/). There is no native support for Vulkan on MacOS, so what MoltenVK does is act as a layer that translates Vulkan API calls to Apple's Metal graphics framework. With this you can take advantage of debugging and performance benefits of Apple's Metal framework.

After downloading it, simply extract the contents to a folder of your choice. Inside the extracted folder, in the `Applications` folder you should have some executable files that will run a few demos using the SDK. Run the `vkcube` executable and you will see the following:

![](./images/cube_demo_mac.png)

#### Setup Environment

When running a Vulkan application outside of the Vulkan SDK directory, you will likely also need to run the `setup-env.sh` script from the Vulkan SDK to avoid errors about the inability to find Vulkan libraries (e.g., `libvulkan.dylib`). If you installed the Vulkan SDK in the default location, this script should be located in a path like this: `~/VulkanSDK/1.3.280.1/setup-env.sh` (replace the version number to match your Vulkan SDK installation).

You can also add this script to be executed by default by adding it to your shell's setup script. For example you could add a statement like this to `~/.zshrc`:

```
source ~/VulkanSDK/1.3.280.1/setup-env.sh
```
