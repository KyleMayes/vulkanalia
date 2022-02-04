## [0.15.0] - UNRELEASED

### Bindings Updates
- [January 25, 2022 Vulkan 1.3.204 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/15d807ce4839d8feb523ca5c133a42a2aa448ade)
- [xml/vk: Reference underlying type instead of typedef in `structextends` (#1746)](https://github.com/KhronosGroup/Vulkan-Docs/commit/3422ccac57350871eb5a087acf09ab932db71263)
- [February 4, 2022 Vulkan 1.3.205 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/9e4c847a9e93cccd5f46e173c3ccf752b89c35df)

## [0.14.0] - 2022-01-18

### Bindings Updates
- [December 20, 2021 Vulkan 1.2.203 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/ac23aa229fd9b8ea06aa99cf07b79cdc15af406f)
- [Add a new driver id VK_DRIVER_ID_MESA_VENUS (#1733)](https://github.com/KhronosGroup/Vulkan-Docs/commit/5a1f1580c773f0db57507e6de14a34e4613f3a38)

### Fixed
- Fixed signatures for commands that take opaque pointers as arguments
- Fixed signatures for commands that always return values successfully (#81)

## [0.13.0] - 2021-12-14

### Bindings Updates
- [Remaining newly introduced `vk-video` types (#1663)](https://github.com/KhronosGroup/Vulkan-Docs/commit/36cec8e02d9d73ce7c88203567d3196064db8605)
- [November 2, 2021 Vulkan 1.2.197 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/2a43f68f2841054d7f2fb6a44c637c533a549dbb)
- [November 9, 2021 Vulkan 1.2.198 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/3a4dc41cc86b4215e9995f20f2be744a106887d5)
- [November 16, 2021 Vulkan 1.2.199 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/83c7507600618d8748bb911dfd8c3d9b4fabaca0)
- [Push internal reservation of extension 436 so public reservation can claim 437](https://github.com/KhronosGroup/Vulkan-Docs/commit/a5598d1a2e80757ceab78fc39919c5c05db81319)
- [reserve extension (#1693)](https://github.com/KhronosGroup/Vulkan-Docs/commit/71af58ed5123950f20252764147bac5ab7ae5a40)
- [November 23, 2021 Vulkan 1.2.200 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/9ed8caef1a0b5abe9778adb39feff435b2328f1b)
- [November 30, 2021 Vulkan 1.2.201 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/d70e01c0be7b8a7d20b186b30b29a75b18bba75d)
- [December 7, 2021 Vulkan 1.2.202 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/48b76697118c275337e49836259f0571b3410d84)

### Changed
- Updated generator to properly generate fields for bitfields (#73)

## [0.12.0] - 2021-10-30

### Bindings Updates
- [September 7, 2021 Vulkan 1.2.191 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/ef0df9ab7cfd5081018b21174d10bc1d47cb734c)
- [September 14, 2021 Vulkan 1.2.192 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/a09f9efe488cdd97c0362622fc1187f09302689e)
- [Add new VK_DRIVER_IDs for upcoming Mesa drivers. (#1642)](https://github.com/KhronosGroup/Vulkan-Docs/commit/4eb2b9e1b8990862cc4c491f209f4fe9c3ae9210)
- [September 21, 2021 Vulkan 1.2.193 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/129f6f69dcbc00184d771d2da969b4ed394ac80c)
- [vk.xml: Adding len attribute to VkCuModuleCreateInfoNVX struct member pData. (#1646)](https://github.com/KhronosGroup/Vulkan-Docs/commit/b1b9113f0c605573024a8a0e61adf0a576563145)
- [September 28, 2021 Vulkan 1.2.194 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/a0960966d565bdfc6e853a0bc471e58fdbd374ef)
- [October 5, 2021 Vulkan 1.2.195 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/9dc6dfbb7aea0999f4108ad0faeee099d0adb173)
- [vk.xml: Reorder `VkFormatFeatureFlags2KHR` xml attributes for alignment (#1653)](https://github.com/KhronosGroup/Vulkan-Docs/commit/a80e21cf5efdc44cfba179d5763f3ced563338c7)
- [October 13, 2021 Vulkan 1.2.196 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/0714884a45ffe66c2746b4f53c9035bf72c32dfd)

### Fixed
- Fixed `StringArray` equality and hashing (previously equality and hashing would compare or hash the *entire* array of characters instead of stopping at the first null terminator which resulted in string arrays that contain identical null-terminated strings not being considered equal because there were different bytes after the first null terminator)

### Changed
- Changed `StringArray::from_bytes` constructor method to truncate the byte string instead of panicking when the byte string is too long to fit in the string array

### Added
- Added `StringArray::new` constructor method
- Added `StringArray::from_cstr` constructor method
- Added `StringArray::as_array` method
- Added `StringArray::as_bytes` method
- Added `StringArray::as_cstr` method

### Removed
- Removed `StringArray` tuple struct field

## [0.11.0] - 2021-09-01

### Bindings Updates
- [July 20, 2021 Vulkan 1.2.185 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/62e02d5234fbbbd997d7c71099373a273a27dbf8)
- [August 3, 2021 Vulkan 1.2.186 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/cd8da6a515811c4ff9e1f10a6d6ce4ab968333fb)
- [August 3, 2021 Vulkan 1.2.187 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/907c556530050b0f4af073753335f23c885c02bf)
- [August 10, 2021 Vulkan 1.2.188 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/f1eda2cb1656363ccd67e07f5654a80fc13b47af)
- [August 29, 2021 Vulkan 1.2.190 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/e57864f6a51a2d56dc7f012b21957755c59aafa8)

### Fixed
- Fixed composite bitflag values (e.g., `CullModeFlags::FRONT_AND_BACK`)

### Changed
- Bumped MSRV to 1.51
- Added `Send` and `Sync` requirement to `Loader` error type
- Changed type of stored layer names to `vk::ExtensionName` instead of `CString`
- Added constants, constructor, and additional conversions to `Version`
- Changed `patch` field to default to `0` in `Version` instead of using `Option`
- Reworked byte and string array types in Vulkan structs
- Removed dummy `EMPTY` bitflags used in empty `bitflags!` instances
- Fixed names of bitflags with numbers in the name (`AccessFlags2KHR` and `PipelineStageFlags2KHR`)

### Added
- Added `Debug` requirement to extension struct traits (e.g., `ExtendsDeviceCreateInfo`)

## [0.10.0] - 2021-07-10

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
