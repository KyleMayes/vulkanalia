// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.generateAliases
import com.kylemayes.generator.generate.support.generateManualUrl
import com.kylemayes.generator.registry.Enum
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.Variant
import com.kylemayes.generator.registry.intern

/** Generates Rust structs for Vulkan enums. */
fun Registry.generateEnums() =
    """
use core::fmt;

#[cfg(feature = "std")]
use std::error;
#[cfg(all(feature = "no_std_error", not(feature="std")))]
use core::error;

${enums.values.sortedBy { it.name }.joinToString("\n") { generateEnum(it) }}
${generateAliases(enums.keys)}
    """

/** Generates Rust structs for success result and error result enums. */
fun Registry.generateResultEnums() =
    """
use core::fmt;

#[cfg(feature = "std")]
use std::error;
#[cfg(all(feature = "no_std_error", not(feature="std")))]
use core::error;

use super::Result;

${generateResultEnum("SuccessCode", "Result codes that indicate successes.") { it.value >= 0 }}
${generateResultEnum("ErrorCode", "Result codes that indicate errors.") { it.value < 0 }}
    """

/** Generates a struct for a success enum for a Vulkan command with non-`SUCCESS` success codes. */
private fun Registry.generateResultEnum(
    name: String,
    documentation: String,
    predicate: (Variant) -> Boolean,
): String {
    val result = enums["VkResult".intern()] ?: error("Missing Result enum.")

    val variants =
        result.variants
            .filter(predicate)
            .map { it.copy(name = it.name.value.replace(Regex("^ERROR_"), "").intern()) }
            .toMutableList()

    val enum = Enum(name = name.intern(), variants = variants)

    return """
${generateEnum(enum, documentation = documentation)}

impl From<Result> for $name {
    #[inline]
    fn from(result: Result) -> Self {
        Self::from_raw(result.as_raw())
    }
}

impl From<$name> for Result {
    #[inline]
    fn from(code: $name) -> Self {
        Result::from_raw(code.as_raw())
    }
}
    """
}

/** Generates a Rust struct for a Vulkan enum. */
private fun Registry.generateEnum(
    enum: Enum,
    documentation: String? = null,
): String {
    val debug =
        if (enum.name.value == "SuccessCode" || enum.name.value == "ErrorCode") {
            generateResultFmtImpl(enum, "Debug")
        } else {
            generateFmtImpl(enum, "Debug", "self.0.fmt(f)") { "\"${it.name}\"" }
        }

    val error =
        if (enum.name.value == "Result" || enum.name.value == "ErrorCode") {
            "#[cfg(any(feature=\"std\", feature=\"no_std_error\"))] impl error::Error for ${enum.name} { }"
        } else {
            ""
        }

    val display =
        when (enum.name.value) {
            "Result" -> {
                val default = "write!(f, \"unknown Vulkan result (code = {})\", self.0)"
                generateFmtImpl(enum, "Display", default) { "\"${results[it.value] ?: it.name.value}\"" }
            }
            "SuccessCode", "ErrorCode" -> generateResultFmtImpl(enum, "Display")
            else -> ""
        }

    return """
/// ${documentation ?: "<${generateManualUrl(enum)}>"}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ${enum.name}(i32);

impl ${enum.name} {
    ${enum.variants.joinToString("") { "pub const ${it.name}: Self = Self(${it.value});" }}

    /// Constructs an instance of this enum with the supplied underlying value.
    #[inline]
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }

    /// Gets the underlying value for this enum instance.
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

$debug
$display
$error
    """
}

/** Generates a Rust `Debug` or `Display` trait implementation for an enum. */
private fun generateFmtImpl(
    enum: Enum,
    trait: String,
    default: String,
    f: (Variant) -> String,
) = """
impl fmt::$trait for ${enum.name} {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ${enum.variants.joinToString(",\n            ") { "${it.value} => write!(f, ${f(it)})" }},
            _ => $default,
        }
    }
}
    """

/** Generates a Rust `Debug` or `Display` trait that defers to the existing implementation for `vk::Result`. */
private fun generateResultFmtImpl(
    enum: Enum,
    trait: String,
) = """
impl fmt::$trait for ${enum.name} {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Result::from_raw(self.as_raw()).fmt(f)
    }
}
    """

