// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.registry

import com.kylemayes.generator.support.getAttributeText
import com.kylemayes.generator.support.getElement
import com.kylemayes.generator.support.getElementText
import com.kylemayes.generator.support.getElements
import com.kylemayes.generator.support.mapNodes
import com.kylemayes.generator.support.queryElements
import org.intellij.lang.annotations.Language
import org.w3c.dom.Element
import org.w3c.dom.Node
import java.math.BigInteger

// ===============================================
// Registry
// ===============================================

/** A Vulkan API registry. */
data class Registry(
    val aliases: Map<Identifier, Typedef>,
    val basetypes: Map<Identifier, Typedef>,
    val bitmasks: Map<Identifier, Bitmask>,
    val constants: Map<Identifier, Constant>,
    val commands: Map<Identifier, Command>,
    val commandAliases: Map<Identifier, Identifier>,
    val enums: Map<Identifier, Enum>,
    val extensions: Map<Identifier, Extension>,
    val functions: Map<Identifier, Function>,
    val handles: Map<Identifier, Handle>,
    val structs: Map<Identifier, Structure>,
    val unions: Map<Identifier, Structure>,
    val versions: Map<Identifier, Version>,
)

fun extractEntities(e: Element): Registry {
    // Extract the bitmasks.
    //
    // All bitmasks have a `VkFlags` typedef in `types/type` that defines the
    // type used to store bitflags for that bitmask but only bitmasks that
    // actually have one or more bitflags have an entry in `enums`. Here we
    // first extract the populated bitmasks from `enums` and then add empty
    // bitmasks for the remaining bitmasks in `types/type`.

    val bitmasks = e.queryEntities("enums[@type='bitmask']", ::extractBitmask).toMutableMap()
    for (bitmask in e.queryEntities("types/type[@category='bitmask' and not(@alias)]", ::extractBitmaskType)) {
        bitmasks.putIfAbsent(bitmask.key, bitmask.value)
    }

    // Extract the commands and command aliases.

    val commands =
        e.queryEntities(
            "commands/command[not(@alias) and (not(@api) or @api='vulkan' or @api='vulkan,vulkanbase')]",
            ::extractCommand,
        ).toMutableMap()

    val commandAliases =
        e.queryElements("commands/command[@alias]").associate {
            val name = it.getAttribute("name").intern()
            val alias = it.getAttribute("alias").intern()
            val command = commands[alias] ?: error("Missing aliased command.")
            commands[name] = command.copy(name = name)
            name to alias
        }

    // Extract the other entities.

    return Registry(
        aliases = e.queryEntities("types/type[@alias and @name]", ::extractAlias),
        basetypes = e.queryEntities("types/type[@category='basetype']", ::extractBasetype),
        bitmasks = bitmasks,
        constants = e.queryEntities("enums[@name='API Constants']/enum[not(@alias)]", ::extractConstant),
        commands = commands,
        commandAliases = commandAliases,
        enums = e.queryEntities("enums[@type='enum']", ::extractEnum),
        extensions = e.queryEntities("extensions/extension", ::extractExtension),
        functions = e.queryEntities("types/type[@category='funcpointer']", ::extractFunction),
        handles = e.queryEntities("types/type[@category='handle' and not(@alias)]", ::extractHandle),
        structs = e.queryEntities("types/type[@category='struct' and not(@alias)]", ::extractStructure),
        unions = e.queryEntities("types/type[@category='union' and not(@alias)]", ::extractStructure),
        versions = e.queryEntities("feature[@api]", ::extractVersion),
    )
}

// ===============================================
// Bitmask
// ===============================================

/** A collection of bitflags. */
data class Bitmask(
    override val name: Identifier,
    override val api: String? = null,
    val bitflags: MutableList<Bitflag>,
) : Entity

private fun extractBitmask(e: Element) =
    Bitmask(
        name = e.getAttribute("name").intern(),
        api = null,
        bitflags =
            e.getElements("enum")
                .filter { !it.hasAttribute("alias") }
                .map(::extractBitflag)
                .toMutableList(),
    )

