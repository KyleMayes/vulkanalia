## [0.33.0] - UNRELEASED

### Changed
- Bumped MSRV to 1.88

### Bindings Updates
- [Generate refpage aliases in Antora site build (#2610)](https://github.com/KhronosGroup/Vulkan-Docs/commit/1626c4df3f8f3b415d5a0cad387151a4a6b0498c)
- [Fix VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE 'returnedonly="true"' (#2607)](https://github.com/KhronosGroup/Vulkan-Docs/commit/8e30b6ca28038dc37a7c610b62f2d99747a91123)
- [November 7, 2025 Vulkan 1.4.332 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/09d464f85360160747c4ad6dc5a9841e15822aa5)

## [0.32.0] - 2025-10-30

### Added
- Added `vk::layer::NegotiateLayerStructType` struct
- Added `vk::layer::NegotiateLayerInterface` struct
- Added `vk::DispatchableHandle::dispatch_key` method

### Bindings Updates
- [Merge branch 'main' of github.com:KhronosGroup/Vulkan-Docs into github-main](https://github.com/KhronosGroup/Vulkan-Docs/commit/d38733398a4bec673bd5d24c904d4739a45f020d)
- [Create Antora spec nav.adoc from vkspec.adoc (#2589)](https://github.com/KhronosGroup/Vulkan-Docs/commit/6299b8ddede3fd8e37baae8255c41d9eec9c834d)
- [Clean up `VK_QCOM_tile_shading.adoc` proposal (#2590)](https://github.com/KhronosGroup/Vulkan-Docs/commit/ce1eb7c4eb02db9182bed362024418556315bc09)
- [Fix link typo per #2590.](https://github.com/KhronosGroup/Vulkan-Docs/commit/7fe2b623f13dabb885e35b953dd690f01093d4bb)
- [Fix formatting in indirect_memory_copy_common.adoc (#2593)](https://github.com/KhronosGroup/Vulkan-Docs/commit/fa8768a1a02615c887ff7d8859671bac76363035)
- [Fix malformed curved double quotes in `writing.adoc` (#2592)](https://github.com/KhronosGroup/Vulkan-Docs/commit/1992dbb91c95ee373b0fb9d8f792dabe3eb0127d)
- [Fix some spelling errors (#2591)](https://github.com/KhronosGroup/Vulkan-Docs/commit/540e08ec7ccfd8b1e0574f331765526813966570)
- [Add step to Github CI to match gitlab CI (#2599)](https://github.com/KhronosGroup/Vulkan-Docs/commit/a6fbb686ebb04b2262ad1b6951eccc1e63ce26ab)
- [October 10, 2025 Vulkan 1.4.329 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/ac3d0660c45ecf86b2952b298d125270228150d1)
- [Reorganization Makefile.release comments](https://github.com/KhronosGroup/Vulkan-Headers/commit/89268a6d17fc87003b209a1422c17ab288be99a0)
- [CMake: Don't install C++ module headers if module build is disabled](https://github.com/KhronosGroup/Vulkan-Headers/commit/37057b4756df4009ad85803bd2e06ec8a3bb1bca)
- [Further update after fix in Vulkan-Hpp repository](https://github.com/KhronosGroup/Vulkan-Headers/commit/a01329f307fa6067da824de9f587f292d761680b)
- [Update for Vulkan-Docs 1.4.324](https://github.com/KhronosGroup/Vulkan-Headers/commit/a1c3885d4e1f7a585bd1fd6635f90f43f6b9516a)
- [Update for Vulkan-Docs 1.4.325](https://github.com/KhronosGroup/Vulkan-Headers/commit/2e0a6e699e35c9609bde2ca4abb0d380c0378639)
- [GN Build: Add a way to skip xlib includes](https://github.com/KhronosGroup/Vulkan-Headers/commit/2efaa559ff41655ece68b2e904e2bb7e7d55d265)
- [build(deps): bump actions/checkout from 4 to 5](https://github.com/KhronosGroup/Vulkan-Headers/commit/be73614e1dafe7dc4f4867305d4d1b91fe9d1649)
- [Update for Vulkan-Docs 1.4.326](https://github.com/KhronosGroup/Vulkan-Headers/commit/d1cd37e925510a167d4abef39340dbdea47d8989)
- [Update for Vulkan-Docs 1.4.327](https://github.com/KhronosGroup/Vulkan-Headers/commit/d7a7044334ad88485c0a6113d1bf51520ac9e541)
- [Update for Vulkan-Docs 1.4.328](https://github.com/KhronosGroup/Vulkan-Headers/commit/9a0f3099c8a9607a7c0f3127d8abfdc19a93e8c5)
- [Add instruction to cd to correct directory before tag-branch](https://github.com/KhronosGroup/Vulkan-Headers/commit/8ed12a9178e73ed67fed295a0ba65769895fa2c7)
- [fixed faulty var name..](https://github.com/KhronosGroup/Vulkan-Headers/commit/a4f8ada9f4f97c45b8c89c57997be9cebaae65d2)
- [propagate DVULKAN_HEADERS_DISABLE_MODULE_STD value to CTests](https://github.com/KhronosGroup/Vulkan-Headers/commit/dcfd96691c5133d16065097dd51bf607c227dcfa)
- [reset module flag](https://github.com/KhronosGroup/Vulkan-Headers/commit/f4c405e02a6e1a68f5562d628e381d330207fbef)
- [set VULKAN_HEADERS_DISABLE_MODULE_STD=ON in CI, while tooling is unable to handle import std](https://github.com/KhronosGroup/Vulkan-Headers/commit/78eb9bd05697e0c1490f7be3355dbf6d2cd499c4)
- [add option VULKAN_HEADERS_DISABLE_MODULE_STD to avoid automatic use of import std](https://github.com/KhronosGroup/Vulkan-Headers/commit/ef7df1629cc65bcbf83cbec93bfb751649c76396)
- [build(deps): bump fsfe/reuse-action from 5 to 6](https://github.com/KhronosGroup/Vulkan-Headers/commit/4eebce1723aced9afa272c38dfd4f82a786ed9ad)
- [Revert "CMake: Don't install C++ module headers if module build is disabled"](https://github.com/KhronosGroup/Vulkan-Headers/commit/f5fad0a209a2e09eb5e622d930d5eade1a235b13)
- [Update for Vulkan-Docs 1.4.329](https://github.com/KhronosGroup/Vulkan-Headers/commit/33d7f512583b8de44d1b6384aa1cf482f92e53e9)
- [Push Vulkan Base changes without doing a full spec update, to verify](https://github.com/KhronosGroup/Vulkan-Docs/commit/ac874e1acdeb42856e1e6e87f4abaedc6f93f438)
- [October 24, 2025 Vulkan 1.4.330 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/805d2ca04d00074ee4debe327fc7d7eb3016f67e)
- [Update for Vulkan-Docs 1.4.230](https://github.com/KhronosGroup/Vulkan-Headers/commit/ee3b5caaa7e372715873c7b9c390ee1c3ca5db25)
- [October 31, 2025 Vulkan 1.4.331 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/48034cad6dd20ad6f2b01194436f87eecf71a9bb)

## [0.31.0] - 2025-10-09

> __This release contains breaking changes!__<br>
> Most users will only be affected by the changes to the extension traits.<br>
> Replace usages of the removed extension traits with the new instance-level or device-level command traits like shown in the examples below.

### Added
- Added implementation of `vk::EntryV1_0` for `vk::EntryCommands`
- Added implementation of `vk::InstanceV1_0` for tuples of `vk::InstanceCommands` and `vk::Instance`
- Added implementation of `vk::DeviceV1_0` for tuples of `vk::DeviceCommands` and `vk::Device`
- Added `Version::V1_3_0` and `Version::V1_4_0` associated constants
- Added `layer` module to `vulkanalia-sys` which provides layer interface types
- Added `Entry::from_commands` constructor method
- Added `Instance::from_created` constructor method
- Added `Device::from_created` constructor method
- Added `Device::physical_device` method

### Changed
- Non-1.0 command traits are now implemented for any type that implements the 1.0 command trait
  - For example: `vk::DeviceV1_1` is implemented for any type that implements `vk::DeviceV1_0`
- **BREAKING:** `vk::DeviceCommands::load` no longer takes an instance-level loader function
- **BREAKING:** Moved instance-level commands added by device extensions from `DeviceCommands` to `InstanceCommands`
- **BREAKING:** Replaced extension traits with one extension commands trait per extension and type of command added by the extension
  - For example:
    - `vk::KhrSurfaceExtension` is replaced by **one** trait (since this extension only adds instance-level commands):
      - `vk::KhrSurfaceExtensionInstanceCommands`
    - `vk::ExtMultiDrawExtension` is replaced by **one** trait (since this extension only adds device-level commands)
      - `vk::ExtMultiDrawExtensionDeviceCommands`
    - `vk::KhrSwapchainExtension` is replaced by **two** traits (since this extension adds instance-level _and_ device-level commands):
      - `vk::KhrSwapchainExtensionInstanceCommands` (provides instance-level commands)
      - `vk::KhrSwapchainExtensionDeviceCommands` (provides device-level commands)

## [0.30.0] - 2025-10-04

### Added
- Added `DispatchableHandle` and `NonDispatchableHandle` trait aliases for `Handle` trait
- Added `Send` and `Sync` implementations for Vulkan structs containing pointers

### Changed
- Bumped `cocoa` to `0.26`
- Bumped `metao` to `0.32`

### Fixed
- Fixed signatures for commands that take slices of slices
- Fixed type of `vk::WHOLE_SIZE` to be `u64`

### Bindings Updates
- [gn: Add vpn9std headers to GN build](https://github.com/KhronosGroup/Vulkan-Headers/commit/1d6c53f65443ceeb97d3bdc695aaecc7ea6cc441)
- [Update for Vulkan-Docs 1.4.318](https://github.com/KhronosGroup/Vulkan-Headers/commit/b39ab380a44b6c8df462c34e976ea9ce2d2c336b)
- [Update for Vulkan-Docs 1.4.319](https://github.com/KhronosGroup/Vulkan-Headers/commit/10739e8e00a7b6f74d22dd0a547f1406ff1f5eb9)
- [Update for Vulkan-Docs 1.4.320](https://github.com/KhronosGroup/Vulkan-Headers/commit/16cedde3564629c43808401ad1eb3ca6ef24709a)
- [Update for Vulkan-Docs 1.4.321](https://github.com/KhronosGroup/Vulkan-Headers/commit/2cd90f9d20df57eac214c148f3aed885372ddcfe)
- [Build script updates](https://github.com/KhronosGroup/Vulkan-Headers/commit/a2ab2a76125d7fc076ae1398a2c29a4cf0586e43)
- [Update release Makefile to refer to ../Vulkan-Docs/Vulkan-Hpp instead of](https://github.com/KhronosGroup/Vulkan-Headers/commit/4fcc684c7ed8535b7018830f3f7c3e6c5b8b89f3)
- [Update for Vulkan-Docs 1.4.](https://github.com/KhronosGroup/Vulkan-Headers/commit/7cff847503174e2049b08253ee5f30428866fea3)
- [Update release Makefile again](https://github.com/KhronosGroup/Vulkan-Headers/commit/f69f0433bae0b30598380ef0420b9d2d02dbac4d)
- [Chzange log for July 18, 2025 Vulkan 1.4.323 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/5d386163f25cca10d2af7be2bbea07d1e6fb52ba)
- [Fix REVISION expansion in Makefile.release](https://github.com/KhronosGroup/Vulkan-Headers/commit/00a752019b1dc1419164a24be069e4eb03cef01b)
- [Update for Vulkan-Docs 1.4.323](https://github.com/KhronosGroup/Vulkan-Headers/commit/088a00d81d1fc30ff77aacf31485871aebec7cb2)
- [Fixed missing type reference `VkPipelineCacheCreateFlagBits` (#2558)](https://github.com/KhronosGroup/Vulkan-Docs/commit/40d05246e3c29fb81522dd4c76e3fd28943a1fc9)
- [August 1, 2025 Vulkan 1.4.324 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/f8cc250718793afb6c15db69980ba5ff7c24920f)
- [August 1, 2025 Vulkan 1.4.324 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/0a5f2a25dabcb343417cc8c8a91a015e9d8fc2db)
- [Merge branch 'main' of github.com:KhronosGroup/Vulkan-Docs into github-main](https://github.com/KhronosGroup/Vulkan-Docs/commit/f2678d2d6dbf7d76527671ca357b12c22b11c5b9)
- [Editorial update.](https://github.com/KhronosGroup/Vulkan-Docs/commit/2972f5070950d07bab91f7f7bc9553a75129bad0)
- [Add test matrix to hpp-compile CI stage to target multiple C++ standards (#2566)](https://github.com/KhronosGroup/Vulkan-Docs/commit/9f9fe5848047990219934e441646e42040bc51f9)
- [Add newlines at EOF to avoid gitlab conflicts](https://github.com/KhronosGroup/Vulkan-Docs/commit/701b4e19ade2744005fcc4855f5287a38e473b3a)
- [August 8, 2025 Vulkan 1.4.325 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/72cd1f587fe55c80873fe6430d667056048a5113)
- [Framework for adding refpages to docs.vulkan.org (#2553)](https://github.com/KhronosGroup/Vulkan-Docs/commit/31a398b34ec314e62a2d96cea68227430b86e4ae)
- [Fix typo: missing `be` in `must be less than` (#2572)](https://github.com/KhronosGroup/Vulkan-Docs/commit/1d815e3a93a24b112ba4d360ac316015b0795ae3)
- [Update present_wait.adoc (#2569)](https://github.com/KhronosGroup/Vulkan-Docs/commit/f6e45bbfa4284695ad4d67ecafb082417f04bef4)
- [August 29, 2025 Vulkan 1.4.326 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/9c6d565f72ba6929c239c3e20f90b6375acad3bd)
- [Fix typos in "Acceleration Structures" chapter (#2580)](https://github.com/KhronosGroup/Vulkan-Docs/commit/fa19a89518c936181ec4b8044dc10c4b3226de66)
- [Resolve malformed cross-reference in `spirvenv.adoc` appendix (#2581)](https://github.com/KhronosGroup/Vulkan-Docs/commit/d66177a6ddc6a626e32826036bdd2d55704cf0e4)
- [Remove extraneous backticks (#2583)](https://github.com/KhronosGroup/Vulkan-Docs/commit/6fd91a4743abe2d7ae82f429e20e7510b2effd7f)
- [Fix mention of "C++" in "API Registry" (#2582)](https://github.com/KhronosGroup/Vulkan-Docs/commit/d6e7f443f8abca606b3f312b84bc8903d14e08e8)
- [Modify genRef.py to respect the 'type' attribute for 'consts' instead of (#2584)](https://github.com/KhronosGroup/Vulkan-Docs/commit/d0b382477df0120f24277ab345124b0796fd8803)
- [September 19, 2025 Vulkan 1.4.327 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/d2893929769fe6e3a7d9ef96672da551c35d4cd9)
- [Merge branch 'main' of github.com:KhronosGroup/Vulkan-Docs into github-main](https://github.com/KhronosGroup/Vulkan-Docs/commit/ef2dd227fb7310db03a7d2d40579226c4280b84b)
- [Fix bunch of issues in `ChangeLog.adoc` (#2587)](https://github.com/KhronosGroup/Vulkan-Docs/commit/dad8cec7a33e672d3a575072b75da6261686e2d2)
- [September 25, 2025 Vulkan 1.4.328 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/39961b9409730ee4edb8b5ad0ab1fc9b552f9044)

## [0.29.0] - 2025-07-14

### Added
- Added support for Android windowing
- Added `vulkanalia::prelude::v1_4` module (a prelude module for Vulkan 1.4+)

### Bindings Updates
- [Add VUID for relaxed control barrier with storage class semantics (#2495)](https://github.com/KhronosGroup/Vulkan-Docs/commit/e21e798a5e106519cb50cd7f12aab6bf55a30b45)
- [March 7, 2025 Vulkan 1.4.310 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/ab5fa3690263718c4c4dc8f7110ba4a52528e063)
- [Update for Vulkan-Docs 1.4.306](https://github.com/KhronosGroup/Vulkan-Headers/commit/e43027aa41c4f51b12d79aeae53ff608951c36ec)
- [Update for Vulkan-Docs 1.4.307](https://github.com/KhronosGroup/Vulkan-Headers/commit/39f924b810e561fd86b2558b6711ca68d4363f68)
- [cmake: Update minimum version to 3.22.1](https://github.com/KhronosGroup/Vulkan-Headers/commit/234c4b7370a8ea3239a214c9e871e4b17c89f4ab)
- [Update for Vulkan-Docs 1.4.309](https://github.com/KhronosGroup/Vulkan-Headers/commit/952f776f6573aafbb62ea717d871cd1d6816c387)
- [GN build: Define VK_USE_PLATFORM_XLIB_KHR](https://github.com/KhronosGroup/Vulkan-Headers/commit/0f0cfd88d7e6ece3ca6456df692f0055bde94be7)
- [Update for Vulkan-Docs 1.4.310](https://github.com/KhronosGroup/Vulkan-Headers/commit/cacef3039d277c448c89336290ec3937270b0996)
- [add VkPushConstantRange link in VkPipelineLayoutCreateInfo (#2508)](https://github.com/KhronosGroup/Vulkan-Docs/commit/33c2f285d9100da435a9bc040dea42bddebd0b62)
- [cmake: Rename Vulkan-Module to Vulkan-HppModule](https://github.com/KhronosGroup/Vulkan-Headers/commit/d64e9e156ac818c19b722ca142230b68e3daafe3)
- [All _identically defined_ link to glossary-identically-defined (#2510)](https://github.com/KhronosGroup/Vulkan-Docs/commit/876240690b465fda5300929940f8d171133ce5e9)
- [March 21, 2025 Vulkan 1.4.311 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/7719d5eb8eb1df49469cdf684c6b370d0ee56a34)
- [Update for Vulkan-Docs 1.4.311](https://github.com/KhronosGroup/Vulkan-Headers/commit/78c359741d855213e8685278eb81bb62599f8e56)
- [adjusted fnc name](https://github.com/KhronosGroup/Vulkan-Headers/commit/2ac81691baf291e7f4aad07596d7073974dbc4dd)
- [simplified cpp module to not require function loading](https://github.com/KhronosGroup/Vulkan-Headers/commit/a523083f727e2b0585047557659a32c6cd0fa195)
- [replaced minimal example for modules](https://github.com/KhronosGroup/Vulkan-Headers/commit/cb28cd18196e1cdb81bbdca044e4ab13f126aed6)
- [fixed linewrap on windows and CXX compiler on ubuntu](https://github.com/KhronosGroup/Vulkan-Headers/commit/24ce771ec10046bd128c22e2e2cb2a17e3f8ade3)
- [Added licenses and used included headers in minimal examples](https://github.com/KhronosGroup/Vulkan-Headers/commit/cafcda93602251fc5fe306bc4523b5ffcb7cb8eb)
- [added c++20 workflows for modules in ubuntu and windows](https://github.com/KhronosGroup/Vulkan-Headers/commit/2b1e9e74097979fe81805ed2dcb3cd6952385e2b)
- [minimal compilation setups for Vulkan::Hpp and Vulkan::HppModule](https://github.com/KhronosGroup/Vulkan-Headers/commit/a72b4466b3ddee502b1723faa9cebd1665455195)
- [merged subdirectory tests](https://github.com/KhronosGroup/Vulkan-Headers/commit/fdc7404466c30e6977ff27013ec34cf39e0b2cd4)
- [integration tests for cxx modules](https://github.com/KhronosGroup/Vulkan-Headers/commit/9417d66dcf5b94a2c2c86374f45965818ca9eff0)
- [April 4, 2025 Vulkan 1.4.312 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/ec56ad8828af2b089de2fc7a64cf59c588d8c753)
- [Update for Vulkan-Docs 1.4.312](https://github.com/KhronosGroup/Vulkan-Headers/commit/5ceb9ed481e58e705d0d9b5326537daedd06b97d)
- [Fix "If If" in Texel Replacement section (#2521)](https://github.com/KhronosGroup/Vulkan-Docs/commit/1049a264d1067cb3f9f0f05aebf1464c347b750b)
- [ignore Jetbrains config directories](https://github.com/KhronosGroup/Vulkan-Headers/commit/110b6c989ccb4e874089db777e2b54eb9abb5670)
- [April 18, 2025 Vulkan 1.4.313 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/19b765119a9ddef1034e95442f82f94235167f36)
- [Update for Vulkan-Docs 1.4.313](https://github.com/KhronosGroup/Vulkan-Headers/commit/409c16be502e39fe70dd6fe2d9ad4842ef2c9a53)
- [mark Vulkan-HppModule target as OBJECT](https://github.com/KhronosGroup/Vulkan-Headers/commit/e2e53a724677f6eba8ff0ce1ccb64ee321785cbd)
- [fixed FILE_SET path, separated install targets somewhat](https://github.com/KhronosGroup/Vulkan-Headers/commit/7b82913567665f75f92449a56be1549bd333c980)
- [installing module and exporting via CXX_MODULES_DIRECTORY](https://github.com/KhronosGroup/Vulkan-Headers/commit/5407d73452d368f03ebd51597d623a616c66ee64)
- [Apply suggestions from code review](https://github.com/KhronosGroup/Vulkan-Headers/commit/8e12a6c7ff788924055ac771ba8ddb8f4485f913)
- [revert](https://github.com/KhronosGroup/Vulkan-Headers/commit/9fe6a5263a1737acb2ecc47accd3347282453ee3)
- [reposition FILE_SET key](https://github.com/KhronosGroup/Vulkan-Headers/commit/140c8c711b40a4f62129e52b5529ae01d0e4233b)
- [split up install again to accodomate older cmake versions](https://github.com/KhronosGroup/Vulkan-Headers/commit/47d1f104c4031653acc93293046dbf0d7108da8a)
- [add FILE_SET module to export](https://github.com/KhronosGroup/Vulkan-Headers/commit/cc4e52bad8a4754d8219fa5126f6eff3767b8a94)
- [append to install target list](https://github.com/KhronosGroup/Vulkan-Headers/commit/c4060c5c91f2e4b81e593671a2eabed3639e9424)
- [enable FindPackage testing for vk module](https://github.com/KhronosGroup/Vulkan-Headers/commit/1c846fb1a83f25c8f1abab7bba883828cec31e43)
- [use vk-hpp func for version](https://github.com/KhronosGroup/Vulkan-Headers/commit/41af963e422e14fe3b7f1d6dde984bbcca2ee400)
- [properly utilize generator env var](https://github.com/KhronosGroup/Vulkan-Headers/commit/b9390925eaaf231a69c78859b34e196904873084)
- [Match pname with vk.xml. (#2527)](https://github.com/KhronosGroup/Vulkan-Docs/commit/50f4a2392e2dd6c64565b55079c1a8f5b922c7c0)
- [Fix member names. (#2526)](https://github.com/KhronosGroup/Vulkan-Docs/commit/b56aa5eff855d7e738cde5786b32e587a7742a3f)
- [May 2, 2025 Vulkan 1.4.314 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/86c8fd72055a859f08fff20182ef758089dd180a)
- [Update for Vulkan-Docs 1.4.314](https://github.com/KhronosGroup/Vulkan-Headers/commit/9c77de5c3dd216f28e407eec65ed9c0a296c1f74)
- [Rework and expand docs site landing page (#2524)](https://github.com/KhronosGroup/Vulkan-Docs/commit/d37aee96ac460fb46c26227be8832eb7fd9fa0fb)
- [May 9, 2025 Vulkan 1.4.315 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/112aee75d162412a4623e7d22a3de52e0233cbf5)
- [Update for Vulkan-Docs 1.4.315](https://github.com/KhronosGroup/Vulkan-Headers/commit/75ad707a587e1469fb53a901b9b68fe9f6fbc11f)
- [Remove superfluous underscore (#2532)](https://github.com/KhronosGroup/Vulkan-Docs/commit/a2b893d8cd53b1814a5c50e7800964fbce4a2c66)
- [May 30, 2025 Vulkan 1.4.316 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/dd1a021e15026b17c6f844cb42a59036b9f9a9b8)
- [Update for Vulkan-Docs 1.4.316](https://github.com/KhronosGroup/Vulkan-Headers/commit/b11eecd68fb4b770f30fe2c9da522ff966f95b1e)
- [June 6, 2025 Vulkan 1.4.317 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/1228b2beff1a664da6dabea122f49f9818ecda8a)
- [Update for Vulkan-Docs 1.4.317](https://github.com/KhronosGroup/Vulkan-Headers/commit/2642d51e1e9230720a74d8c76bc7b301e69881bf)
- [Rework and expand docs site landing page (#2524)](https://github.com/KhronosGroup/Vulkan-Docs/commit/28261d476b791d4c5f1daeb158d9989650b87192)
- [Try to make git commands from CI actions work (#2542)](https://github.com/KhronosGroup/Vulkan-Docs/commit/57239bba44e2752e8c85f5d315959a187d8046ce)
- [June 13, 2025 Vulkan 1.4.318 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/38278ef4d7756e0257cede29473edee194b580ed)
- [Clarify WorkgroupId definition (#2541)](https://github.com/KhronosGroup/Vulkan-Docs/commit/08c63f528abc95718940335ee8ea74c0c127febf)
- [June 20, 2025 Vulkan 1.4.319 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/0dd79adbba6afe2b8e135aba53b0af9c87f616e1)
- [Update nav.adoc for recently added chapters (#2545)](https://github.com/KhronosGroup/Vulkan-Docs/commit/481dd347dae6cac4bf4ef8dcfae41789ab39f650)
- [June 27, 2025 Vulkan 1.4.320 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/1eef90d1d1e00c514393544164b0536ec4803cbb)
- [July 4, 2025 Vulkan 1.4.321 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/dad857284bf63d027ac9d6a97e9a1689831bab0d)
- [Systematic VUs for MemorySemantics (#2528)](https://github.com/KhronosGroup/Vulkan-Docs/commit/9e59769dd813769b0f95dc22f81957be84adfffb)
- [Chzange log for July 11, 2025 Vulkan 1.4.322 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/6ecb205ac29c6c07186c500460c1d9b837993ec2)

## [0.28.0] - 2025-03-06

### Changed
- Bumped MSRV to 1.81
- Reworked handling of bitfields in Vulkan structs to use custom structs for each 32-bit sequence of bitfields

### Bindings Updates
- [Fix GN include checks](https://github.com/KhronosGroup/Vulkan-Headers/commit/9dff1f571ce25b92639854b89b28539602b6b97b)
- [Update for Vulkan-Docs 1.4.303](https://github.com/KhronosGroup/Vulkan-Headers/commit/49af1bfe467dd5a9efc22f7867d95fdde50e2b00)
- [Add missing validusage.json and fix Makefile.release for new registry structure](https://github.com/KhronosGroup/Vulkan-Headers/commit/6a74a7d65cafa19e38ec116651436cce6efd5b2e)
- [Update for Vulkan-Docs 1.4.304](https://github.com/KhronosGroup/Vulkan-Headers/commit/d4a196d8c84e032d27f999adcea3075517c1c97f)
- [Remove Ash CI (#2479)](https://github.com/KhronosGroup/Vulkan-Docs/commit/921585827e7f60123c5a5eb1820ddabb984076c5)
- [January 17, 2025 Vulkan 1.4.305 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/840c7b817f7ff0e61a047a842c2569a1ecf5d807)
- [Update for Vulkan-Docs 1.4.305](https://github.com/KhronosGroup/Vulkan-Headers/commit/a03d2f6d5753b365d704d58161825890baad0755)
- [Add VK_EXT_external_memory_metal (#2414)](https://github.com/KhronosGroup/Vulkan-Docs/commit/25ec2b708ae742276fc3a28890f415b14fe0334c)
- [January 24, 2025 Vulkan 1.4.306 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/faf3ba598d6339c5590e544ffb3fdd182b46c544)
- [Remov rogue link (#2485)](https://github.com/KhronosGroup/Vulkan-Docs/commit/9ff93266ddde23a389004ae34459d629558677f2)
- [January 30, 2025 Vulkan 1.4.307 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/645c59c70e826d9738b6bb103316c03d887dfed3)
- [Fix small typo in proposal for VK_KHR_compute_shader_derivatives (#2493)](https://github.com/KhronosGroup/Vulkan-Docs/commit/0bfe685840b35fa727e108c8283ac8e24b185c09)
- [Remove superfluous semicolon in `VK_NV_optical_flow` (#2489)](https://github.com/KhronosGroup/Vulkan-Docs/commit/459e391899375ad9905cd27900146792d7e3fda3)
- [Replace fetches with fetch (#2494)](https://github.com/KhronosGroup/Vulkan-Docs/commit/a370995727451e75c44ed4af99c0d95c33ff906f)
- [February 7, 2025 Vulkan 1.4.308 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/ccac48a1126828018e570331999b9a7c2e17d742)
- [Fix promoted extension dependencies (#2492)](https://github.com/KhronosGroup/Vulkan-Docs/commit/80128cc302e96a191f452c15bd0d0a0c79312149)
- [Fix the CullPrimitiveEXT VUID's (#2475)](https://github.com/KhronosGroup/Vulkan-Docs/commit/54195f705c624cc58ada10b73e80ea3e1c126c37)
- [February 21, 2025 Vulkan 1.4.309 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/5a38e1f41285241a29f6428978aae6d1cf13fd16)

## [0.27.0] - 2025-02-23

### Changed
- Reworked builder for `vk::LayerSettingEXT` with different builder methods for different value types

### Added
- Added `fmt::Display` implementation for `vk::SuccessCode`

### Fixed
- Added missing descriptions (returned by `fmt::Display`) for `vk::Result`
- Fixed `fmt::Debug` implementation for `vk::SuccessCode` not always printing appropriate Vulkan result

## [0.26.0] - 2024-12-28

### Changed
- Bumped MSRV to 1.73

### Bindings Updates
- [Add discussion of processing frameworks being responsible for all defined attributes and tags (#2417)](https://github.com/KhronosGroup/Vulkan-Docs/commit/58d38a51ca9d1b4132af54c88b89481e7fe27dc9)
- [Reserve extension #604 (#2419)](https://github.com/KhronosGroup/Vulkan-Docs/commit/55255e28e68016d9861cb134dcc9db9255670a5d)
- [August 23, 2024 Vulkan 1.3.294 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/fb9f45c5ee8f4e8cd6f2b19eeb751fd305773a67)
- [Missing require for pipeline binary key size - #2422 (#2423)](https://github.com/KhronosGroup/Vulkan-Docs/commit/2eaf6b23921b1a597371d0646e3202a684fb973e)
- [minor wording change to avoid ambiguity in 'the act of waiting'. (#2418)](https://github.com/KhronosGroup/Vulkan-Docs/commit/ccd06598b7745aca51ea45c27055f5fe9573114d)
- [August 30, 2024 Vulkan 1.3.295 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/8e435deb15124bbf9a7e3340e6cc7975999ed5c5)
- [Rename Lina's GitHub username to @linyaa-kiwi (#2432)](https://github.com/KhronosGroup/Vulkan-Docs/commit/45c1f5dba1f3cbd56f3361147cd559ff804589b9)
- [build: Update build.gn with AV1 headers](https://github.com/KhronosGroup/Vulkan-Headers/commit/5ac36269f50381bdd92a5e1973d8eb041771e59e)
- [Update for Vulkan-Docs 1.3.278](https://github.com/KhronosGroup/Vulkan-Headers/commit/31aa7f634b052d87ede4664053e85f3f4d1d50d3)
- [Update for Vulkan-Docs 1.3.279](https://github.com/KhronosGroup/Vulkan-Headers/commit/46dc0f6e514f5730784bb2cac2a7c731636839e8)
- [build(deps): bump fsfe/reuse-action from 2 to 3](https://github.com/KhronosGroup/Vulkan-Headers/commit/f030d9dd8ca297b84b10330cee87f72acc3b0aab)
- [Update for Vulkan-Docs 1.3.280](https://github.com/KhronosGroup/Vulkan-Headers/commit/577baa05033cf1d9236b3d078ca4b3269ed87a2b)
- [Update for Vulkan-Docs 1.3.281](https://github.com/KhronosGroup/Vulkan-Headers/commit/cfebfc96b2b0bce93da7d12f2c14cc01793ae25c)
- [Update for Vulkan-Docs 1.3.282](https://github.com/KhronosGroup/Vulkan-Headers/commit/1e7b8a6d03d30c9254b5f533b561d62bba8c3199)
- [Update for Vulkan-Docs 1.3.283](https://github.com/KhronosGroup/Vulkan-Headers/commit/eaa319dade959cb61ed2229c8ea42e307cc8f8b3)
- [Update for Vulkan-Docs 1.3.284](https://github.com/KhronosGroup/Vulkan-Headers/commit/4bc77c26ff9ce89cf4a4f79e1c24a44604132d53)
- [Update for Vulkan-Docs 1.3.285](https://github.com/KhronosGroup/Vulkan-Headers/commit/5677bafb820e476441e9e1f745371b72133407d3)
- [Update for Vulkan-Docs 1.3.286](https://github.com/KhronosGroup/Vulkan-Headers/commit/192d051db3382e213f8bd9d8048fc9eaa78ed6ab)
- [Update for Vulkan-Docs 1.3.287](https://github.com/KhronosGroup/Vulkan-Headers/commit/d192041a2fc9c9fd8ae67d8ae3f32c5511541f04)
- [Update for Vulkan-Docs 1.3.288](https://github.com/KhronosGroup/Vulkan-Headers/commit/05fe2cc910a68c9ba5dac07db46ef78573acee72)
- [cmake: Allow external control of whether to test or install](https://github.com/KhronosGroup/Vulkan-Headers/commit/8f034f6b48fd2d30f711396a021e1dc050c8941c)
- [cmake: Workaround MSVC module support compiler bug](https://github.com/KhronosGroup/Vulkan-Headers/commit/e3c37e6e184a232e10b01dff5a065ce48c047f88)
- [ci: Add explicit build step](https://github.com/KhronosGroup/Vulkan-Headers/commit/cde27c9456aed7117385e7469c4fc5ea3e1baa7f)
- [cmake: Remove explicit install of Vulkan-Module](https://github.com/KhronosGroup/Vulkan-Headers/commit/7dacb97fbace2f102d65aa3a49bff3cfe316b515)
- [ci: Add install step](https://github.com/KhronosGroup/Vulkan-Headers/commit/a692a9c939e1cafbf30ecdfb561a4ff179504fa6)
- [Add `vulkan.cppm` as a separate library: `Vulkan::VulkanHppModule`](https://github.com/KhronosGroup/Vulkan-Headers/commit/85256c712ce9ee9755ae74faacc3e6a1d2c5eddf)
- [Update for Vulkan-Docs 1.3.289](https://github.com/KhronosGroup/Vulkan-Headers/commit/190d2cb24e90e5bf2bec0a75604a9b3586485b6d)
- [Change name of update to Makefile.release](https://github.com/KhronosGroup/Vulkan-Headers/commit/4b9ea26d48f23e260c57bece311660d9e5c7ff23)
- [Makefile to import generated scripts and headers when doing a spec update](https://github.com/KhronosGroup/Vulkan-Headers/commit/67dcf5647c697a3a21fc6fa4976b1ec57338fb51)
- [build(deps): bump fsfe/reuse-action from 3 to 4](https://github.com/KhronosGroup/Vulkan-Headers/commit/6c539b2ed2dba2997cdedeac0b376ff2fe382595)
- [build: Require MSVC 17.11 for module support](https://github.com/KhronosGroup/Vulkan-Headers/commit/78a92e2c7c5aa1c7b7e6f9c475bd9009977ea8f9)
- [Update for Vulkan-Docs 1.3.290](https://github.com/KhronosGroup/Vulkan-Headers/commit/f41928bd4ac3b0451b68898d8e58a6ed5ee99f2b)
- [Disable VulkanHppModule support in clang-cl](https://github.com/KhronosGroup/Vulkan-Headers/commit/fc6c06ac529e4b4b6e34c17cc650a8f62dee2eb0)
- [Add windows clang & clang-cl CI jobs](https://github.com/KhronosGroup/Vulkan-Headers/commit/59527917988fe6cce9e35d75e00b4cc030167e7c)
- [cmake: Disable modules if clang-scan-deps is missing](https://github.com/KhronosGroup/Vulkan-Headers/commit/b379292b2ab6df5771ba9870d53cf8b2c9295daf)
- [Update for Vulkan-Docs 1.3.291](https://github.com/KhronosGroup/Vulkan-Headers/commit/fabe9e2672334fdb9a622d42a2e8f94578952082)
- [Update for Vulkan-Docs 1.3.292](https://github.com/KhronosGroup/Vulkan-Headers/commit/595c8d4794410a4e64b98dc58d27c0310d7ea2fd)
- [Update for Vulkan-Docs 1.3.293](https://github.com/KhronosGroup/Vulkan-Headers/commit/d205aff40b4e15d4c568523ee6a26f85138126d9)
- [Update for Vulkan-Docs 1.3.294](https://github.com/KhronosGroup/Vulkan-Headers/commit/a6a5dc0d078ade9bde75bd78404462509cbdce99)
- [Update for Vulkan-Docs 1.3.295](https://github.com/KhronosGroup/Vulkan-Headers/commit/fbda05468ec5956bc4f163b50c18b8df9312cdee)
- [Fix MSVC identification for modules support](https://github.com/KhronosGroup/Vulkan-Headers/commit/c6391a7b8cd57e79ce6b6c832c8e3043c4d9967b)
- [Support the `stride` attribute for array pointers in both command (#2437)](https://github.com/KhronosGroup/Vulkan-Docs/commit/eab3ac3bc5a2f9e0cf7096dcee742fea791df88a)
- [September 26, 2024 Vulkan 1.3.296 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/486e4b289053a7d64784e7ce791711843c60c235)
- [Update for Vulkan-Docs 1.3.296](https://github.com/KhronosGroup/Vulkan-Headers/commit/29f979ee5aa58b7b005f805ea8df7a855c39ff37)
- [Fix references to images in proposal documents  (#2441)](https://github.com/KhronosGroup/Vulkan-Docs/commit/817d35d46b04324b196b02b64359724d04b6b880)
- [Fix misuse of spec macro in a proposal document](https://github.com/KhronosGroup/Vulkan-Docs/commit/d78959ab68122f270b95447317ebe22ae1f19a78)
- [Fix missing attribute in another proposal document](https://github.com/KhronosGroup/Vulkan-Docs/commit/bfd68417298c69ca3608ae4756206c9e6a9edd11)
- [October 4, 2024 Vulkan 1.3.297 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/74d7efc182c8e91002221c68f321c05fd596cc70)
- [Update for Vulkan-Docs 1.3.297](https://github.com/KhronosGroup/Vulkan-Headers/commit/14345dab231912ee9601136e96ca67a6e1f632e7)
- [Update VK_AMDX_shader_enqueue to v2 (#2442)](https://github.com/KhronosGroup/Vulkan-Docs/commit/03f2654057e1d97a5fa824f4bdd5e235a5829b9c)
- [Add location order to the definition of from-reads (#2402)](https://github.com/KhronosGroup/Vulkan-Docs/commit/2ff3b679e9ec24498218c11d01b769bc8bb97496)
- [October 11, 2024 Vulkan 1.3.298 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/05d5444cce55e340e07ead87d552407da5ea8771)
- [Update for Vulkan-Docs 1.3.298](https://github.com/KhronosGroup/Vulkan-Headers/commit/d91597a82f881d473887b560a03a7edf2720b72c)
- [October 18, 2024 Vulkan 1.3.299 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/46e04bb59e0f85b9124899e694a477dd9025d17c)
- [Update for Vulkan-Docs 1.3.299](https://github.com/KhronosGroup/Vulkan-Headers/commit/b955ae0edb4f02074bfbf134ccc1980e83122d30)
- [added SHADY author id (#2448)](https://github.com/KhronosGroup/Vulkan-Docs/commit/e3a413637f200c872461c14ff397af01043a3958)
- [ci: Only run once if pushing to main repo branch](https://github.com/KhronosGroup/Vulkan-Headers/commit/e271cfd4809ed133cadc6c3de7903e59628b3d8a)
- [Migrate Antora makefile targets into top-level Makefile (#2450)](https://github.com/KhronosGroup/Vulkan-Docs/commit/f3a114c4841d065e64fbc1d10fbe7f768867b96d)
- [October 25, 2024 Vulkan 1.3.300 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/c9304f8dd1aa20183942c6a3638b28c81b4e681d)
- [Update for Vulkan-Docs 1.3.300](https://github.com/KhronosGroup/Vulkan-Headers/commit/ab1ea9059d75b42a5717c7ab55713bdf194ccf21)
- [Do not remove nav.adoc target in clean rules](https://github.com/KhronosGroup/Vulkan-Docs/commit/30b651ee3ba1d19d23056be8e00549cd29ed42b0)
- [build: Disable Vulkan-Module by default](https://github.com/KhronosGroup/Vulkan-Headers/commit/f2eb740f3f0340ad941e563484d5ba45a5ec6af0)
- [Add docs.vulkan.org page information to validusage.json (#2453)](https://github.com/KhronosGroup/Vulkan-Docs/commit/69bbb65bc0ab16ee87d631ea6f25f6687facf3df)
- [November 1, 2024 Vulkan 1.3.301 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/d095b206f80dc85140898a7979b2c3bb81aeb7f9)
- [Fix build for extremely long list of files passed to antora-prep.py which exceeded system limits](https://github.com/KhronosGroup/Vulkan-Docs/commit/c040dded44296a29676d327c68225ca139434ebc)
- [Update for Vulkan-Docs 1.3.301](https://github.com/KhronosGroup/Vulkan-Headers/commit/cbcad3c0587dddc768d76641ea00f5c45ab5a278)
- [Antora build fixes](https://github.com/KhronosGroup/Vulkan-Docs/commit/921e62413d3d6b5df662dc06e193eb9fa3820821)
- [Fix some issues with VK_EXT_device_generated_commands proposal (#2459)](https://github.com/KhronosGroup/Vulkan-Docs/commit/a61256a946d2b48b9efdd3a18c3fd4c4c354b949)
- [build(deps): bump fsfe/reuse-action from 4 to 5](https://github.com/KhronosGroup/Vulkan-Headers/commit/f864bc6dfe6229a399566e979c16795386d0f308)
- [November 21, 2024 Vulkan 1.3.302 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/310c86fb5a06544a84bce70867f7c038b748e51c)
- [Update for Vulkan-Docs 1.3.302](https://github.com/KhronosGroup/Vulkan-Headers/commit/36872f9062b17b1a30b8ed1d81ca5ea6bb608a72)
- [Fix markup](https://github.com/KhronosGroup/Vulkan-Docs/commit/8d9f35a010e272ceccb7cccc3e94dc6ff866f8d1)
- [Fix typo that triggered CI failure](https://github.com/KhronosGroup/Vulkan-Docs/commit/4c2df0a935e039ee231af8e0f09d0f69910151da)
- [registry.adoc: Backticks in quotes?](https://github.com/KhronosGroup/Vulkan-Docs/commit/8b9e199d72c78a2d7e9bb6650d9de02bf01bf78e)
- [Add author ID FREDEMMOTT (#2467)](https://github.com/KhronosGroup/Vulkan-Docs/commit/6c1f255baecde79af3bc526fcd46a36af784bc1f)
- [December 3, 2024 Vulkan 1.4.303 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/c7a3955e47d223c6a37fb29e2061c973eec98d0a)
- [Add missing custom attribute](https://github.com/KhronosGroup/Vulkan-Docs/commit/485e1c9ff6c8e78ac0a92f85318293db3fdbbcb6)
- [Update nav.adoc](https://github.com/KhronosGroup/Vulkan-Docs/commit/a6c9d5c3a2a37525d13019508667b42e67695b36)
- [December 20, 2024 Vulkan 1.4.304 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/b04fc6a9c5893fe0f8345844a1d62ebed94d09d9)

## [0.25.0] - 2024-08-22

### Changed
- Bumped MSRV to 1.70

### Added
- Added `ResultExt` trait
- Added `Instance::version` method

### Fixed
- Fixed `no_std_error` feature not compiling
- Instance-level functions that are part of device extensions are loaded with `vkGetInstanceProcAddr` ([#281](https://github.com/KyleMayes/vulkanalia/issues/281))

### Bindings Updates
- [July 26, 2024 Vulkan 1.3.292 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/e090b1020fb9636b752e73adfc82a3c595fb6615)
- [Merge equivalent require blocks (#2406)](https://github.com/KhronosGroup/Vulkan-Docs/commit/8f60e57622707a0555cf79a83275c3c5e6dfa146)
- [August 16, 2024 Vulkan 1.3.293 spec update](https://github.com/KhronosGroup/Vulkan-Docs/commit/59dc3a34d2c3a6704cc569a82a2798fae337367c)
- [Reserve extension 603 (#2416)](https://github.com/KhronosGroup/Vulkan-Docs/commit/306edf6a920ae372d5134424fcc9c009c31e0760)

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
