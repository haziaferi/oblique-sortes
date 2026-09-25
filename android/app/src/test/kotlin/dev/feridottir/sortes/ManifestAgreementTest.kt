package dev.feridottir.sortes

import java.io.File
import javax.xml.parsers.DocumentBuilderFactory
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import org.w3c.dom.Document
import org.w3c.dom.Element

/**
 * The XML and the Kotlin agree on the facts they both state.
 *
 * Each of these is one fact written in two places, where the two places drift
 * apart with no compile error and no visible failure — a broadcast delivered to
 * nothing, a widget that quietly starts redrawing itself, a README that says
 * "one layout" over a tree that has two. The tests read the resource files as
 * plain XML from the source tree, so they run on the JVM with no device, the way
 * the rest of this suite does.
 *
 * The pattern is mnemo's WidgetSizeClassTest, which pins its Kotlin size ladder
 * to its provider XML for the same reason.
 */
class ManifestAgreementTest {

    private val main: File = generateSequence(File(System.getProperty("user.dir")).absoluteFile) { it.parentFile }
        .map { File(it, "src/main") }
        .firstOrNull { File(it, "AndroidManifest.xml").isFile }
        ?: error("no src/main/AndroidManifest.xml above ${System.getProperty("user.dir")}")

    // Namespace-aware, or `getAttributeNS` sees no `android:` attributes at all
    // and every assertion below would compare against an empty string.
    private fun xml(relative: String): Document =
        DocumentBuilderFactory.newInstance().apply { isNamespaceAware = true }
            .newDocumentBuilder().parse(File(main, relative))

    private fun Document.elements(tag: String): List<Element> =
        getElementsByTagName(tag).let { nodes -> (0 until nodes.length).map { nodes.item(it) as Element } }

    private fun Element.android(attribute: String): String =
        getAttributeNS("http://schemas.android.com/apk/res/android", attribute)

    /**
     * The receiver's tap action is the same string CardWidget sends. Written in
     * both the manifest and the Kotlin, and if either is edited alone the tap goes
     * to no one: `onReceive` never sees it, the card never changes, nothing logs.
     */
    @Test
    fun `the manifest and the widget agree on the tap action`() {
        val receiver = xml("AndroidManifest.xml").elements("receiver").single()
        val actions = receiver.getElementsByTagName("action")
            .let { nodes -> (0 until nodes.length).map { (nodes.item(it) as Element).android("name") } }
        assertTrue("the receiver filter should carry ${CardWidget.ACTION_DRAW}, has $actions",
                   CardWidget.ACTION_DRAW in actions)
    }

    /**
     * CardWidget's KDoc promises the widget never redraws by itself. That promise
     * is one attribute in the provider XML, and this is what holds the KDoc to it.
     */
    @Test
    fun `the widget never redraws on a timer`() {
        val provider = xml("res/xml/card_widget_info.xml").documentElement
        assertEquals("0", provider.android("updatePeriodMillis"))
    }

    /**
     * The README says the app is one Activity and one widget, with no permissions
     * and one XML layout that belongs to the widget. Each is a count over the tree.
     */
    @Test
    fun `one activity, one receiver, no permissions`() {
        val manifest = xml("AndroidManifest.xml")
        assertEquals(1, manifest.elements("activity").size)
        assertEquals(1, manifest.elements("receiver").size)
        assertEquals("the app asks for no permissions", 0, manifest.elements("uses-permission").size)
    }

    /**
     * `WidgetBox` works in dp and sp that are really written in the widget's
     * XML, where the views that spend them live. It is the same fact in two
     * files: edit one and the widget goes on rendering, with the lines it
     * allows the card no longer the lines the card has room for.
     */
    @Test
    fun `the widget's box is the one the layout describes`() {
        val layout = xml("res/layout/card_widget.xml")
        val views = layout.elements("TextView").associateBy { it.android("id") }

        val label = views.getValue("@+id/widget_deck")
        assertEquals("the tap that opens the app is a platform-minimum target",
                     "${WidgetBox.LABEL_DP}dp", label.android("minHeight"))

        val card = views.getValue("@+id/widget_card")
        assertEquals("uniform", card.android("autoSizeTextType"))
        assertEquals("the smallest the card is allowed to get",
                     WidgetBox.MIN_SP, card.android("autoSizeMinTextSize").removeSuffix("sp").toDouble(), 0.0)
        assertEquals("the leading a line of card carries",
                     WidgetBox.LINE_EXTRA_DP, card.android("lineSpacingExtra").removeSuffix("dp").toDouble(), 0.0)
        assertEquals("the ceiling no box may exceed",
                     WidgetBox.MAX_LINES.toLong(), card.android("maxLines").toLong())
        assertEquals("the padding the box spends twice",
                     WidgetBox.PADDING_DP.toLong(), layout.documentElement.android("padding").removeSuffix("dp").toLong())
    }

    /**
     * The default placement shows the longest card whole, and the widget can
     * still be made small: the two used to be one number, and declaring only
     * `minHeight` left a widget that could not be resized below the size the
     * longest card needs -- 246x293dp on the launcher grid, for a median card
     * that fills under a third of it.
     */
    @Test
    fun `the widget's floor holds its longest card and its resize floor holds two lines`() {
        val provider = xml("res/xml/card_widget_info.xml").documentElement
        val default = provider.android("minHeight").removeSuffix("dp").toInt()
        val floor = provider.android("minResizeHeight").removeSuffix("dp").toInt()

        assertEquals("the default placement holds the longest card whole",
                     LONGEST_CARD_LINES.toLong(), WidgetBox.linesFor(default).toLong())
        assertEquals("the resize floor is the room two lines need",
                     WidgetBox.heightFor(WidgetBox.MIN_LINES).toLong(), floor.toLong())
        assertTrue("a resize floor at the default size is no floor at all", floor < default)
    }

    private companion object {
        /** The 127-character card, wrapped at 12sp in a 180dp-wide widget. */
        const val LONGEST_CARD_LINES = 7
    }

    @Test
    fun `the one layout is the widget's`() {
        val layouts = File(main, "res/layout").listFiles { f -> f.extension == "xml" }.orEmpty().map { it.name }
        assertEquals(listOf("card_widget.xml"), layouts)
        val provider = xml("res/xml/card_widget_info.xml").documentElement
        assertEquals("@layout/card_widget", provider.android("initialLayout"))
    }
}