private fun extractBitmaskType(e: Element) =
    Bitmask(
        name = e.getElementText("name")!!.intern(),
        api = e.getAttributeText("api"),
        bitflags = ArrayList(),
    )

/** A bitflag. */
data class Bitflag(
    override val name: Identifier,
    override val api: String? = null,
    val value: BigInteger,
) : Entity

private fun extractBitflag(e: Element) =
    Bitflag(
        name = e.getAttribute("name").intern(),
        api = e.getAttributeText("api"),
        value =
            e.getAttributeText("bitpos")
                ?.toNumber()
                ?.let { BigInteger.ONE.shiftLeft(it.toInt()) }
                ?: e.getAttribute("value").toNumber().toBigInteger(),
    )

// ===============================================
// Command
// ===============================================

/** A command. */
data class Command(
    override val name: Identifier,
    override val api: String? = null,
    val params: List<Param>,
    val result: Type,
    val successcodes: List<Identifier>,
    val errorcodes: List<Identifier>,
) : Entity

private fun extractCommand(e: Element): Command {
    val proto = e.getElement("proto")!!
    return Command(
        name = proto.getElementText("name")!!.intern(),
        api = e.getAttributeText("api"),
        params = e.queryElements("param", ::extractParam),
        result = extractType(proto.getElement("type")!!),
        successcodes =
            e.getAttributeText("successcodes")
                ?.split(",")
                ?.map { it.intern() }
                ?: emptyList(),
        errorcodes =
            e.getAttributeText("errorcodes")
                ?.split(",")
                ?.map { it.intern() }
                ?: emptyList(),
    )
}

/** A command parameter. */
data class Param(
    override val name: Identifier,
    override val api: String? = null,
    val type: Type,
    val len: Identifier?,
    val arglen: List<Identifier>?,
    val optional: Boolean,
) : Entity

private fun extractParam(e: Element): Param {
    val len = e.getAttributeText("len")
    return Param(
        name = e.getElementText("name")!!.intern(),
        api = e.getAttributeText("api"),
        type = extractType(e.getElement("type")!!),
        len = len?.intern(),
        arglen = len?.split("->")?.map { it.intern() },
        optional = e.getAttributeText("optional")?.startsWith("true") ?: false,
    )
}

// ===============================================
// Constant
// ===============================================

/** A constant. */
data class Constant(
    override val name: Identifier,
    override val api: String? = null,
    val type: IdentifierType,
    val expr: String,
) : Entity

private fun extractConstant(e: Element): Constant {
    val name = e.getAttribute("name")
    val value = e.getAttribute("value")

    val type =
        when {
            name == "VK_TRUE" || name == "VK_FALSE" -> "uint32_t"
            name != "VK_WHOLE_SIZE" && (name.startsWith("VK_MAX") || name.endsWith("SIZE")) -> "size_t"
            value.contains("ULL") -> "uint64_t"
            value.contains("U") -> "uint32_t"
            value.contains(Regex("[fF]$")) -> "float"
            else -> "int32_t"
        }

    val expr =
        if (value.startsWith("(~")) {
            value.replace('~', '!').replace(Regex("(\\d+)U(LL)?"), "$1")
        } else {
            value
        }

    return Constant(
        name = name.intern(),
        api = e.getAttributeText("api"),
        type = IdentifierType(type.intern()),
        expr = expr.trim('(', ')').replace(Regex("[fF]$"), ""),
    )
}

// ===============================================
// Enum
// ===============================================

/** An enum. */
data class Enum(
    override val name: Identifier,
    override val api: String? = null,
    val variants: MutableList<Variant>,
) : Entity

private fun extractEnum(e: Element) =
    Enum(
        name = e.getAttribute("name").intern(),
        api = e.getAttributeText("api"),
        variants =
            e.getElements("enum")
                .filter { !it.hasAttribute("alias") }
                .map(::extractVariant)
                .toMutableList(),
    )

