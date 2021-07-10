## [0.10.0] - UNRELEASED

### Bindings Updates
- [June 7, 2021 Vulkan 1.2.180 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/b4e8cd820b2487bc892b391fb26b49501473a6a6)
- [Add specification for VK_EXT_physical_device_drm (#1356)](https://github.com/KhronosGroup/Vulkan-Docs/commit/7e30d02e5de56191c6f964d0d8a267d84e62306e)
- [June 21, 2021 Vulkan 1.2.182 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/1b32e9d5beeee44d81520cbf35193047e60dbf6b)
- [June 28, 2021 Vulkan 1.2.183 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/1934d6f0eee2ca4a2794fed548521b11a33343a6)
- [July 5, 2021 Vulkan 1.2.184 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/02c265fb0af1eb5e82651e6c001d3790101adf96)

## [0.9.0] - 2021-06-01

### Bindings Updates
- [April 19, 2021 Vulkan 1.2.176 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/67f599afee77b0e598e7a325f13b9878edcacdfd)
- [April 26, 2021 Vulkan 1.2.177 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/88da24862cf91d692801c6ae64665d26d39f06c4)
- [May 10, 2021 Vulkan 1.2.178 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/dfe3bcd0e1b7239e2d6ae8b6afe563780a711854)
- [May 24, 2021 Vulkan 1.2.179 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/e840a78772b61abaf6aabcc4e3f44ee9d7afce55)

## [0.8.0] - 2021-04-15

### Bindings Updates
- [April 13, 2021 Vulkan 1.2.175 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/b82ae46bb82c7a522509fd7b8f3d92a311c1b5a5)

### Changed
- Changed wrapper methods that return extendable structs to take a mutable reference to the struct to allow the caller to extend the struct

### Removed
- Removed bindings for unsupported extensions

## [0.7.0] - 2021-04-08

### Bindings Updates
- [March 8, 2021 Vulkan 1.2.172 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/d2d9ed985ef74f3c5252ac713367b98815e9188f)
- [March 21, 2021 Vulkan 1.2.173 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/0d5fb72f39c4282edf1754ce5eed0654f7960cb4)
- [March 29, 2021 Vulkan 1.2.174 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/3fefdc503242ce529330c51a869ed99a069a3010)

## [0.6.0] - 2021-02-27

### Changed
- Marked commands as `unsafe`
- Marked function loading functions as `unsafe`
- Changed `window::get_required_instance_extensions` to take a raw window handle instead of a Vulkan entry point and to always successfully return a slice of extension name references

## [0.5.0] - 2021-02-26

### Bindings Updates
- [Finalizing VK_QNX_screen_surface extension (#1449)](https://github.com/KhronosGroup/Vulkan-Docs/commit/256c004b56b981a25a12088d087f086700428de8)

### Changed
- Moved extension metadata to `vulkanalia-sys`

## [0.4.0] - 2021-02-24

### Bindings Updates
- [November 30, 2020 Vulkan 1.2.163 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/a48d8432aacf8a23de3c471d6fa074d0a326cfcc)
- [Add Vendor ID for PoCL (http://portablecl.org/) (#1411)](https://github.com/KhronosGroup/Vulkan-Docs/commit/d39f6fc50e778e9982e83e08ab6b7711f81432fe)
- [December 7, 2020 Vulkan 1.2.164 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/8f718b4194ed1e0a572d37072e5558dd9ceabcb0)
- [December 14, 2020 Vulkan 1.2.165 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/ffbc67c499b92e864ad51275e606468975b5e397)
- [Claim the extension number and add the new vendor. (#1425)](https://github.com/KhronosGroup/Vulkan-Docs/commit/4b93f2838c8864923be81ddc00589a26003394a6)
- [January 19, 2021 Vulkan 1.2.167 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/d342f27444fbc31244458a23994aed818a4902ba)
- [January 25, 2021 Vulkan 1.2.168 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/cd10a156f111cfc930c1821cc41b987661a22c76)
- [February 15, 2021 Vulkan 1.2.170 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/815e07c850d642d996292f5cdec25d41ecdff0d0)

### Changed
- Bumped `libloading` version to `0.7`
- Bumped `metal` version to `0.21`
- Renamed `libloading_` Cargo feature to `libloading`
- Removed `winit_` Cargo feature in favor of `window` feature
- Added `provisional` Cargo feature which exposes provisional extensions
- Changed commands that returned `Vec<c_void>` to return `Vec<u8>` instead

### Added
- Added extension metadata to extension traits

## [0.2.3] - 2020-11-23

### Bindings Updates
- [November 23, 2020 Vulkan 1.2.162 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/c5d94a31766e91607473ca0049a293e3f632c6ff)

## [0.2.2] - 2020-11-17

### Bindings Updates
- [November 9, 2020 Vulkan 1.2.160 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/f90136facacd25f016e523064f03713bdfe1b22d)

## [0.2.1] - 2020-11-04

### Bindings Updates
- [November 1, 2020 Vulkan 1.2.159 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/9adbc1846ddad202a0584f5c03a1916cf9801179)

### Added
- Added builder methods for array length fields associated with optional array fields

## [0.2.0] - 2020-10-30

### Bindings Updates
- [October 19, 2020 Vulkan 1.2.158 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/9fd8fd599b47a67b2eb078b2f5c9e6a2adc922a4)

### Changed
- Changed wrapper methods with possible success codes other than `SUCCESS` or `INCOMPLETE` to return `VkSuccessResult` (new typedef) instead of `VkResult`
- Changed builder methods for void pointers to take references to unconstrained generic types instead (e.g., `&'b mut T` for `*mut c_void`)
- Changed builder methods for opaque arrays (e.g., `*const c_void` with a corresponding length field) to take `u8` slices instead (e.g., `&'b [u8]` for `*const c_void`)

### Added
- Added `SuccessCode` and `ErrorCode` enums
- Added `VkSuccessResult` type alias

## [0.1.0] - 2020-10-19
- Initial release