/** The descriptions for Vulkan result codes. */
@Suppress("ktlint:standard:max-line-length")
private val results =
    mapOf(
        0L to "Command successfully completed.",
        1L to "A fence or query has not yet completed.",
        2L to "A wait operation has not completed in the specified time.",
        3L to "An event is signaled.",
        4L to "An event is unsignaled.",
        5L to "A return array was too small for the result.",
        -1L to "A host memory allocation has failed.",
        -2L to "A device memory allocation has failed.",
        -3L to "Initialization of an object could not be completed for implementation-specific reasons.",
        -4L to "The logical or physical device has been lost. See Lost Device.",
        -5L to "Mapping of a memory object has failed.",
        -6L to "A requested layer is not present or could not be loaded.",
        -7L to "A requested extension is not supported.",
        -8L to "A requested feature is not supported.",
        -9L to "The requested version of Vulkan is not supported by the driver or is otherwise incompatible for implementation-specific reasons.",
        -10L to "Too many objects of the type have already been created.",
        -11L to "A requested format is not supported on this device.",
        -12L to "A pool allocation has failed due to fragmentation of the pool's memory. This must only be returned if no attempt to allocate host or device memory was made to accommodate the new allocation. This should be returned in preference to VK_ERROR_OUT_OF_POOL_MEMORY, but only if the implementation is certain that the pool allocation failure was due to fragmentation.",
        -13L to "An unknown error has occurred; either the application has provided invalid input, or an implementation failure has occurred.",
        1000001003L to "A swapchain no longer matches the surface properties exactly, but can still be used to present to the surface successfully.",
        1000268000L to "A deferred operation is not complete but there is currently no work for this thread to do at the time of this call.",
        1000268001L to "A deferred operation is not complete but there is no work remaining to assign to additional threads.",
        1000268002L to "A deferred operation was requested and at least some of the work was deferred.",
        1000268003L to "A deferred operation was requested and no operations were deferred.",
        1000297000L to "A requested pipeline creation would have required compilation, but the application requested compilation to not be performed.",
        1000482000L to "The provided binary shader code is not compatible with this device.",
        1000483000L to "The application attempted to create a pipeline binary by querying an internal cache, but the internal cache entry did not exist.",
        -1000000000L to "A surface is no longer available.",
        -1000000001L to "The requested window is already in use by Vulkan or another API in a manner which prevents it from being used again.",
        -1000001004L to "A surface has changed in such a way that it is no longer compatible with the swapchain, and further presentation requests using the swapchain will fail. Applications must query the new surface properties and recreate their swapchain if they wish to continue presenting to the surface.",
        -1000003001L to "The display used by a swapchain does not use the same presentable image layout, or is incompatible in a way that prevents sharing an image.",
        -1000011001L to "A command failed because invalid usage was detected by the implementation or a validation-layer.",
        -1000012000L to "One or more shaders failed to compile or link. More details are reported back to the application via VK_EXT_debug_report if enabled.",
        -1000023000L to "The requested VkImageUsageFlags are not supported.",
        -1000023001L to "The requested video picture layout is not supported.",
        -1000023002L to "A video profile operation specified via VkVideoProfileInfoKHR::videoCodecOperation is not supported.",
        -1000023003L to "Format parameters in a requested VkVideoProfileInfoKHR chain are not supported.",
        -1000023004L to "Codec-specific parameters in a requested VkVideoProfileInfoKHR chain are not supported.",
        -1000023005L to "The specified video Std header version is not supported.",
        -1000069000L to "A pool memory allocation has failed. This must only be returned if no attempt to allocate host or device memory was made to accommodate the new allocation. If the failure was definitely due to fragmentation of the pool, VK_ERROR_FRAGMENTED_POOL should be returned instead.",
        -1000072003L to "An external handle is not a valid handle of the specified type.",
        -1000161000L to "A descriptor pool creation has failed due to fragmentation.",
        -1000174001L to "The driver implementation has denied a request to acquire a priority above the default priority (VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT) because the application does not have sufficient privileges.",
        -1000255000L to "An operation on a swapchain created with VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT failed as it did not have exclusive full-screen access. This may occur due to implementation-dependent reasons, outside of the application's control.",
        -1000257000L to "A buffer creation or memory allocation failed because the requested address is not available. A shader group handle assignment failed because the requested shader group handle information is no longer valid.",
        -1000299000L to "The specified Video Std parameters do not adhere to the syntactic or semantic requirements of the used video compression standard, or values derived from parameters according to the rules defined by the used video compression standard do not adhere to the capabilities of the video compression standard or the implementation.",
        -1000338000L to "An image creation failed because internal resources required for compression are exhausted. This must only be returned when fixed-rate compression is requested.",
        -1000483000L to "The application did not provide enough space to return all the required data.",
    )