/** An enum variant. */
data class Variant(
    override val name: Identifier,
    override val api: String? = null,
    val value: Long,
) : Entity

private fun extractVariant(e: Element) =
    Variant(
        name = e.getAttribute("name").intern(),
        api = e.getAttributeText("api"),
        value = e.getAttribute("value").toNumber(),
    )

// ================================================
// Function
// ================================================

/** A function pointer. */
data class Function(
    override val name: Identifier,
    override val api: String? = null,
    val params: List<Type>,
    val result: Type?,
) : Entity

private fun extractFunction(e: Element) =
    Function(
        name = e.getElementText("name")!!.intern(),
        api = e.getAttributeText("api"),
        params = e.getElements("type", ::extractType),
        result =
            when (val type = e.textContent.substring(8, e.textContent.indexOf("(VKAPI_PTR")).trim()) {
                "void" -> null
                "void*" -> PointerType(IdentifierType("void".intern()), false)
                "VkBool32" -> IdentifierType("VkBool32".intern())
                "PFN_vkVoidFunction" -> IdentifierType("PFN_vkVoidFunction".intern())
                else -> error("Unsupported function pointer result type ($type).")
            },
    )

// ===============================================
// Handle
// ===============================================

/** A Vulkan handle. */
data class Handle(
    override val name: Identifier,
    override val api: String? = null,
    val dispatchable: Boolean,
) : Entity

private fun extractHandle(e: Element) =
    Handle(
        name = e.getElementText("name")!!.intern(),
        api = e.getAttributeText("api"),
        dispatchable = !e.getElementText("type")!!.contains("NON_DISPATCHABLE"),
    )

// ===============================================
// Structure
// ===============================================

/** A struct or union. */
data class Structure(
    override val name: Identifier,
    override val api: String? = null,
    val members: List<Member>,
    val structextends: List<Identifier>?,
) : Entity

private fun extractStructure(e: Element) =
    Structure(
        name = e.getAttribute("name").intern(),
        api = e.getAttributeText("api"),
        members = e.getElements("member", ::extractMember),
        structextends = e.getAttributeText("structextends")?.split(",")?.map { it.intern() },
    )

/** A struct or union member. */
data class Member(
    override val name: Identifier,
    override val api: String? = null,
    val type: Type,
    val values: Identifier?,
    val len: List<Identifier>?,
    val altlen: String,
    val optional: Boolean,
    val bits: Int?,
) : Entity

private fun extractMember(e: Element) =
    Member(
        name = e.getElementText("name")!!.intern(),
        api = e.getAttributeText("api"),
        type = extractType(e.getElement("type")!!),
        values = e.getAttributeText("values")?.intern(),
        len = e.getAttributeText("len")?.split(",")?.map { it.intern() },
        altlen = e.getAttribute("altlen"),
        optional = e.getAttributeText("optional") == "true",
        bits = Regex(":(\\d+)$").find(e.textContent)?.let { it.groupValues[1].toInt() },
    )

// ===============================================
// Typedef
// ===============================================

/** A type alias. */
data class Typedef(
    override val name: Identifier,
    override val api: String? = null,
    val type: IdentifierType,
) : Entity

private fun extractAlias(e: Element) =
    Typedef(
        name = e.getAttribute("name").intern(),
        api = e.getAttributeText("api"),
        type = IdentifierType(e.getAttribute("alias").intern()),
    )

private fun extractBasetype(e: Element): Typedef? {
    val name = e.getElementText("name") ?: return null
    val type = e.getElementText("type") ?: return null
    return Typedef(
        name = name.intern(),
        api = e.getAttributeText("api"),
        type = IdentifierType(type.intern()),
    )
}

// ===============================================
// Version / Extension
// ===============================================

/** A Vulkan version. */
data class Version(
    override val name: Identifier,
    override val api: String? = null,
    val number: Float,
    val require: Require,
) : Entity

