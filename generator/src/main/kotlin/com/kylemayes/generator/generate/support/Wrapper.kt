// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.support

import com.kylemayes.generator.registry.ArrayType
import com.kylemayes.generator.registry.Command
import com.kylemayes.generator.registry.Param
import com.kylemayes.generator.registry.PointerType
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.getIdentifier
import com.kylemayes.generator.registry.isOpaquePointer
import com.kylemayes.generator.registry.isPointer
import com.kylemayes.generator.support.PeekableIterator

/** Generates a more Rust-friendly wrapper method around a command for a version or extension trait. */
fun Registry.generateCommandWrapper(command: Command): String {
    val type = getCommandType(command)
    val hasSuccessCodes = getCommandSuccessCodes(command).isNotEmpty()
    val hasErrorCodes = command.result.getIdentifier()?.value == "Result"

    // The Rust method parameters.
    val params = mutableListOf<String>()
    // The Rust method result types (which will combined into a single type).
    val resultTypes = mutableListOf<String>()
    // The Rust method result expressions (which will combined into a single expression).
    val resultExprs = mutableListOf<String>()
    // The Rust statements before the setup command invocation.
    val preSetupStmts = mutableListOf<String>()
    // The Rust statements before the actual command invocation.
    val preActualStmts = mutableListOf<String>()
    // The Rust statements after the actual command invocation.
    val postActualStmts = mutableListOf<String>()
    // The Rust arguments for the command during setup (if required).
    val setupArgs = mutableListOf<String>()
    // The Rust arguments for the command after setup.
    val actualArgs = mutableListOf<String>()

    // Some commands return a value directly instead of returning values using
    // output pointer parameters.
    if (!hasErrorCodes && command.result.getIdentifier()?.value != "void") {
        resultTypes.add(command.result.generate())
        resultExprs.add("__result")
    }

    fun addArgument(
        actual: String,
        setup: String? = null,
    ) {
        setupArgs.add(setup ?: actual)
        actualArgs.add(actual)
    }

    val iterator = PeekableIterator(command.params)

    // If the type of the first command parameter matches the type of the
    // command, the first command parameter must be either an `Instance` or
    // `Device` handle and will be provided by the version or extension trait.
    if (iterator.peek()?.type?.getIdentifier()?.value == type.display) {
        iterator.advance()
        addArgument("self.handle()")
    }

    while (!iterator.isEmpty()) {
        val current = iterator.advance()

        // Adds an input or output slice parameter with a length expression
        // that uses the values of other parameters.
        fun addLengthSliceParam(length: String) {
            val pointer = current.type as PointerType
            val pointee = pointer.pointee
            if (pointer.const) {
                // Input slice parameter.
                val (item, cast) = generateInputSliceTypeAndCast(pointer)
                params.add("${current.name}: ${"[$item]".generateRef(true)}")
                addArgument("${current.name}.as_ptr()$cast")
            } else {
                // Output slice parameter.
                resultTypes.add("Vec<${pointee.generate()}>")
                resultExprs.add(current.name.value)
                preActualStmts.add("let mut ${current.name} = Vec::with_capacity($length as usize);")
                postActualStmts.add("${current.name}.set_len($length as usize);")
                addArgument("${current.name}.as_mut_ptr()")
            }
        }

        // Each command parameter that follows the current command parameter
        // that has a `len` attribute that equals the name of the current
        // command parameter represents a slice parameter that uses the current
        // parameter as the length.
        fun Param?.isSlice() = this?.len?.value == current.name.value

        if (iterator.peek().isSlice()) {
            // Slice parameter(s) (length determined by current parameter).

            val outputWithLength = current.type is PointerType
            if (outputWithLength) {
                preSetupStmts.add("let mut ${current.name} = 0;")
                addArgument("&mut ${current.name}")
            }

            val slices = iterator.takeWhile { it.isSlice() }
            for ((index, slice) in slices.withIndex()) {
                val pointer = slice.type as PointerType
                val pointee = pointer.pointee

                // Sometimes input slice parameters can be followed by
                // output slice parameters that reuse the length from the
                // input slice parameter rather than using a setup command
                // invocation to determine the length.
                val outputWithoutLength = index != 0 && !pointer.const

                if (outputWithLength || outputWithoutLength) {
                    // Output slice parameter.

                    val length =
                        if (outputWithLength) {
                            current.name.value
                        } else {
                            "${slices[0].name}.len()"
                        }

                    if (pointee.getIdentifier()?.value == "void") {
                        resultTypes.add("Vec<u8>")
                        addArgument("${slice.name}.as_mut_ptr() as *mut c_void", setup = "ptr::null_mut()")
                    } else {
                        resultTypes.add("Vec<${pointee.generate()}>")
                        addArgument("${slice.name}.as_mut_ptr()", setup = "ptr::null_mut()")
                    }

                    resultExprs.add(slice.name.value)
                    preActualStmts.add("let mut ${slice.name} = Vec::with_capacity($length as usize);")
                    postActualStmts.add("debug_assert!(${slice.name}.capacity() == $length as usize);")
                    postActualStmts.add("${slice.name}.set_len($length as usize);")
                } else {
                    // Input slice parameter.

                    val conv =
                        if (pointer.const) {
                            ".as_ptr()"
                        } else {
                            ".as_mut_ptr()"
                        }

                    val (item, cast) = generateInputSliceTypeAndCast(pointer)
                    params.add("${slice.name}: ${"[$item]".generateRef(pointer.const)}")
                    if (index == 0) addArgument("${slice.name}.len() as ${current.type.generate()}")
                    addArgument("${slice.name}$conv$cast")
                }
            }
        } else if (current.len?.value == "null-terminated") {
            // String parameter.
            if (current.optional) {
                // String parameter (optional).
                params.add("${current.name}: Option<&[u8]>")
                addArgument("${current.name}.map_or(ptr::null(), |v| v.as_ptr().cast())")
            } else {
                // String parameter (required).
                params.add("${current.name}: &[u8]")
                addArgument("${current.name}.as_ptr().cast()")
            }
        } else if (current.arglen != null && current.arglen.size == 2) {
            // Slice parameter (length determined from argument field).
            //
            // For example, `len="pAllocateInfo-&gt;descriptorSetCount"`
            // indicates that the length of the slice parameter is equivalent to
            // the value of the `descriptorSetCount` field in the
            // `pAllocateInfo` argument.
            addLengthSliceParam("${current.arglen[0]}.as_ref().${current.arglen[1]}")
        } else if (current.len != null && command.params.any { it.name == current.len }) {
            // Slice parameter (length determined from argument slice length).
            val slice = command.params.find { it.len == current.len && it.type.isPointer() }
            addLengthSliceParam("${slice!!.name.value}.len()")
        } else if (current.type is PointerType) {
            // Pointer parameter.
            val pointee = current.type.pointee

            // Non-const pointer parameters are usually used for command
            // outputs. However, non-const pointer parameters are used for
            // command inputs when the pointer is to an opaque platform type
            // (e.g., a Wayland display [`wl_display*`]).
            val output = !current.type.const && !current.type.isOpaquePointer()

            // The user needs to be able to supply a command output as a command
            // input when the output type is an extendable struct so that the
            // user can provide the extension structs for the information they
            // want to be populated in the output parameter.
            val extendable = getChainStructs().containsKey(pointee.getIdentifier())

            if (output && !extendable) {
                // Output pointer parameter (uninit-provided).

                val pointeeType = pointee.generate()
                val (resultType, exprCast) =
                    if (pointeeType == "Bool32") {
                        Pair("bool", " == TRUE")
                    } else {
                        Pair(pointeeType, "")
                    }

                preActualStmts.add("let mut ${current.name} = MaybeUninit::<$pointeeType>::uninit();")
                resultTypes.add(resultType)
                resultExprs.add("${current.name}.assume_init()$exprCast")
                addArgument("${current.name}.as_mut_ptr()")
            } else if (output) {
                // Output pointer parameter (user-provided).
                if (current.optional) {
                    params.add("${current.name}: Option<&mut ${pointee.generate()}>")
                    addArgument("${current.name}.map_or(ptr::null_mut(), |v| v)", "ptr::null_mut()")
                } else {
                    params.add("${current.name}: &mut ${pointee.generate()}")
                    addArgument(current.name.value, "ptr::null_mut()")
                }
            } else if (current.type.isOpaquePointer()) {
                // Input pointer parameter (opaque).
                params.add("${current.name}: ${current.type.generate()}")
                addArgument(current.name.value)
            } else if (current.optional) {
                // Input pointer parameter (optional).
                params.add("${current.name}: Option<&${pointee.generate()}>")
                addArgument("${current.name}.map_or(ptr::null(), |v| v)")
            } else {
                // Input pointer parameter (required).
                params.add("${current.name}: &${pointee.generate()}")
                addArgument(current.name.value)
            }
        } else if (current.type is ArrayType) {
            // Array parameter.
            params.add("${current.name}: ${current.type.generate()}")
            addArgument("${current.name}.as_ptr()")
        } else if (current.type.getIdentifier()?.value == "Bool32") {
            // Value parameter (boolean).
            params.add("${current.name}: bool")
            addArgument("${current.name} as Bool32")
        } else {
            // Value parameter (non-boolean).
            params.add("${current.name}: ${current.type.generate()}")
            addArgument(current.name.value)
        }
    }

    // Generate method signature components.

    val resultType = resultTypes.joinTuple()
    val outputType =
        when {
            hasSuccessCodes && resultType == "()" -> "-> crate::VkResult<SuccessCode>"
            hasSuccessCodes -> "-> crate::VkSuccessResult<$resultType>"
            hasErrorCodes -> "-> crate::VkResult<$resultType>"
            resultType != "()" -> " -> $resultType"
            else -> ""
        }

    // Generate setup command invocation, if required.

    val setup =
        if (preSetupStmts.isNotEmpty()) {
            """
${preSetupStmts.joinToString("")}

${generateInvocation(command, setupArgs)};
        """
        } else {
            ""
        }

    // Generate actual command invocation.

    val resultExpr = resultExprs.joinTuple()
    val outputExpr =
        when {
            hasSuccessCodes ->
                """
if __result >= Result::SUCCESS {
    Ok(${if (resultExpr != "()") "($resultExpr, __result.into())" else "__result.into()"})
} else {
    Err(__result.into())
}
            """
            hasErrorCodes ->
                """
if __result == Result::SUCCESS {
    Ok($resultExpr)
} else {
    Err(__result.into())
}
            """
            resultExpr != "()" -> resultExpr
            else -> ""
        }

    val actual =
        """
${preActualStmts.joinToString("")}

let __result = ${generateInvocation(command, actualArgs)};

${postActualStmts.joinToString("")}

$outputExpr
        """

    // Generate wrapper method.

    return """
/// <${generateManualUrl(command)}>
#[inline]
unsafe fn ${command.name}(&self, ${params.joinToString()})$outputType {
    $setup
    $actual
}
    """
}

/** Generates the Rust type and cast expression suffix for an input slice parameter. */
private fun Registry.generateInputSliceTypeAndCast(pointer: PointerType): Pair<String, String> =
    when {
        structs.containsKey(pointer.pointee.getIdentifier()) ->
            Pair(
                "impl Cast<Target=${pointer.pointee.generate()}>",
                ".cast()",
            )
        pointer.pointee.getIdentifier()?.value == "void" ->
            Pair(
                "u8",
                "as ${"c_void".generatePtr(pointer.const)}",
            )
        pointer.pointee is PointerType ->
            Pair(
                "&[${generateInputSliceTypeAndCast(pointer.pointee).first}]",
                ".cast()",
            )
        else -> Pair(pointer.pointee.generate(), "")
    }

/** Generates a Rust expression which invokes a command in a version or extension trait method. */
private fun generateInvocation(
    command: Command,
    arguments: List<String>,
): String {
    return "(self.commands().${command.name})(${arguments.joinToString()})"
}

/** Joins a list of strings as a Rust tuple expression or type. */
private fun List<String>.joinTuple() =
    when (size) {
        0 -> "()"
        1 -> this[0]
        else -> joinToString(prefix = "(", postfix = ")")
    }
