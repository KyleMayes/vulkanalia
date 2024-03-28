// SPDX-License-Identifier: Apache-2.0

package com.kylemayes.generator.support

import org.intellij.lang.annotations.Language
import org.w3c.dom.Element
import org.w3c.dom.Node
import org.w3c.dom.NodeList
import javax.xml.xpath.XPathConstants
import javax.xml.xpath.XPathFactory

// ===============================================
// Element
// ===============================================

/** Gets the text content of an attribute in this element with the supplied name. */
fun Element.getAttributeText(name: String): String? = if (hasAttribute(name)) getAttribute(name) else null

/** Gets the text content of the first child element in this element with the supplied tag. */
fun Element.getElementText(tag: String): String? = getElement(tag)?.textContent

/** Gets the first child element in this element with the supplied tag. */
fun Element.getElement(tag: String): Element? = getElements(tag).firstOrNull()

/** Gets and maps all of the child elements in this element with the supplied tag. */
fun <T> Element.getElements(
    tag: String,
    transform: (Element) -> T?,
): List<T> = getElements(tag).mapNotNull(transform)

/** Gets all of the child elements in this element with the supplied tag. */
fun Element.getElements(tag: String): List<Element> = getElementsByTagName(tag).mapNodes { it as Element }

// ===============================================
// Node
// ===============================================

/** Finds and maps the elements in this node that match the supplied XPath expression. */
fun <T> Node.queryElements(
    @Language("XPath") expr: String,
    transform: (Element) -> T?,
): List<T> = queryElements(expr).mapNotNull(transform)

/** Finds the elements in this node that match the supplied XPath expression. */
fun Node.queryElements(
    @Language("XPath") expr: String,
): List<Element> {
    val xpath = XPathFactory.newInstance().newXPath()
    val nodes = xpath.evaluate(expr, this, XPathConstants.NODESET) as NodeList
    return nodes.mapNodes { it as Element }
}

// ===============================================
// NodeList
// ===============================================

/** Maps the non-null nodes in this node list. */
fun <T> NodeList.mapNodes(transform: (Node) -> T): List<T> = (0..length).mapNotNull(::item).map(transform)
