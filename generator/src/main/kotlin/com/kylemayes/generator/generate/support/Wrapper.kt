// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.generate.support

import com.kylemayes.generator.registry.ArrayType
import com.kylemayes.generator.registry.Command
import com.kylemayes.generator.registry.Param
import com.kylemayes.generator.registry.PointerType
import com.kylemayes.generator.registry.Registry
import com.kylemayes.generator.registry.getIdentifier
import com.kylemayes.generator.registry.isOpaquePointer
import com.kylemayes.generator.support.PeekableIterator

/** Generates a more Rust-friendly wrapper method around a command for a version or extension trait. */
fun Registry.generateCommandWrapper(command: Command): String {
    val type = getCommandType(command)
    val hasSuccessCodes = getCommandSuccessCodes(command).isNotEmpty()

    // The Rust method type parameters.
    val typeParams = mutableListOf<String>()
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

    fun addArgument(actual: String, setup: String? = null) {
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

        // Each command parameter that follows the current command parameter
        // that has a `len` attribute that equals the name of the current
        // command parameter represents a slice parameter that uses the current
        // parameter as the length.
        fun Param?.isSlice() = this?.len?.value == current.name.value

        if (iterator.peek().isSlice()) {
            // Slice parameter(s).

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
                    val length = if (outputWithLength) { current.name.value } else { "${slices[0].name}.len()" }
                    resultTypes.add("Vec<${pointee.generate()}>")
                    resultExprs.add(slice.name.value)
                    preActualStmts.add("let mut ${slice.name} = Vec::with_capacity($length as usize);")
                    postActualStmts.add("debug_assert!(${slice.name}.capacity() == $length as usize);")
                    postActualStmts.add("unsafe { ${slice.name}.set_len($length as usize) };")
                    addArgument("${slice.name}.as_mut_ptr()", setup = "ptr::null_mut()")
                } else {
                    // Input slice parameter.

                    val (item, cast) = when {
                        structs.containsKey(pointee.getIdentifier()) -> Pair(
                            "impl Cast<Target=${pointee.generate()}>",
                            ".cast()"
                        )
                        pointee.getIdentifier()?.value == "void" -> Pair(
                            "u8",
                            "as ${"c_void".generatePtr(pointer.const)}"
                        )
                        pointee is PointerType -> Pair(
                            "&${pointee.pointee.generate()}",
                            ".cast()"
                        )
                        else -> Pair(pointee.generate(), "")
                    }

                    params.add("${slice.name}: ${"[$item]".generateRef(pointer.const)}")
                    if (index == 0) addArgument("${slice.name}.len() as ${current.type.generate()}")
                    addArgument("${slice.name}.as_ptr()$cast")
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
            // Output slice parameter (length determined from argument field).
            //
            // For example, `len="pAllocateInfo-&gt;descriptorSetCount"`
            // indicates that the length of the output slice is equivalent to
            // the value of the `descriptorSetCount` field in the
            // `pAllocateInfo` argument.
            val pointer = current.type as PointerType
            val length = "${current.arglen[0]}.as_ref().${current.arglen[1]}"
            resultTypes.add("Vec<${pointer.pointee.generate()}>")
            resultExprs.add(current.name.value)
            preActualStmts.add("let mut ${current.name} = Vec::with_capacity($length as usize);")
            postActualStmts.add("unsafe { ${current.name}.set_len($length as usize) };")
            addArgument("${current.name}.as_mut_ptr()")
        } else if (current.type is PointerType) {
            // Pointer parameter.
            val pointee = current.type.pointee
            if (!current.type.const) {
                // Output pointer parameter.

                val (pointeeType, argCast) = if (current.type.isOpaquePointer()) {
                    val typeParam = "T_${pointee.generate()}"
                    typeParams.add(typeParam)
                    Pair(typeParam, ".cast::<c_void>()")
                } else {
                    Pair(pointee.generate(), "")
                }

                val (resultType, exprCast) = if (pointeeType == "Bool32") {
                    Pair("bool", " == TRUE")
                } else {
                    Pair(pointeeType, "")
                }

                preActualStmts.add("let mut ${current.name} = MaybeUninit::<$pointeeType>::uninit();")
                resultTypes.add(resultType)
                resultExprs.add("unsafe { ${current.name}.assume_init() }$exprCast")
                addArgument("${current.name}.as_mut_ptr()$argCast")
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

    val generics = typeParams.joinToString(prefix = "<", postfix = ">").replace("<>", "")
    val fallible = command.result.getIdentifier()?.value == "Result"
    val resultType = resultTypes.joinTuple()
    val outputType = when {
        hasSuccessCodes && resultType == "()" -> "-> crate::VkResult<SuccessCode>"
        hasSuccessCodes -> "-> crate::VkSuccessResult<$resultType>"
        fallible -> "-> crate::VkResult<$resultType>"
        resultType != "()" -> " -> $resultType"
        else -> ""
    }

    // Generate setup command invocation, if required.

    val setup = if (preSetupStmts.isNotEmpty()) {
        """
${preSetupStmts.joinToString("")}

${generateInvocation(command, setupArgs)};
        """
    } else {
        ""
    }

    // Generate actual command invocation.

    val resultExpr = resultExprs.joinTuple()
    val outputExpr = when {
        hasSuccessCodes ->
            """
if __result >= Result::SUCCESS {
    Ok(${if (resultExpr != "()") { "($resultExpr, __result.into())" } else { "__result.into()" }})
} else {
    Err(__result.into())
}
            """
        fallible ->
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
fn ${command.name}$generics(&self, ${params.joinToString()})$outputType {
    $setup
    $actual
}
    """
}

/** Generates a Rust expression which invokes a command in a version or extension trait method. */
private fun generateInvocation(command: Command, arguments: List<String>): String {
    return "(self.commands().${command.name})(${arguments.joinToString()})"
}

/** Joins a list of strings as a Rust tuple expression or type. */
private fun List<String>.joinTuple() = when (size) {
    0 -> "()"
    1 -> this[0]
    else -> joinToString(prefix = "(", postfix = ")")
}