private fun extractVersion(e: Element) =
    Version(
        name = e.getAttribute("name").intern(),
        api = e.getAttributeText("api"),
        number = e.getAttribute("number").toFloat(),
        require = extractRequire(e.getElements("require")),
    )

/** A Vulkan extension. */
data class Extension(
    override val name: Identifier,
    override val api: String? = null,
    val number: Long,
    val type: String?,
    val author: String,
    val contact: String,
    val platform: String?,
    val requires: String?,
    val requiresCore: String?,
    val deprecatedby: String?,
    val obsoletedby: String?,
    val promotedto: String?,
    val supported: String,
    val provisional: Boolean,
    val require: Require,
) : Entity

private fun extractExtension(e: Element) =
    Extension(
        name = e.getAttribute("name").intern(),
        api = e.getAttributeText("api"),
        number = e.getAttribute("number").toNumber(),
        type = e.getAttributeText("type"),
        author = e.getAttribute("author"),
        contact = e.getAttribute("contact"),
        platform = e.getAttributeText("platform"),
        requires = e.getAttributeText("requires"),
        requiresCore = e.getAttributeText("requiresCore"),
        deprecatedby = e.getAttributeText("deprecatedby"),
        obsoletedby = e.getAttributeText("obsoletedby"),
        promotedto = e.getAttributeText("promotedto"),
        supported = e.getAttribute("supported"),
        provisional = e.getAttributeText("provisional") == "true",
        require = extractRequire(e.getElements("require")),
    )

/** The commands, types, and enum extensions provided by a version or extension. */
data class Require(
    val commands: Set<Identifier>,
    val types: Set<String>,
    val values: List<RequireValue>,
)

private fun extractRequire(es: List<Element>): Require {
    val commands = HashSet<Identifier>()
    val types = HashSet<String>()
    val values = ArrayList<RequireValue>()

    for (e in es) {
        commands.addAll(e.getElements("command") { it.getAttribute("name").intern() })
        types.addAll(e.getElements("type") { it.getAttribute("name") })
        values.addAll(e.getElements("enum", ::extractRequireValue))
    }

    return Require(commands, types, values)
}

/** An additional bitmask bitflag or enum variant defined by a version or extension. */
data class RequireValue(
    override val name: Identifier,
    override val api: String? = null,
    val extends: Identifier,
    val value: Long?,
    val bitpos: Long?,
    val extnumber: Long?,
    val offset: Long?,
    val negative: Boolean,
) : Entity

private fun extractRequireValue(e: Element): RequireValue? {
    if (e.hasAttribute("alias")) {
        return null
    }

    val value = e.getAttributeText("value")
    if (value != null && value.startsWith('"')) {
        return null
    }

    return RequireValue(
        name = e.getAttribute("name").intern(),
        api = e.getAttributeText("api"),
        extends = e.getAttribute("extends").intern(),
        value = value?.toNumber(),
        bitpos = e.getAttributeText("bitpos")?.toNumber(),
        extnumber = e.getAttributeText("extnumber")?.toNumber(),
        offset = e.getAttributeText("offset")?.toNumber(),
        negative = e.getAttributeText("dir") == "-",
    )
}

// ===============================================
// Entity
// ===============================================

/** A Vulkan API registry entity (e.g., struct, union, command). */
interface Entity {
    /** The name of this entity. */
    val name: Identifier

    /** The Vulkan API this entity is for. */
    val api: String?

    /** Renames this entity. */
    fun rename(transform: (name: String) -> String) {
        if (!name.renamed) {
            name.rename(transform(name.value))
        }
    }

    /** Gets whether this entity is for the core Vulkan API (and not something like Vulkan SC). */
    fun isVulkanApi(): Boolean = api == null || api?.split(",")?.contains("vulkan") ?: false
}

/** Gets and maps all of the entities in this element with the supplied tag. */
fun <T : Entity> Element.getEntities(
    @Language("XPath") expr: String,
    transform: (Element) -> T?,
) = getElements(expr, transform).associateBy { it.name }

