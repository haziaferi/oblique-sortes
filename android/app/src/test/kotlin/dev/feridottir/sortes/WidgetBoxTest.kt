package dev.feridottir.sortes

import org.junit.Assert.assertEquals
import org.junit.Test

/**
 * The arithmetic that used to live in an XML comment.
 *
 * It decides how much of a card the widget shows, and it is the reason the
 * widget may now be made small: a box that cannot hold the card ends it in an
 * ellipsis instead of cutting it, which is only true if this agrees with the
 * height the launcher hands over.
 */
class WidgetBoxTest {

    @Test
    fun `a box the launcher has not measured yet keeps the layout's own ceiling`() {
        assertEquals("no size reported is not a size of zero", WidgetBox.MAX_LINES, WidgetBox.linesFor(0))
        assertEquals(WidgetBox.MAX_LINES, WidgetBox.linesFor(-1))
    }

    @Test
    fun `a box larger than the longest card is still capped at the layout's maxLines`() {
        assertEquals(WidgetBox.MAX_LINES, WidgetBox.linesFor(1000))
    }

    @Test
    fun `a box with no room for a line still shows one`() {
        // Padding and the tap box alone: there is nothing left for text, and a
        // widget showing none of the card would say nothing at all.
        assertEquals(1, WidgetBox.linesFor(2 * WidgetBox.PADDING_DP + WidgetBox.LABEL_DP))
    }

    @Test
    fun `the height for a count of lines is the smallest that holds them`() {
        for (lines in 1..WidgetBox.MAX_LINES) {
            val height = WidgetBox.heightFor(lines)
            assertEquals("${height}dp should hold exactly $lines lines", lines, WidgetBox.linesFor(height))
            if (lines > 1) {
                assertEquals(
                    "a dp less than ${height}dp should hold one line fewer",
                    lines - 1,
                    WidgetBox.linesFor(height - 1),
                )
            }
        }
    }

    /**
     * A reader who has asked for larger text gets fewer lines in the same box.
     * The box is in dp and the card is in sp; a count that ignored the font
     * scale would leave `maxLines` above the lines there is room for, which is
     * the silent clip this whole file exists to stop.
     */
    @Test
    fun `larger text means fewer lines in the same box`() {
        val box = WidgetBox.heightFor(5)
        assertEquals(5, WidgetBox.linesFor(box, 1.0))
        assertEquals(4, WidgetBox.linesFor(box, 1.3))
        assertEquals(3, WidgetBox.linesFor(box, 1.8))
    }

    @Test
    fun `the resize floor is two lines and the default placement is the longest card`() {
        assertEquals(111, WidgetBox.heightFor(WidgetBox.MIN_LINES))
        assertEquals(LONGEST_CARD_LINES, WidgetBox.linesFor(198))
    }

    private companion object {
        /** The 127-character card, wrapped at 12sp in a 180dp-wide widget. */
        const val LONGEST_CARD_LINES = 7
    }
}
