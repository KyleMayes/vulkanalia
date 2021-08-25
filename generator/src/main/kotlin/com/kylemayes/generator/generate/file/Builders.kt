// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.file

import com.kylemayes.generator.generate.support.generatePtr
import com.kylemayes.generator.generate.support.generateRef
import com.kylemayes.generator.generate.support.getStructExtensions
import com.kylemayes.generator.generate.support.getStructLifetime
import com.kylemayes.generator.generate.support.getUnsupportedExtensionEntities
import com.kylemayes.generator.registry.Identifier
import com.kylemayes.generator.registry.Member
import com.kylemayes.generator.registry.PointerType
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.Structure
import com.kylemayes.generator.registry.getIdentifier
import com.kylemayes.generator.registry.getPointee
import com.kylemayes.generator.registry.isByteArray
import com.kylemayes.generator.registry.isOpaquePointer
import com.kylemayes.generator.registry.isStringArray
import com.kylemayes.generator.registry.isStringPointer

/** Generates Rust structs to build Vulkan structs. */
fun Registry.generateBuilders() =
    """
use std::fmt;
use std::marker::PhantomData;
use std::ops;
use std::os::raw::{c_char, c_int};

use super::*;

/// A type that can be used interchangeably with another in FFI.
pub unsafe trait Cast {
    /// The other type this type type can be used interchangeably with in FFI.
    type Target;

    /// Converts this value into a value of the other type.
    fn into(self) -> Self::Target;

    /// Converts this reference into a reference to the other type.
    #[inline]
    fn as_ref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }

    /// Converts this mutable reference into a mutable reference to the other type.
    #[inline]
    fn as_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}

/// A Vulkan type that has an associated builder.
pub trait HasBuilder<'b> {
    /// The associated builder for this type.
    type Builder: Copy
        + Clone
        + fmt::Debug
        + Default
        + ops::Deref<Target=Self>
        + ops::DerefMut<Target=Self>
        + 'b;

    /// Constructs an instance of the associated builder for this type.
    #[inline]
    fn builder() -> Self::Builder {
        Default::default()
    }
}

${structs.values
        .filter { !getUnsupportedExtensionEntities().contains(it.name) }
        .sortedBy { it.name }
        .joinToString("") { generateBuilder(it) }}
    """

/** Generates a Rust struct to build a Vulkan struct. */
fun Registry.generateBuilder(struct: Structure): String {
    val lifetime = if (getStructLifetime(struct)) { "<'b>" } else { "" }
    val traitLifetime = if (lifetime.isNotEmpty()) { "<'b>" } else { "<'static>" }
    val marker = if (lifetime.isNotEmpty()) { "_marker: PhantomData<&'b ()>," } else { "" }
    val methods = generateMethods(struct)
    return """
${generateExtends(struct)}

unsafe impl Cast for ${struct.name} {
    type Target = ${struct.name};

    #[inline]
    fn into(self) -> Self::Target {
        self
    }
}

impl$lifetime HasBuilder$traitLifetime for ${struct.name} {
    type Builder = ${struct.name}Builder$lifetime;
}

/// A builder for a [`${struct.name}`].
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ${struct.name}Builder$lifetime {
    value: ${struct.name},$marker
}

impl$lifetime ${struct.name}Builder$lifetime {
    $methods

    #[inline]
    pub fn build(self) -> ${struct.name} {
        self.value
    }
}

impl$lifetime ops::Deref for ${struct.name}Builder$lifetime {
    type Target = ${struct.name};

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl$lifetime ops::DerefMut for ${struct.name}Builder$lifetime {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

unsafe impl$lifetime Cast for ${struct.name}Builder$lifetime {
    type Target = ${struct.name};

    #[inline]
    fn into(self) -> Self::Target {
        self.value
    }
}
    """
}

/** Generates a Rust trait and implementations for the Vulkan structs that can extend another Vulkan struct. */
private fun Registry.generateExtends(struct: Structure): String {
    val extends = (getStructExtensions()[struct.name] ?: emptyList())
        .filter { !getUnsupportedExtensionEntities().contains(it) }
        .sorted()
    if (extends.isEmpty()) {
        return ""
    }

    return """
/// A Vulkan struct that can be used to extend a [`${struct.name}`].
pub unsafe trait Extends${struct.name} { }
${extends.joinToString("\n") { "unsafe impl Extends${struct.name} for $it { }" }}
    """
}

/** Generates Rust builder methods for a Vulkan struct. */
private fun Registry.generateMethods(struct: Structure): String {
    val members = struct.members.associateBy { it.name }
    val arraysByLength = mutableMapOf<Identifier, Member>()
    for (member in members.values) {
        val length = members[member.len?.get(0)]
        if (length != null) {
            arraysByLength[length.name] = member
        }
    }

    // Filter out the fields that do not require builder methods since they will
    // be set by other builder methods (i.e., array length fields for
    // non-optional array fields).
    val requireBuilders = members.values.filter {
        arraysByLength[it.name].let { a -> a?.optional ?: true }
    }

    // Generate the builder methods.
    val methods = ArrayList<String>()
    for (member in requireBuilders) {
        val len = member.len?.get(0)
        if (len != null && len.value != "null-terminated") {
            if (arraysByLength.containsKey(len)) {
                val lengthMember = members[len] ?: error("Missing length member.")
                val length = Pair(lengthMember.name.value, lengthMember.type.generate())
                methods.add(generateArrayMethod(member, length))
            } else {
                // Some array fields have either static pre-determined lengths
                // (e.g., `2*VK_UUID_SIZE`) or lengths expressed as a
                // mathematical expression involving other fields (e.g.,
                // `codeSize / 4`). For simplicity's sake we will put the burden
                // of enforcing the correctness of these lengths on the user.
                methods.add(generateArrayMethod(member))
            }
        } else if (member.name.value != "s_type" || member.values == null) {
            if (member.name.value == "next") {
                if (getStructExtensions()[struct.name]?.isNotEmpty() == true) {
                    methods.add(generateNextMethod(struct))
                }
            } else {
                methods.add(generateOtherMethod(member))
            }
        }
    }

    return methods.joinToString("")
}