/** Finds the entities in this node that match the supplied XPath expression. */
fun <T : Entity> Node.queryEntities(
    @Language("XPath") expr: String,
    transform: (Element) -> T?,
) = queryElements(expr, transform).associateBy { it.name }

// ===============================================
// Type
// ===============================================

/** The mapping from C/C++ primitive types to Rust primitive types. */
private val primitives =
    mapOf(
        "void" to "c_void",
        "char" to "c_char",
        "int" to "c_int",
        "size_t" to "usize",
        "int32_t" to "i32",
        "int64_t" to "i64",
        "uint8_t" to "u8",
        "uint16_t" to "u16",
        "uint32_t" to "u32",
        "uint64_t" to "u64",
        "float" to "f32",
        "double" to "f64",
    )

/** A C/C++ type. */
interface Type {
    /** Generates the Rust type for this type. */
    fun generate(): String

    /** Generates the Rust type for this type as it would appear in a command. */
    fun generateForCommand(): String = generate()

    /** Generates the Rust default expression for this type. */
    fun generateDefault(): String
}

private fun extractType(e: Element): Type {
    val identifier = IdentifierType(e.textContent.intern())

    // Array types, e.g.:
    // `<type>float</type> <name>matrix</name>[3][4]`
    // `<type>uint8_t</type> <name>deviceUUID</name>[<enum>VK_UUID_SIZE</enum>]`
    if (e.parentNode is Element) {
        val contents =
            e.parentNode.childNodes
                .mapNodes { it }
                .filter { it.nodeType == Node.TEXT_NODE || (it is Element && it.tagName == "enum") }
                .joinToString("") { it.textContent }
        val lengths =
            Regex("\\[([^]]+)]")
                .findAll(contents)
                .map { it.groups[1]!!.value.intern() }
                .toList()
                .reversed()
        if (lengths.isNotEmpty()) {
            var array = ArrayType(identifier, lengths[0])
            lengths.subList(1, lengths.size).forEach { array = ArrayType(array, it) }
            return array
        }
    }

    // Pointer types, e.g.:
    // `<type>void</type>*`
    // `const <type>char</type>* const*`
    val next = e.nextSibling?.textContent?.trim()
    if (next != null && next.startsWith("*")) {
        val previous = e.previousSibling ?: e.parentNode
        val const = previous?.textContent?.contains("const") ?: false
        val pointer = PointerType(identifier, const)
        return when {
            next.startsWith("* const*") -> PointerType(pointer, true)
            next.startsWith("**") -> PointerType(pointer, const)
            else -> pointer
        }
    }

    return identifier
}

fun Type.getBaseIdentifier(): Identifier? =
    when (this) {
        is ArrayType -> element.getBaseIdentifier()
        is IdentifierType -> identifier
        is PointerType -> null
        else -> error("Unreachable.")
    }

// Array =========================================

/** A C/C++ fixed-length array type. */
data class ArrayType(val element: Type, val length: Identifier) : Type {
    override fun generate() =
        when (element.getIdentifier()?.original) {
            "char" -> "StringArray<$length>"
            "uint8_t" -> "ByteArray<$length>"
            else -> "[${element.generate()}; $length]"
        }

    override fun generateForCommand() = "*const ${element.generateForCommand()}"

    override fun generateDefault() =
        when (element.getIdentifier()?.original) {
            "char" -> "StringArray::default()"
            "uint8_t" -> "ByteArray::default()"
            else -> "[${element.generateDefault()}; $length]"
        }
}

fun Type.getElement() = if (this is ArrayType) element else null

fun Type.getLength() = if (this is ArrayType) length else null

fun Type.isByteArray() = getElement()?.getIdentifier()?.original == "uint8_t"

fun Type.isStringArray() = getElement()?.getIdentifier()?.original == "char"

// Identifier ====================================

