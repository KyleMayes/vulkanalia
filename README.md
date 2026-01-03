# vulkanalia

[![Crate](https://img.shields.io/crates/v/vulkanalia)](https://crates.io/crates/vulkanalia)
[![Documentation](https://docs.rs/vulkanalia/badge.svg)](https://docs.rs/vulkanalia)
[![CI](https://img.shields.io/github/actions/workflow/status/KyleMayes/vulkanalia/ci.yml?branch=master)](https://github.com/KyleMayes/vulkanalia/actions?query=workflow%3ACI)
![MSRV](https://img.shields.io/badge/MSRV-1.88.0-blue)

Vulkan bindings for Rust.

Released under the Apache License 2.0.

*Heavily inspired by the [`ash`](https://github.com/MaikKlein/ash) crate.*

## [Vulkan Tutorial](https://kylemayes.github.io/vulkanalia)

For users new to Vulkan, there is a complete adaptation of https://vulkan-tutorial.com by [Alexander Overvoorde](https://github.com/Overv) to use Rust and `vulkanalia` instead of C++. The published version of this tutorial can be found [here](https://kylemayes.github.io/vulkanalia) and the sources for the tutorial (including standalone working code examples for each chapter) are in this repository in the `tutorial` directory.

## Overview

[`vulkanalia-sys`](https://docs.rs/vulkanalia-sys/latest/vulkanalia_sys) consists of the Vulkan types and command signatures generated from the [Vulkan API Registry](https://github.com/KhronosGroup/Vulkan-Docs/blob/main/xml/vk.xml). If you want to use the raw Vulkan API and are willing to handle function loading yourself you can use this crate.

[`vulkanalia`](https://docs.rs/vulkanalia/latest/vulkanalia) offers a fairly thin wrapper around `vulkanalia-sys` that handles function loading for you and makes the Vulkan API somewhat less error prone and more idiomatic to use from Rust. For a detailed overview of how `vulkanalia` wraps the Vulkan API, see the `API Concepts` section of the `Overview` chapter of the Vulkan tutorial which can be found [here](https://kylemayes.github.io/vulkanalia/overview.html#api-concepts).

#### But wait, there's more!

- Support for the Vulkan video extensions
- Support for creating custom layers (see the [`layer`](layer/) directory for a simple example layer)
- [`vulkanalia-vma`](https://docs.rs/vulkanalia-vma/latest/vulkanalia_vma) &ndash; an integration of [Vulkan Memory Allocator](https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator) with [`vulkanalia`](https://github.com/KyleMayes/vulkanalia)

## Cargo Features

The `vulkanalia` crate has the following notable non-default Cargo features:

* `libloading` (**non-default**) &ndash; enables integration with [`libloading`](https://crates.io/crates/libloading) (adds the [`LibloadingLoader`](https://docs.rs/vulkanalia/latest/vulkanalia/loader/struct.LibloadingLoader.html) struct which can be used to load the initial Vulkan commands from a Vulkan shared library)
* `window` (**non-default**) &ndash; enables integration with [`raw-window-handle`](https://crates.io/crates/raw-window-handle) (adds the [`window`](https://docs.rs/vulkanalia/latest/vulkanalia/window/index.html) module which can be used to create surfaces for windows from libraries that support `raw-window-handle` (e.g., [`winit`](https://crates.io/crates/winit))
* `provisional` (**non-default**) &ndash; enables access to [provisional Vulkan extensions](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/provisional-headers.html) (**WARNING:** these extensions are not guaranteed to be backwards compatible and are not intended to be used in production applications)

By default, the `vulkanalia-sys` and `vulkanalia` crates depend on the Rust standard library. However, by disabling the default features for these crates, you can use either of these crates in a `no_std` environment. If you do this, the following features are of note:

* `no_std_error` (**non-default**): enables implementations of the [`Error` trait](https://doc.rust-lang.org/beta/core/error/trait.Error.html) for various error types in `vulkanalia` and `vulkanalia-sys` when the default `std` feature is not enabled (the usage of the `Error` trait in `core` is [not yet stable](https://github.com/rust-lang/rust/issues/103765) and requires the `core-error` feature to be enabled)

## Example

See the `examples` directory for an implementation of the classic triangle example using `vulkanalia`.
