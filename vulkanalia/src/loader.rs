// SPDX-License-Identifier: Apache-2.0

//! Vulkan function loaders.

use std::error;

/// The default Vulkan shared library filename on the current platform (Windows).
#[cfg(windows)]
pub const LIBRARY: &str = "vulkan-1.dll";

/// The default Vulkan shared library filename on the current platform (Android).
#[cfg(target_os = "android")]
pub const LIBRARY: &str = "libvulkan.so";

/// The default Vulkan shared library filename on the current platform (iOS / macOS).
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub const LIBRARY: &str = "libvulkan.dylib";

/// The default Vulkan shared library filename on the current platform (BSD / Linux).
#[cfg(all(
    unix,
    not(target_os = "android"),
    not(target_os = "ios"),
    not(target_os = "macos")
))]
pub const LIBRARY: &str = "libvulkan.so.1";

/// A Vulkan function loader.
pub trait Loader {
    /// Loads a Vulkan function.
    #[allow(clippy::missing_safety_doc)]
    unsafe fn load(
        &self,
        name: &[u8],
    ) -> Result<extern "system" fn(), Box<dyn error::Error + 'static>>;
}

#[cfg(feature = "libloading")]
mod libloading_loader {
    use std::error;
    use std::ffi::OsStr;
    use std::mem;

    use libloading::{Error, Library, Symbol};

    use super::*;

    /// A Vulkan function loader that uses `libloading`.
    #[derive(Debug)]
    pub struct LibloadingLoader(Library);

    impl LibloadingLoader {
        /// Constructs a Vulkan function loader from a Vulkan dynamic library.
        #[inline]
        pub fn new(filename: impl AsRef<OsStr>) -> Result<Self, Error> {
            unsafe { Library::new(filename).map(Self) }
        }

        /// The loaded Vulkan dynamic library.
        #[inline]
        pub fn library(&self) -> &Library {
            &self.0
        }
    }

    impl Loader for LibloadingLoader {
        #[inline]
        unsafe fn load(
            &self,
            name: &[u8],
        ) -> Result<extern "system" fn(), Box<dyn error::Error + 'static>> {
            let symbol: Symbol<Option<extern "C" fn()>> = self.0.get(name)?;
            let symbol = symbol.lift_option().ok_or("missing function")?;
            Ok(mem::transmute(symbol))
        }
    }
}

#[cfg(feature = "libloading")]
pub use self::libloading_loader::*;
