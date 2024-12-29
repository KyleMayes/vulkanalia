fn main() {
    build();

    #[cfg(feature = "bind")]
    bind();
}

fn build() {
    let mut build = cc::Build::new();

    // Disable VMA assertions when Rust assertions are disabled.
    #[cfg(not(debug_assertions))]
    build.define("NDEBUG", "");

    // Set up VMA to only use Vulkan functions explicitly provided to it.
    build.define("VMA_DYNAMIC_VULKAN_FUNCTIONS", "0");
    build.define("VMA_STATIC_VULKAN_FUNCTIONS", "0");

    // Input.
    build.include("vendor/VulkanMemoryAllocator/include");
    build.include("vendor/Vulkan-Headers/include");
    build.file("wrapper.cpp");

    let target = std::env::var("TARGET").unwrap();

    if !target.contains("msvc") {
        build
            .flag("-std=c++17")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-nullability-completeness")
            .flag("-Wno-reorder")
            .flag("-Wno-type-limits")
            .flag("-Wno-unused-function")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-unused-variable");
    }

    if target.contains("darwin") || target.contains("ios") {
        build.cpp_set_stdlib("c++").cpp(true);
    } else if target.contains("android") {
        build.cpp_link_stdlib("c++").cpp(true);
    } else if target.contains("linux") || (target.contains("windows") && target.contains("gnu")) {
        build.cpp_link_stdlib("stdc++").cpp(true);
    }

    build.compile("vma");
}

#[cfg(feature = "bind")]
static DEBUG: &[&str] = &[
    "VmaBudget",
    "VmaDetailedStatistics",
    "VmaStatistics",
    "VmaTotalStatistics",
];

#[cfg(feature = "bind")]
fn bind() {
    #[derive(Debug)]
    struct ParseCallbacks;

    impl bindgen::callbacks::ParseCallbacks for ParseCallbacks {
        // Add missing/additional derives.
        fn add_derives(&self, info: &bindgen::callbacks::DeriveInfo) -> Vec<String> {
            let mut derives = vec![];

            if DEBUG.contains(&info.name) {
                derives.push("Debug".into());
            }

            derives
        }

        // Remove redundant enum variant prefixes.
        //
        // Before: `VmaMemoryUsage::VMA_MEMORY_USAGE_GPU_ONLY`
        //  After: `VmaMemoryUsage::GPU_ONLY`
        fn enum_variant_name(
            &self,
            enum_name: Option<&str>,
            variant_name: &str,
            _: bindgen::callbacks::EnumVariantValue,
        ) -> Option<String> {
            let enum_name = enum_name?.trim_start_matches("enum ").to_uppercase();

            let mut index = 0;
            let mut enum_bytes = enum_name.as_bytes().iter();
            for variant_byte in variant_name.as_bytes() {
                if *variant_byte == b'_' || enum_bytes.next() == Some(variant_byte) {
                    index += 1;
                } else {
                    break;
                }
            }

            let bytes = &variant_name.as_bytes()[index..];
            Some(std::str::from_utf8(bytes).unwrap().into())
        }

        // Replace Vulkan types with imported vulkanalia types.
        //
        // Before: `VkDevice`
        //  After: `Device`
        fn item_name(&self, item_name: &str) -> Option<String> {
            if item_name.starts_with("Vk") {
                Some(item_name.trim_start_matches("Vk").into())
            } else {
                None
            }
        }

        // Convert Doxygen comments to Rustdoc comments.
        fn process_comment(&self, comment: &str) -> Option<String> {
            Some(doxygen_rs::transform(comment))
        }
    }

    let major = 73;
    let minor = 0;
    let rust_target = if let Ok(rust_target) = bindgen::RustTarget::stable(major, minor) {
        rust_target
    } else {
        panic!("invalid Rust version: {major}.{minor}");
    };

    let bindings = bindgen::builder()
        .rust_target(rust_target)
        // Input.
        .clang_arg("-I./vendor/Vulkan-Headers/include")
        .clang_arg("-I./wrapper")
        .header("vendor/VulkanMemoryAllocator/include/vk_mem_alloc.h")
        // Options.
        .allowlist_function("PFN_vma.*")
        .allowlist_function("vma.*")
        .allowlist_type("Vma.*")
        .blocklist_type("__darwin_.*")
        .blocklist_type("PFN_vk.*")
        .blocklist_type("Vk.*")
        .layout_tests(false)
        .parse_callbacks(Box::new(ParseCallbacks))
        .rustified_enum("Vma.*")
        .use_core()
        // Output.
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(rustdoc::broken_intra_doc_links)]")
        .raw_line("use vulkanalia::vk::*;")
        .formatter(bindgen::Formatter::Rustfmt)
        .generate()
        .expect("unable to generate VMA bindings");

    bindings
        .write_to_file("src/vma.rs")
        .expect("unable to write VMA bindings");
}
