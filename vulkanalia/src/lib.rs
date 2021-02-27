// SPDX-License-Identifier: Apache-2.0

//! Vulkan bindings for Rust.

pub mod loader;
pub mod vk;

#[cfg(feature = "window")]
pub mod window;

use std::collections::HashSet;
use std::error;
use std::ffi::{CStr, CString};
use std::fmt;
use std::mem;
use std::os::raw::c_char;
use std::slice;
use std::sync::Arc;

use self::loader::Loader;
use self::prelude::v1_0::*;
use self::vk::{DeviceCommands, EntryCommands, InstanceCommands};

/// Preludes.
pub mod prelude {
    /// Vulkan 1.0 prelude.
    pub mod v1_0 {
        pub use crate::vk;
        pub use crate::vk::{ConvertCStr, Handle, HasBuilder};
        pub use crate::vk::{DeviceV1_0, EntryV1_0, InstanceV1_0};
        pub use crate::{Device, Entry, Instance, VkResult, VkSuccessResult};
    }

    /// Vulkan 1.1 prelude.
    pub mod v1_1 {
        pub use crate::prelude::v1_0::*;
        pub use crate::vk::{DeviceV1_1, EntryV1_1, InstanceV1_1};
    }

    /// Vulkan 1.2 prelude.
    pub mod v1_2 {
        pub use crate::prelude::v1_1::*;
        pub use crate::vk::{DeviceV1_2, EntryV1_2, InstanceV1_2};
    }
}

/// The result of a executing a fallible Vulkan command.
pub type VkResult<T> = Result<T, vk::ErrorCode>;
/// The result of a executing a fallible Vulkan command with multiple success codes.
pub type VkSuccessResult<T> = Result<(T, vk::SuccessCode), vk::ErrorCode>;

/// A Vulkan version.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    /// The major version (the `x` in `x.y.z`).
    pub major: u32,
    /// The minor version (the `y` in `x.y.z`).
    pub minor: u32,
    /// The patch version (the `z` in `x.y.z`).
    ///
    /// If the version of Vulkan is not at least `1.1.0`, then the patch version
    /// will not be known due to the `vkEnumerateInstanceVersion` function not
    /// being available.
    pub patch: Option<u32>,
}

impl Default for Version {
    #[inline]
    fn default() -> Self {
        Self {
            major: 1,
            minor: 0,
            patch: None,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(patch) = self.patch {
            write!(f, "{}.{}.{}", self.major, self.minor, patch)
        } else {
            write!(f, "{}.{}.?", self.major, self.minor)
        }
    }
}

impl From<u32> for Version {
    #[inline]
    fn from(version: u32) -> Self {
        Self {
            major: vk::version_major(version),
            minor: vk::version_minor(version),
            patch: Some(vk::version_patch(version)),
        }
    }
}

/// A Vulkan entry point.
#[derive(Clone)]
pub struct Entry {
    loader: Arc<dyn Loader>,
    get_instance: vk::PFN_vkGetInstanceProcAddr,
    get_device: vk::PFN_vkGetDeviceProcAddr,
    commands: EntryCommands,
}

impl Entry {
    /// Loads a Vulkan entry point using a Vulkan function loader.
    ///
    /// # Safety
    ///
    /// The [`Loader::load`] method will be called on the supplied [`Loader`]
    /// implementation to load the entry commands so the safety requirements of
    /// [`Loader::load`] for the [`Loader`] implementation used must be upheld.
    #[inline]
    pub unsafe fn new(
        loader: impl Loader + 'static,
    ) -> Result<Self, Box<dyn error::Error + 'static>> {
        let loader = Arc::new(loader);

        let raw = loader.load(b"vkGetInstanceProcAddr")?;
        let get_instance = mem::transmute::<_, vk::PFN_vkGetInstanceProcAddr>(raw);
        let raw = loader.load(b"vkGetDeviceProcAddr")?;
        let get_device = mem::transmute::<_, vk::PFN_vkGetDeviceProcAddr>(raw);

        let load = |n| mem::transmute(get_instance(vk::Instance::null(), n));
        let commands = EntryCommands::load(load);

        Ok(Self {
            loader,
            get_instance,
            get_device,
            commands,
        })
    }

    /// Gets the instance-level version of this Vulkan entry point.
    #[inline]
    pub fn version(&self) -> VkResult<Version> {
        let name = b"vkEnumerateInstanceVersion\0".as_ptr() as *const c_char;
        let raw = unsafe { (self.get_instance)(vk::Instance::null(), name) };
        let enumerate: Option<vk::PFN_vkEnumerateInstanceVersion> = unsafe { mem::transmute(raw) };
        if let Some(enumerate) = enumerate {
            let mut version = 0;
            match unsafe { enumerate(&mut version) } {
                vk::Result::SUCCESS => Ok(Version::from(version)),
                error => Err(error.into()),
            }
        } else {
            Ok(Version::default())
        }
    }

