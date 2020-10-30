## [0.2.0] - UNRELEASED

### Changed
- Changed wrapper methods with possible success codes other than `SUCCESS` or `INCOMPLETE` to return `VkSuccessResult` (new typedef) instead of `VkResult`
- Changed builder methods for void pointers to take references to unconstrained generic types instead (e.g., `&'b mut T` for `*mut c_void`)
- Changed builder methods for opaque arrays (e.g., `*const c_void` with a corresponding length field) to take `u8` slices instead (e.g., `&'b [u8]` for `*const c_void`)

### Added
- Added `SuccessCode` and `ErrorCode` enums
- Added `VkSuccessResult` type alias
- Added support for `VK_KHR_fragment_shading_rate` extension
- Added support for `VK_KHR_shader_terminate_invocation` extension

## [0.1.0] - 2020-10-19
- Initial release
