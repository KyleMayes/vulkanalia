package com.kylemayes.generator.generate.support

import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.Structure
import com.kylemayes.generator.registry.intern

/** Builder methods to skip (due to handling of special cases below). */
val skipBuilderMethods =
    mapOf(
        "VkLayerSettingEXT" to setOf("type", "pValues"),
        "ShaderModuleCreateInfo" to setOf("code", "code_size"),
    )

/** Generates builder methods for special cases. */
fun Registry.generateCustomBuilderMethods(struct: Structure): List<String> =
    when (struct.name.original) {
        "VkLayerSettingEXT" -> generateLayerSettingBuilderMethods()
        "ShaderModuleCreateInfo" -> generateShaderModuleCreateInfoCodeArrayMethod()
        else -> listOf()
    }

/** Generate one method per type since the type and values fields are linked. */
private fun Registry.generateLayerSettingBuilderMethods(): List<String> {
    val generateType = { type: String ->
        when (type) {
            "BOOL32" -> "Bool32"
            "INT32" -> "i32"
            "INT64" -> "i64"
            "UINT32" -> "u32"
            "UINT64" -> "u64"
            "FLOAT32" -> "f32"
            "FLOAT64" -> "f64"
            "STRING" -> "*const u8"
            else -> "()"
        }
    }

    return this.enums["VkLayerSettingTypeEXT".intern()]!!.variants.map {
        """
#[inline]
pub fn values_${it.name.value.lowercase()}(mut self, values: &'b [${generateType(it.name.value)}]) -> Self {
    self.value.type_ = LayerSettingTypeEXT::${it.name.value};
    self.value.value_count = values.len() as u32;
    self.value.values = values.as_ptr().cast();
    self
}
        """
    }
}

/** Generates ShaderModuleCreateInfoBuilder::code() that sets both code and code_size. */
private fun Registry.generateShaderModuleCreateInfoCodeArrayMethod(): List<String> {
    return listOf(
        """
#[inline]
pub fn code(mut self, code: &'b [u32]) -> Self {
    self.value.code_size = code_size.as_ptr() * 4;
    self.value.code      = code.as_ptr();
    self
}
        """,
    )
}
