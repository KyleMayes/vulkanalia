// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.registry

import com.kylemayes.generator.support.time
import mu.KotlinLogging
import org.xml.sax.InputSource
import java.io.StringReader
import javax.xml.parsers.DocumentBuilderFactory

private val log = KotlinLogging.logger { /* */ }

/** Parses a Vulkan API registry from an XML string. */
fun parseRegistry(xml: String): Registry {
    val document = log.time("Parse XML") {
        val builder = DocumentBuilderFactory.newInstance().newDocumentBuilder()
        builder.parse(InputSource(StringReader(xml))).documentElement
    }

    val registry = log.time("Extract Entities") { extractEntities(document) }
    log.time("Extend Entities") { registry.extendEntities() }
    log.time("Rename Entities") { registry.renameEntities() }
    return registry
}
