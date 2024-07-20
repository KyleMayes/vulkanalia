## [0.24.0] - 2024-07-20

### Changed
- Removed `LoaderError` implementation for `String` and added `StringLoaderError` struct

### Bindings Updates
- [Fixes to proposal document markup and for Antora](https://github.com/KhronosGroup/Vulkan-Docs/commit/08d90ab2b71c21e8507fc0fb80461eaafa844d1c)
- [Update proposals links to current spec / refpages](https://github.com/KhronosGroup/Vulkan-Docs/commit/7ccda9da169c801c91e690b178c6b00f79cd02f3)
- [Adjust proposal links to point to the docs site from the Antora build](https://github.com/KhronosGroup/Vulkan-Docs/commit/fad27392b1818a7356e8a65a77ce9e723d4a7f3f)
- [Resolve some typos (#2333)](https://github.com/KhronosGroup/Vulkan-Docs/commit/cce62b5fe90a31a3b33786cfe29d60f5d4a27bda)
- [April 13, 2024 Vulkan 1.3.282 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/315493e7b2ed31e3d33f124f95a4b1c0cdbfaf84)
- [Try to resolve warnings in Actions UI by upgrading all actions from @v3 to @v4 (#2352)](https://github.com/KhronosGroup/Vulkan-Docs/commit/d5803f193bf0c3b819a401993a309de127497f7d)
- [April 19, 2024 Vulkan 1.3.283 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/dedb71a7edc6d5af3f9bfd5e2ef53814de999ef7)
- [Refactor proposals into a separate Antora component (#2358)](https://github.com/KhronosGroup/Vulkan-Docs/commit/046a1f32cba893f38fc743c44461093726e0eac5)
- [Revert "Refactor proposals into a separate Antora component (#2358)" (#2360)](https://github.com/KhronosGroup/Vulkan-Docs/commit/beec3ada55b028cd1103378dda0492de11bca987)
- [Refactor 'proposals' into a separate Antora component 'features' (#2361)](https://github.com/KhronosGroup/Vulkan-Docs/commit/8a8a32f67d08e0df011a4406135b0ec7047d90e6)
- [May 5, 2024 Vulkan 1.3.284 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/ff188a8ffa950a010422c4c4b277f06db10d0dce)
- [Update refpage short descriptions of VkAttachmentLoad/StoreOp to be consistent with spec body (#2364)](https://github.com/KhronosGroup/Vulkan-Docs/commit/632ab091cbe467646d0cccbd5a96d4db75e243d5)
- [Add a <<fundamentals-strings, String Representation>> section clarifying (#2365)](https://github.com/KhronosGroup/Vulkan-Docs/commit/6a32915da9286589a52be43aac344f2011cf6b9b)
- [Clarify that the KHR and EXT load_store_op_none extensions were not promoted to Vulkan 1.3 (#2367)](https://github.com/KhronosGroup/Vulkan-Docs/commit/2a114fbd375b355f3f164048429ebbda022ed73e)
- [May 10, 2024 Vulkan 1.3.285 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/8fc686b6321f035dc4a589e5ec1dc67208a71ac2)
- [Add a "constants" type= attribute to <enums> tags (#2366)](https://github.com/KhronosGroup/Vulkan-Docs/commit/ce03761fd6cc1ab8f54c9f785fffb13f492d7d04)
- [Replace <feature number> attribute with <feature depends> (#2331)](https://github.com/KhronosGroup/Vulkan-Docs/commit/8651835d4c709897a590cfc3073ad5121127cdff)
- [Display Timing Queries: Clarify behaviour of VK_PRESENT_MODE_FIFO_RELAXED_KHR (#2363)](https://github.com/KhronosGroup/Vulkan-Docs/commit/5fb5568cece7a13cca68adab38e742c4f3769df7)
- [May 31, 2024 Vulkan 1.3.286 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/ed4ba0242beb89a1795d6084709fa9e713559c94)
- [June 7, 2024 Vulkan 1.3.287 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/ae3e824136b336fe99025eaf0cd55d073c6e6a0a)
- [June 14, 2024 Vulkan 1.3.288 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/bf457079af2a599bbcb018cf5d032429a7121d97)
- [June 28, 2024 Vulkan 1.3.289 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/7bb606eb87cde1d34f65f36f4d4c6f2c78f072c8)
- [July 12, 2024 Vulkan 1.3.290 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/043260d06e96c8586492be41cc109945e92a69ff)
- [July 19, 2024 Vulkan 1.3.291 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/4b01c384d9fc4ffff9bb7dc18a1b76d57c6d7d4f)

## [0.23.0] - 2024-03-29

### Changed
- Bumped MSRV to 1.65
- Bumped `winit` to `0.29`
- Added `no_std` compability for `vulkanalia` and `vulkanalia-sys` crates
- Make all extendable output structs parameters in command wrappers (see [#213](https://github.com/KyleMayes/vulkanalia/issues/213) for details)

### Fixed
- [Fixed crash when zero extension or layers enabled](https://github.com/KyleMayes/vulkanalia/issues/254)

### Added
- Added support for Vulkan video extensions
- Added `vulkanalia::prelude::v1_3` module (a prelude module for Vulkan 1.3+)

### Bindings Updates
- [September 22, 2023 Vulkan 1.3.265 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/4871ab9e57fb07f98bf016cb10a3088924976e29)
- [Add a driver ID for AGXV (Asahi) (#2238)](https://github.com/KhronosGroup/Vulkan-Docs/commit/2b587d9c4adc65429a0063704d1ed6abe79ddb78)
- [Add riscv64 to 64-bit platforms (#2236)](https://github.com/KhronosGroup/Vulkan-Docs/commit/0df82fed1670db44e15e6f281715d71eae0b6299)
- [September 29, 2023 Vulkan 1.3.266 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/e5dbdd580cf0696db8ed0aeb0736e0f512d9bbe1)
- [October 6, 2023 Vulkan 1.3.267 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/66b95bd350a014c7a4dcdcd309206b571750deb3)
- [Update the Antora site landing page to include API spec revision number (#2243)](https://github.com/KhronosGroup/Vulkan-Docs/commit/aeef71db7529d267eaa61d53a9d6a41f573723b6)
- [Fix URL](https://github.com/KhronosGroup/Vulkan-Docs/commit/28987b83440387845058bb4e49ac3a8fc9658ede)
- [Make chapter appear in Antora build](https://github.com/KhronosGroup/Vulkan-Docs/commit/7aaf705f1881a28cc4db9e975c3614e0b532e3c7)
- [Remove Vulkan SC-specific file from Antora build](https://github.com/KhronosGroup/Vulkan-Docs/commit/69ae59d3aeb4fa6430d455f3bfaabe4b85475b68)
- [Add a spec build test (#2248)](https://github.com/KhronosGroup/Vulkan-Docs/commit/a0797ed156639f3fc54be9e640a54dd382191a5a)
- [October 13, 2023 Vulkan 1.3.268 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/bb6783481f96d778772ea9607e2991c27e2bbe96)
- [Style guide wording cleanup](https://github.com/KhronosGroup/Vulkan-Docs/commit/fbc4d519231fead5f2bd07b4fac8198d5f13659f)
- [Add/correct a couple of optional attributes (#2252)](https://github.com/KhronosGroup/Vulkan-Docs/commit/f3f8a5da40fcaf05f979917393522b107ebef92c)
- [Merge branch 'main' of github.com:KhronosGroup/Vulkan-Docs into github-main](https://github.com/KhronosGroup/Vulkan-Docs/commit/708ab3d685dbea7d6b7bd6a0cb45315031ddb262)
- [October 20, 2023 Vulkan 1.3.269 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/463f8c616f49fb83ae4736de0d84b0048a7c76e2)
- [Add VK_KHR_timeline_semaphore dependency for VK_NV_low_latency2. (#2264)](https://github.com/KhronosGroup/Vulkan-Docs/commit/60ff2191347572d218e945e9b8e2c42e9e0b57b7)
- [November 9, 2023 Vulkan 1.3.270 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/b4792eab92a1d132ef95b56a7681cc6af69b570e)
- [November 17, 2023 Vulkan 1.3.271 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/dbad946f7edc9137dbb972ea8e271592e3fb9746)
- [December 1, 2023 Vulkan 1.3.272 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/73207ec2babd7fce921920499ac4427583d7293d)
- [Add missing bitwidth attribute for VkPhysicalDeviceSchedulingControlsFlagBitsARM (#2280)](https://github.com/KhronosGroup/Vulkan-Docs/commit/6bfb45cff8daf3a1a64292bf1469c2b9c17fced1)
- [December 8, 2023 Vulkan 1.3.273 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/f8d76125ca22ec65dfcaedc7177e204f11ad7c7b)
- [Remove incorrect optional attribute on VkCudaLaunchInfoNV::pParams and pExtras (#2263)](https://github.com/KhronosGroup/Vulkan-Docs/commit/6f7dc055750238cb8eea0d38de3a0f636e5ca115)
- [make VkRenderPassStripeBeginInfoARM::pStripeInfos const (#2279)](https://github.com/KhronosGroup/Vulkan-Docs/commit/e70469ca36a4b653acea11b9d1c65c531149384c)
- [December 19, 2023 Vulkan 1.3.274 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/542718ca30aef2c3c5afbca41de6424bd37e1808)
- [Update VK_EXT_image_compression_control support query example (#2285)](https://github.com/KhronosGroup/Vulkan-Docs/commit/24e57ed4c775a7e8558dc1e3f55f322d3fdafc28)
- [January 5, 2024 Vulkan 1.3.275 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/9b94c27d65dc7d11e50a7c00581b89f1983d34ff)
- [Add epub generation (#2286)](https://github.com/KhronosGroup/Vulkan-Docs/commit/7abf6f00e099cb24061d2c7cc9bfd02a4c1282c1)
- [January 25, 2024 Vulkan 1.3.276 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/bf11e6d4ca4b71ae311bb925ae68d215fbe09a86)
- [February 1, 2024 Vulkan 1.3.277 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/8c4e3f27f12060fd0bd1302393c4808fee6ce81f)
- [Update for Vulkan-Docs 1.3.277](https://github.com/KhronosGroup/Vulkan-Headers/commit/5a5c9a643484d888873e32c5d7d484fae8e71d3d)
- [Fix invalid parameter section generation for function pointers and function prototypes (#2304)](https://github.com/KhronosGroup/Vulkan-Docs/commit/0d651979dc96d0a9a60c89744d94e6d833501ef2)
- [Fix some improperly named files.](https://github.com/KhronosGroup/Vulkan-Docs/commit/d99193d3fcc4b2a0dacc0a9d7e4951ea611a3e96)
- [Add a new extension: VK_EXT_map_memory_placed (#1906)](https://github.com/KhronosGroup/Vulkan-Docs/commit/0c355559d6ca3d92edbaafb27347b0cbe34f3b2f)
- [February 16, 2024 Vulkan 1.3.278 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/2e3aca8d6a3a6d52a9d904d0511a1c5e57a09e0f)
- [Snapshot from internal repo with various Antora fixes so we can try them out](https://github.com/KhronosGroup/Vulkan-Docs/commit/03d750e1c0fa243207be8bb7af826b8b35467562)
- [Add a few overlooked Antora-related files.](https://github.com/KhronosGroup/Vulkan-Docs/commit/4b80f8fd153e396f77912ca3346d84170e24c0ec)
- [March 1, 2024 Vulkan 1.3.279 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/b8d528e16a5380fc1ffb8e94297d311fe81a6791)
- [Use proper\full QFOT name (#2320)](https://github.com/KhronosGroup/Vulkan-Docs/commit/30a9c4c1927c40c613c91b1a57a88f7b887a987d)
- [March 8, 2024 Vulkan 1.3.280 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/69ba4fefbafa045d0fd1b07060b768b3b1c115cc)
- [Always generate failure code section (#2325)](https://github.com/KhronosGroup/Vulkan-Docs/commit/7b42841ef9d3b3873ca4a62feb191e4debaa1aab)
- [Update ChangeLog.adoc for 1.3.280](https://github.com/KhronosGroup/Vulkan-Docs/commit/2d3b82c86c023b27022b3e272c3e35cb14af4d1d)
- [March 22, 2024 Vulkan 1.3.281 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/d6029cc2b7499faf2e7857420ec4996fc5cb0a50)

## [0.22.0] - 2023-09-15

### Fixed
- Fixed `push_next` not adding a chain of structs properly ([#188](https://github.com/KyleMayes/vulkanalia/issues/188))

### Added
- Added `InputChainStruct` and `OutputChainStruct` traits which are implemented for structs that can be part of pointer chains
- Added `chain` module to `vulkanalia` with pointer chain helpers (e.g. iteration)
- Added `bytecode` module to `vulkanalia` with SPIR-V bytecode buffer helper

### Bindings Updates
- [June 1, 2023 Vulkan 1.3.252 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/5718db0a370b5bd91e6cf2268a3dc2af9cfc15d1)
- [June 9, 2023 Vulkan 1.3.253 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/2f4ef8371aa309f91954536996582066900ef2a0)
- [Add spec for VK_QNX_external_memory_screen_buffer (#2138)](https://github.com/KhronosGroup/Vulkan-Docs/commit/b5543e9029775140325b0d9593c83eb9f3afe831)
- [June 16, 2023 Vulkan 1.3.254 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/7c839d85b51cb639444183c586e53b2b2ab21e2e)
- [fix VK_FORMAT_B10G11R11_UFLOAT_PACK32 (#2145)](https://github.com/KhronosGroup/Vulkan-Docs/commit/6da1e1089e0a12f383ac79340e237cec79bf1bf1)
- [fix VK_FORMAT_B5G5R5A1_UNORM_PACK16 (#2147)](https://github.com/KhronosGroup/Vulkan-Docs/commit/491f5dad442d3884148028c9e4251ec48d3578f2)
- [June 23, 2023 Vulkan 1.3.255 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/012db30fd16929f9fd30dfbc2a7c86e048d64015)
- [fix VK_FORMAT_R64G64 and VK_FORMAT_BC snorm formats (#2156)](https://github.com/KhronosGroup/Vulkan-Docs/commit/b258d9b1e5f347b04092d5b8df4eaff73cdfa9e6)
- [June 30, 2023 Vulkan 1.3.256 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/3dae5d7fbf332970ae0a97d5ab05ae5db93e62f0)
- [July 7, 2023 Vulkan 1.3.257 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/2b7d9c9f65cde43580573a677490c3c7d099c09f)
- [July 21, 2023 Vulkan 1.3.258 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/2c1e07a72b43f53cc7866c55ae32afe97d933621)
- [July 21, 2023 Vulkan 1.3.259 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/3da7531f2f9d48993ab627c02a866479d5163ba4)
- [July 28, 2023 Vulkan 1.3.260 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/12ab5855b1608e4b05b270e0dedecd1b1a5458f8)
- [August 4, 2023 Vulkan 1.3.261 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/b9aad705f0d9e5e6734ac2ad671d5d1de57b05e0)
- [August 25, 2023 Vulkan 1.3.262 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/6952ad062f031e3fc99b53ef28e009cbeca64583)
- [September 2, 2023 Vulkan 1.3.263 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/4e32929f96e5af77fe5202b959f18c4ad7a06bbc)
- [Merge branch 'layered-driver-proposal' of github.com:jenatali/Vulkan-Docs into jenatali-layered-driver-proposal](https://github.com/KhronosGroup/Vulkan-Docs/commit/11c3bfcc46b856e66a32215ebc475516225cf01b)
- [September 8, 2023 Vulkan 1.3.264 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/3d25cd996b16ae0781e15735df5762efa9f71010)

## [0.21.0] - 2023-05-28

### Changed
- Fixed multi-dimensional array code generation (only affected the `TransformMatrixKHR::matrix` field)

### Bindings Updates
- [May 28, 2023 Vulkan 1.3.251 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/45b5ba66f8128be493745da2d45f0bb407d9296a)

## [0.20.0] - 2023-05-26

### Changed
- Fixed functions that take a `create_infos` slice to return a `Vec` of objects instead of a single object (e.g., `vkCreateGraphicsPipelines`)

### Bindings Updates
- [April 20, 2023 Vulkan 1.3.248 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/9fff8b252a3688c0231fa78709084bbe677d3bf7)
- [April 27, 2023 Vulkan 1.3.249 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/58e747b4b453a787c1043f30fbf6669b3ba29e0e)
- [May 4, 2023 Vulkan 1.3.250 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/8f2ca84561db97d13717827dc26d600e1c2ac719)
- [Sync internal extension reservation with github per #2124.](https://github.com/KhronosGroup/Vulkan-Docs/commit/ce88c3111f2c79b72b10143aee324851ba71545f)
- [Add new empty extension number 530. (#2128)](https://github.com/KhronosGroup/Vulkan-Docs/commit/c5bb6f9a65da0ddf26addb44ab4894af50f81b14)
- [Add MSFT tag and reserve an extension number (#2129)](https://github.com/KhronosGroup/Vulkan-Docs/commit/15e7eac8d34dd8b101b5a422632263f07163244a)
- [Fix typo in SPV_INTEL_shader_integer_functions2 (#2132)](https://github.com/KhronosGroup/Vulkan-Docs/commit/abefa12bc383e6c340e0f744a6a26ecf67cbb15d)

## [0.19.0] - 2023-04-26

### Changed
- Fixed parameters and return type for `KhrAccelerationStructureExtension::get_acceleration_structure_build_sizes_khr` method ([#150](https://github.com/KyleMayes/vulkanalia/issues/150))
- Fixed `push_next` methods in builder structs ([#149](https://github.com/KyleMayes/vulkanalia/issues/149))

### Bindings Updates
- [April 12, 2023 Vulkan 1.3.247 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/32be5bd5594e3973db0b7e5e7a950becce070a82)

## [0.18.0] - 2023-04-04

### Changed
- Changed `vk_window::create_surface` to take [separated `raw-window-handle` traits](https://github.com/KyleMayes/vulkanalia/pull/130)

### Bindings Updates
- [December 19, 2022 Vulkan 1.3.238 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/9a2e576a052a1e65a5d41b593e693ff02745604b)
- [January 19, 2023 Vulkan 1.3.239 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/4941f94e8e36e99e6e1fe430c9e2569dfb6c1937)
- [January 26, 2023 Vulkan 1.3.240 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/b33bd816a24012b0ac51e8b05567cc221171ccf1)
- [February 16, 2023 Vulkan 1.3.241 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/1b1c4dd43a35341c8c8e82ad985ed66d8beff5ba)
- [Only parenthesize formal arguments of VK_DEFINE*HANDLE macros in vulkansc api (#2068)](https://github.com/KhronosGroup/Vulkan-Docs/commit/bc4666ebd41f591c92f6e67d6b4ff8a346fc6b1c)
- [February 26, 2023 Vulkan 1.3.242 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/f3e686fc22251102713a2a1a1cd071f60ab6efd8)
- [Update vk.xml (#2076)](https://github.com/KhronosGroup/Vulkan-Docs/commit/992febb8e01a0c602c425b2139bc9449fd7f9ba9)
- [March 12, 2023 Vulkan 1.3.243 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/84ccde6b1b92820feee54c8b7577387b2cf1ce17)
- [March 17, 2023 Vulkan 1.3.244 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/7beccb60daac498d700a09763945719c31510ab6)
- [March 24, 2023 Vulkan 1.3.245 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/729dda35a16d57611636d6684a1fcb27f7e0a722)
- [March 31, 2023 Vulkan 1.3.246 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/ce847fd14cc3a81751329352ce505501c46ce35e)

## [0.17.0] - 2022-12-14

### Bindings Updates
- [September 22, 2022 Vulkan 1.3.229 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/07560427085af01aafb985bf0cffa959bf85cb8c)
- [September 29, 2022 Vulkan 1.3.230 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/ac3762095459e0190a75c433af1f85d2f6023d44)
- [October 13, 2022 Vulkan 1.3.231 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/7a319840243ea33aa4caa42cdce0143b150e02bb)
- [October 27, 2022 Vulkan 1.3.232 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/7dcf16f3b4a1118ff92207316b68145446f08bb3)
- [November 3, 2022 Vulkan 1.3.233 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/42d3f5a641810c52bbe53c0f049be1130af07f2f)
- [November 10, 2022 Vulkan 1.3.234 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/3abd2bbc91a7e74186a8acc82268d726cee6a731)
- [Reserve a driver ID for NVK (#1983)](https://github.com/KhronosGroup/Vulkan-Docs/commit/34a7173fee63c2c15e2414044710a0a69bbd8984)
- [November 17, 2022 Vulkan 1.3.235 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/f4eb3e9a7acdef9ab62ac9af954a50409895ac6d)
- [Merge branch 'main' of github.com:KhronosGroup/Vulkan-Docs into github-main](https://github.com/KhronosGroup/Vulkan-Docs/commit/d4987d251a1f63f184ea6ed9b20f5125cfd6a2d5)
- [December 1, 2022 Vulkan 1.3.236 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/db2b908b59b7774da9aa8b07baed4f5d018ae4d9)
- [Re-add commit from PR #1983, which was somehow lost in the internal /](https://github.com/KhronosGroup/Vulkan-Docs/commit/d1442602716ef83e6e8deedfab63c2022c0fba13)
- [December 8, 2022 Vulkan 1.3.237 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/0e28607b55dc1eee4ff80967f1e0a7e99356d075)

## [0.16.0] - 2022-09-25

### Bindings Updates
- [May 24, 2022 Vulkan 1.3.215 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/9b5562187a8ad72c171410b036ceedbc450153ba)
- [fix typo: Initialization of **an** object has failed (#1865)](https://github.com/KhronosGroup/Vulkan-Docs/commit/84e29520c4ce42aec2412a98b53139fa45aa2af2)
- [June 2, 2022 Vulkan 1.3.216 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/899dd1c16d5de69bd24e108f393d134fa2989512)
- [VK_EXT_non_seamless_cube_map (#1628)](https://github.com/KhronosGroup/Vulkan-Docs/commit/9b5cef69af1e417a008325b4c8b0cdca35e26cc6)
- [June 9, 2022 Vulkan 1.3.217 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/307906bddb5fab543dcf6d7fff737abeb15f8b10)
- [June 16, 2022 Vulkan 1.3.218 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/067a92d083e6a7ec97ef3bcc34e5b75fa71eec79)
- [Add a new driver id VK_DRIVER_ID_MESA_DOZEN (#1877)](https://github.com/KhronosGroup/Vulkan-Docs/commit/afbecff18cdb2515af523b81521d66a86d79d16f)
- [June 30, 2022 Vulkan 1.3.219 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/080f66a96b61419b8663872d6cab6ce68a3123e8)
- [Fix XML tagging of VkShaderModuleCreateInfo and add explicit VU (#1884)](https://github.com/KhronosGroup/Vulkan-Docs/commit/e561f993cc2d67ef101f2b4f6e185183f1c47b9f)
- [July 7, 2022 Vulkan 1.3.220 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/29f44f378cdeb1b96b47ce00fb32f49954f2d29d)
- [Add pd2 requirement for VK_EXT_blend_operation_advanced (#1887)](https://github.com/KhronosGroup/Vulkan-Docs/commit/f977b487253872ce3f5fd3dc1ae688f4f4b59677)
- [July 14, 2022 Vulkan 1.3.221 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/9aeaebb24b6faa1f05c7a95b0328beecbe195927)
- [July 21, 2022 Vulkan 1.3.222 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/8dcc7469b01529600b712596a5a48ec8c710e228)
- [July 28, 2022 Vulkan 1.3.223 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/9ecfc67442754c9e4c4fecf5e61c48483608a074)
- [August 4, 2022 Vulkan 1.3.224 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/5ca346bf39db36ff06e24686f0a815754c5f7c16)
- [August 18, 2022 Vulkan 1.3.225 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/1f6a09901674b320bff7cf36c364a461cf1069a6)
- [Minor changes before the great renaming](https://github.com/KhronosGroup/Vulkan-Docs/commit/f8e2c9c4a897a4082594c96c049139b3b480b891)
- [September 1, 2022 Vulkan 1.3.226 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/135da3a538263ef0d194cab25e2bb091119bdc42)
- [September 8, 2022 Vulkan 1.3.227 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/09d120580acb942d9cc3a96c863815a05990893c)
- [September 15, 2022 Vulkan 1.3.228 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/355367640f2eabd2a0d492010a0afc166696aa72)

## [0.15.0] - 2022-05-18

### Bindings Updates
- [January 25, 2022 Vulkan 1.3.204 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/15d807ce4839d8feb523ca5c133a42a2aa448ade)
- [xml/vk: Reference underlying type instead of typedef in `structextends` (#1746)](https://github.com/KhronosGroup/Vulkan-Docs/commit/3422ccac57350871eb5a087acf09ab932db71263)
- [February 4, 2022 Vulkan 1.3.205 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/9e4c847a9e93cccd5f46e173c3ccf752b89c35df)
- [Remove unused VkPrivateDataSlotCreateFlagBits type (#1765)](https://github.com/KhronosGroup/Vulkan-Docs/commit/a5afab5927383ae0ff4034855bbff475386213cc)
- [February 17, 2022 Vulkan 1.3.206 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/d80b6159f6d69398dbeae52aa1080e47ae96fe47)
- [add author id for NZXT Inc. (#1785)](https://github.com/KhronosGroup/Vulkan-Docs/commit/5cb2ea0da12d887e8887741ac51b248db2aa3400)
- [March 8, 2022 Vulkan 1.3.207 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/75c276d6fa83a3f1cbf8b3da50b9278c479be021)
- [March 15, 2022 Vulkan 1.3.208 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/e88e27f2cda370444f87fc724412a384cb958824)
- [March 23, 2022 Vulkan 1.3.209 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/fff65570f345288361958ff530e284de27b1a657)
- [March 29, 2022 Vulkan 1.3.210 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/45af5eb1f66898c9f382edc5afd691aeb32c10c0)
- [April 5, 2022 Vulkan 1.3.211 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/2a31e99cbaa07dba4e2036c0bfe76aa3ebe8b2a4)
- [April 21, 2022 Vulkan 1.3.212 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/0a80073e9078eae4266ca779d91e36e91c49a54c)
- [May 10, 2022 Vulkan 1.3.213 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/71decb7f868d3891974eab139f03c7c6c87fea4d)
- [Mark VkSubresourceLayout2EXT as returnedonly. (#1848)](https://github.com/KhronosGroup/Vulkan-Docs/commit/94f4dc1d6384f271875fcade4fe8223383ad1c6c)
- [Add VK_KHR_depth_stencil_resolve dependency to VK_KHR_dynamic_rendering (#1831)](https://github.com/KhronosGroup/Vulkan-Docs/commit/c18adb4f8887f6e8845b09bb8158df85faa6f066)
- [May 17, 2022 Vulkan 1.3.214 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/028789841d7ba54cabc678a128e7b98d30c0b4cd)

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
- Bumped MSRV to 1.64
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