/** A C/C++ identifier type. */
data class IdentifierType(val identifier: Identifier) : Type {
    override fun generate() =
        if (identifier.value.startsWith("StdVideo")) {
            // Types from the Vulkan video headers are prefixed with `StdVideo`
            // and are in a separate `video` module from the registry types.
            "video::${identifier.value}"
        } else {
            primitives.getOrDefault(identifier.value, identifier.value)
        }

    override fun generateDefault() = "${generate()}::default()"
}

fun Type.getIdentifier() = if (this is IdentifierType) identifier else null

// Pointer =======================================

/** A C/C++ pointer type. */
data class PointerType(val pointee: Type, val const: Boolean) : Type {
    override fun generate() = "*${if (const) "const" else "mut"} ${pointee.generate()}"

    override fun generateDefault() = if (const) "ptr::null()" else "ptr::null_mut()"
}

fun Type.getPointee() = if (this is PointerType) pointee else null

fun Type.isPointer() = this is PointerType || isPlatformPointer()

fun Type.isOpaquePointer() = getPointee()?.getIdentifier()?.let { opaque.contains(it.value) } ?: false

fun Type.isPlatformPointer() = platformPointers.contains(getIdentifier()?.value)

fun Type.isStringPointer() = getPointee()?.getIdentifier()?.value == "char"

/** The types which are used in opaque pointers (i.e., `void` and `void` typedefs). */
private val opaque =
    setOf(
        "c_void",
        "ANativeWindow",
        "AHardwareBuffer",
        "IDirectFB",
        "IDirectFBSurface",
        "CAMetalLayer",
        "_screen_context",
        "_screen_window",
        "wl_display",
        "wl_surface",
        "SECURITY_ATTRIBUTES",
        "xcb_connection_t",
        "OHNativeWindow",
    )

/** The platform typedefs which are aliases of pointer types. */
private val platformPointers =
    setOf(
        // iOS / macOS
        "IOSurfaceRef",
        "MTLBuffer_id",
        "MTLCommandQueue_id",
        "MTLDevice_id",
        "MTLSharedEvent_id",
        "MTLTexture_id",
        // Windows
        "HANDLE",
        "HINSTANCE",
        "HMONITOR",
        "HWND",
        "LPCWSTR",
        // X11
        "Display",
        // NvSciBuf / NvSciSync
        "NvSciBufAttrList",
        "NvSciBufObj",
        "NvSciSyncAttrList",
        "NvSciSyncObj",
    )

// ===============================================
// Identifier
// ===============================================

/** The interned identifiers mapped by their original value. */
private val identifiers = HashMap<String, Identifier>()

/** Gets or creates the interned identifier for this string. */
fun String.intern(): Identifier {
    // Vulkan uses `FlagBits` to indicate bitmask types in some places and
    // `Flags` in others. We normalize all bitmask identifiers to use `Flags`
    // here for the sake of my sanity.
    val string = this.replace(Regex("FlagBits(\\d*)([A-Z]+)?$"), "Flags$1$2")
    return identifiers.getOrPut(string) { Identifier(string) }
}

/**
 * An interned identifier.
 *
 * This class exists so that each identifier in a Vulkan API registry (e.g.,
 * `VkBool32` or `VK_UUID_SIZE`) may be represented by a single interned
 * instance of `Identifier`. As this class is a wrapper around a mutable
 * `String` reference, this allows for easy and efficient renaming of
 * identifiers from Vulkan API registries.
 */
class Identifier constructor(val original: String) : Comparable<Identifier> {
    private var _value = original
    private var _renamed = false
    val value get() = _value
    val renamed get() = _renamed

    fun rename(value: String) {
        _value = value
        _renamed = true
    }

    override fun toString() = value

    override fun equals(other: Any?) = other is Identifier && original == other.original

    override fun hashCode() = original.hashCode()

    override fun compareTo(other: Identifier) = original.compareTo(other.original)
}

// ===============================================
// Shared
// ===============================================

/** Converts this decimal or hexadecimal string to a number. */
fun String.toNumber() =
    if (startsWith("0x")) {
        substring(2).toLong(16)
    } else {
        toLong(10)
    }
