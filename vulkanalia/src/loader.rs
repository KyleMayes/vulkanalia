// SPDX-License-Identifier: Apache-2.0

//! Vulkan function loaders.

use alloc::boxed::Box;
use alloc::string::String;

#[cfg(all(feature = "no_std_error", not(feature = "std")))]
use core::error;
#[cfg(feature = "std")]
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

/// An error produced by a failure to load a Vulkan function.
#[cfg(any(feature = "std", feature = "no_std_error"))]
pub trait LoaderError: error::Error + Send + Sync + 'static {}
/// An error produced by a failure to load a Vulkan function.
#[cfg(not(any(feature = "std", feature = "no_std_error")))]
pub trait LoaderError: core::fmt::Debug + core::fmt::Display + Send + Sync + 'static {}

/// A string [`LoaderError`].
#[derive(Clone, Debug)]
pub struct StringLoaderError(String);

impl core::fmt::Display for StringLoaderError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(any(feature = "std", feature = "no_std_error"))]
impl error::Error for StringLoaderError {}

impl LoaderError for StringLoaderError {}

impl<'a> From<&'a str> for Box<dyn LoaderError> {
    fn from(value: &'a str) -> Self {
        Box::new(StringLoaderError(value.into())) as Box<dyn LoaderError>
    }
}

impl From<String> for Box<dyn LoaderError> {
    fn from(value: String) -> Self {
        Box::new(StringLoaderError(value)) as Box<dyn LoaderError>
    }
}

/// A Vulkan function loader.
pub trait Loader {
    /// Loads a Vulkan function.
    ///
    /// # Safety
    ///
    /// See implementations for safety documentation.
    unsafe fn load(&self, name: &[u8]) -> Result<extern "system" fn(), Box<dyn LoaderError>>;
}

#[cfg(feature = "libloading")]
mod libloading_loader {
    use std::boxed::Box;
    use std::ffi::OsStr;
    use std::mem;

    use libloading::{Error, Library, Symbol};

    use super::*;

    impl LoaderError for Error {}

    impl From<Error> for Box<dyn LoaderError> {
        fn from(value: Error) -> Self {
            Box::new(value) as Box<dyn LoaderError>
        }
    }

    /// A Vulkan function loader that uses `libloading`.
    #[derive(Debug)]
    pub struct LibloadingLoader(Library);

    impl LibloadingLoader {
        /// Constructs a Vulkan function loader from a Vulkan dynamic library.
        ///
        /// # Safety
        ///
        /// See [`libloading::Library::new`](https://docs.rs/libloading/0.7/libloading/struct.Library.html#method.new).
        #[inline]
        pub unsafe fn new(filename: impl AsRef<OsStr>) -> Result<Self, Error> {
            Library::new(filename).map(Self)
        }

        /// The loaded Vulkan dynamic library.
        #[inline]
        pub fn library(&self) -> &Library {
            &self.0
        }
    }

    impl Loader for LibloadingLoader {
        /// Loads a Vulkan function.
        ///
        /// # Safety
        ///
        /// See [`libloading::Library::get`](https://docs.rs/libloading/0.7/libloading/struct.Library.html#method.get).
        #[inline]
        unsafe fn load(&self, name: &[u8]) -> Result<extern "system" fn(), Box<dyn LoaderError>> {
            let symbol: Symbol<Option<extern "C" fn()>> = self.0.get(name)?;
            let pointer = symbol.try_as_raw_ptr().ok_or("missing function pointer")?;
            Ok(mem::transmute::<*mut core::ffi::c_void, extern "system" fn()>(pointer))
        }
    }
}

#[cfg(feature = "libloading")]
pub use self::libloading_loader::*;