    /// Creates a Vulkan instance using this Vulkan entry point.
    ///
    /// # Safety
    ///
    /// The [`Loader::load`] method will be called on the supplied [`Loader`]
    /// implementation to load the instance commands so the safety requirements
    /// of [`Loader::load`] for the [`Loader`] implementation used must be
    /// upheld.
    #[inline]
    pub unsafe fn create_instance(
        &self,
        info: &vk::InstanceCreateInfo,
        allocator: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<Instance> {
        let handle = EntryV1_0::create_instance(self, info, allocator)?;
        let load = |n| mem::transmute((self.get_instance)(handle, n));
        let commands = InstanceCommands::load(load);
        let extensions = get_extensions(info.enabled_extension_count, info.enabled_extension_names);
        let layers = get_layers(info.enabled_layer_count, info.enabled_layer_names);
        Ok(Instance {
            get_device: self.get_device,
            handle,
            commands,
            extensions,
            layers,
        })
    }
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entry")
    }
}

unsafe impl Send for Entry {}
unsafe impl Sync for Entry {}

/// A Vulkan instance.
#[derive(Clone)]
pub struct Instance {
    get_device: vk::PFN_vkGetDeviceProcAddr,
    handle: vk::Instance,
    commands: InstanceCommands,
    extensions: HashSet<vk::ExtensionName>,
    layers: HashSet<CString>,
}

impl Instance {
    /// Gets the loaded extensions for this Vulkan instance.
    #[inline]
    pub fn extensions(&self) -> &HashSet<vk::ExtensionName> {
        &self.extensions
    }

    /// Gets the loaded layers for this Vulkan instance.
    #[inline]
    pub fn layers(&self) -> &HashSet<CString> {
        &self.layers
    }

    /// Creates a Vulkan device using this Vulkan instance.
    ///
    /// # Safety
    ///
    /// The [`Loader::load`] method will be called on the supplied [`Loader`]
    /// implementation to load the device commands so the safety requirements of
    /// [`Loader::load`] for the [`Loader`] implementation used must be upheld.
    #[inline]
    pub unsafe fn create_device(
        &self,
        physical_device: vk::PhysicalDevice,
        info: &vk::DeviceCreateInfo,
        allocator: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<Device> {
        let handle = InstanceV1_0::create_device(self, physical_device, info, allocator)?;
        let load = |n| mem::transmute((self.get_device)(handle, n));
        let commands = DeviceCommands::load(load);
        let extensions = get_extensions(info.enabled_extension_count, info.enabled_extension_names);
        let layers = get_layers(info.enabled_layer_count, info.enabled_layer_names);
        Ok(Device {
            handle,
            commands,
            extensions,
            layers,
        })
    }
}

impl fmt::Debug for Instance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Instance")
            .field("handle", &self.handle)
            .field("extensions", &self.extensions)
            .field("layers", &self.layers)
            .finish()
    }
}

unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}

/// A Vulkan device.
#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    commands: DeviceCommands,
    extensions: HashSet<vk::ExtensionName>,
    layers: HashSet<CString>,
}

impl Device {
    /// Gets the loaded extensions for this Vulkan device.
    #[inline]
    pub fn extensions(&self) -> &HashSet<vk::ExtensionName> {
        &self.extensions
    }

    /// Gets the loaded layers for this Vulkan device.
    #[inline]
    pub fn layers(&self) -> &HashSet<CString> {
        &self.layers
    }
}

impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Device")
            .field("handle", &self.handle)
            .field("extensions", &self.extensions)
            .field("layers", &self.layers)
            .finish()
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

#[inline]
fn get_extensions(num_strings: u32, strings: *const *const c_char) -> HashSet<vk::ExtensionName> {
    unsafe { slice::from_raw_parts(strings, num_strings as usize) }
        .iter()
        .map(|s| vk::ExtensionName::from_cstr(unsafe { CStr::from_ptr(*s) }))
        .collect()
}

#[inline]
fn get_layers(num_layers: u32, layers: *const *const c_char) -> HashSet<CString> {
    unsafe { slice::from_raw_parts(layers, num_layers as usize) }
        .iter()
        .map(|l| unsafe { CStr::from_ptr(*l) }.into())
        .collect()
}