/** Generates a Rust builder method for an array value. */
private fun Registry.generateArrayMethod(member: Member, length: Pair<String, String>? = null): String {
    val pointer = member.type as PointerType
    val identifier = pointer.pointee.getIdentifier()

    val (item, cast) = when {
        structs.containsKey(identifier) -> "impl Cast<Target = ${pointer.pointee.generate()}>" to ".cast()"
        identifier?.value == "void" -> "u8" to ".cast()"
        else -> pointer.pointee.generate() to ""
    }

    val type = "[$item]".generateRef(pointer.const, lifetime = "b")
    val method = if (pointer.const) { "as_ptr" } else { "as_mut_ptr" }

    return """
#[inline]
pub fn ${member.name}(mut self, ${member.name}: $type) -> Self {
    ${if (length != null) { "self.value.${length.first} = ${member.name}.len() as ${length.second};" } else { "" }}
    self.value.${member.name} = ${member.name}.$method()$cast;
    self
}
    """
}

/** Generates a Rust builder method which allows extending the Vulkan struct. */
private fun Registry.generateNextMethod(struct: Structure): String {
    val extensions = getStructExtensions()[struct.name] ?: emptyList()
    if (extensions.isEmpty()) {
        return ""
    }

    return """
#[inline]
pub fn push_next<T>(mut self, next: &'b mut impl Cast<Target = T>) -> Self
where
    T: Extends${struct.name}
{
    let next = (next.as_mut() as *mut T).cast::<${struct.name}>();
    unsafe { *next }.next = self.next;
    self.next = next.cast();
    self
}
    """
}

/** Generates a Rust builder method for a non-array value. */
private fun Registry.generateOtherMethod(member: Member): String {
    // Void pointer.
    if (member.type.getPointee()?.getIdentifier()?.value == "void") {
        val pointer = member.type as PointerType
        val ref = "T".generateRef(pointer.const, lifetime = "b")
        val ptr = "T".generatePtr(pointer.const)
        return """
#[inline]
pub fn ${member.name}<T>(mut self, ${member.name}: $ref) -> Self {
    self.value.${member.name} = (${member.name} as $ptr).cast();
    self
}
        """
    }

    val (type, cast) = when {
        // Boolean.
        member.type.getIdentifier()?.value == "Bool32" -> Pair("bool", { m: String -> "$m as Bool32" })
        // Array (byte).
        member.type.isByteArray() -> Pair("impl Into<${member.type.generate()}>", { m: String -> "$m.into()" })
        // Array (byte).
        member.type.isStringArray() -> Pair("impl Into<${member.type.generate()}>", { m: String -> "$m.into()" })
        // Struct.
        structs.containsKey(member.type.getIdentifier()) ->
            Pair("impl Cast<Target = ${member.type.generate()}>", { m: String -> "$m.into()" })
        // Pointer to struct.
        structs.containsKey(member.type.getPointee()?.getIdentifier()) -> {
            val pointer = member.type as PointerType
            val type = "impl Cast<Target = ${pointer.pointee.generate()}>".generateRef(pointer.const, lifetime = "b")
            val cast = if (pointer.const) { ".as_ref()" } else { ".as_mut()" }
            Pair(type, { m: String -> "$m$cast" })
        }
        // Pointer to string.
        member.type.isStringPointer() -> Pair("&'b [u8]", { m: String -> "$m.as_ptr().cast()" })
        // Pointer to pointer.
        member.type is PointerType && member.type.pointee is PointerType -> {
            if (structs.containsKey(member.type.pointee.pointee.getIdentifier())) {
                // Pointer to pointer to struct.
                val item = member.type.pointee.pointee.generate()
                Pair("&'b [&'b impl Cast<Target = $item>]", { m: String -> "$m.as_ptr().cast()" })
            } else {
                // Pointer to pointer to other type.
                val item = member.type.pointee.pointee.generate()
                Pair("&'b [&'b $item]", { m: String -> "$m.as_ptr().cast()" })
            }
        }
        // Pointer to other type (non-opaque).
        member.type is PointerType && !member.type.isOpaquePointer() -> {
            val item = member.type.pointee.generate()
            val type = item.generateRef(member.type.const, lifetime = "b")
            val cast = item.generatePtr(member.type.const)
            Pair(type, { m: String -> "$m as $cast" })
        }
        // Other.
        else -> Pair(member.type.generate(), { m: String -> m })
    }

    return """
#[inline]
pub fn ${member.name}(mut self, ${member.name}: $type) -> Self {
    self.value.${member.name} = ${cast(member.name.value)};
    self
}
    """
}
