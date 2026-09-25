package dev.feridottir.sortes

import kotlin.math.floor

/**
 * How many lines of card the widget's box can hold at the height the launcher
 * gave it.
 *
 * `ellipsize` fires only at `maxLines`, so a widget shorter than its text clips
 * it with no sign it was cut. The provider's `minHeight` used to be the whole
 * answer: it was set to what the longest card needs, which made every placement
 * that tall and — since a provider that declares no `minResize*` is resizable
 * only down to its own minimum — made that the smallest the widget could be.
 * Measured on the launcher grid it came to 246x293dp, while the median card
 * fills under a third of it.
 *
 * So the floor is now the room two lines need, and the cut is made honest
 * instead: [linesFor] says how many lines fit, `CardWidget` sets that as the
 * card's `maxLines`, and the ellipsis appears exactly when the card really is
 * too long for the box.
 *
 * Every number here is also written in the widget's XML, where the views that
 * use them live. `ManifestAgreementTest` holds the two copies together.
 */
internal object WidgetBox {

    /** `android:padding` on the widget's root, which it spends twice. */
    const val PADDING_DP = 14

    /** `android:minHeight` on the deck name: the tap that opens the app. */
    const val LABEL_DP = 48

    /** `autoSizeMinTextSize` on the card: the smallest the text is allowed. */
    const val MIN_SP = 12.0

    /** `android:lineSpacingExtra` on the card. */
    const val LINE_EXTRA_DP = 3.0

    /** `android:maxLines` on the card, and so the most any box can show. */
    const val MAX_LINES = 8

    /** The fewest lines worth keeping a widget for, and so the resize floor. */
    const val MIN_LINES = 2

    /**
     * One line of card at the smallest size it autosizes to, in dp.
     *
     * The text is in sp and the box is in dp, and the two are the same length
     * only at a font scale of 1. A reader who has asked for larger text gets
     * taller lines in the same box: at 1.3 a line is 21.7dp, not 17.4, and a
     * count worked out at 1.0 would promise five lines to a box that holds
     * four -- and clip, because `maxLines` would be higher than the lines
     * there is room for and the ellipsis fires only when it is reached. The
     * padding and the label's box are dp and do not move.
     */
    fun lineDp(fontScale: Double = 1.0): Double = 1.2 * MIN_SP * fontScale + LINE_EXTRA_DP

    /**
     * The lines of card a widget [heightDp] dp tall can show whole at
     * [fontScale].
     *
     * A launcher reports no size until it has laid the widget out, and the
     * first update runs before that. Nothing is known then, so nothing is
     * assumed: the answer is the layout's own ceiling, which is what the widget
     * did before any of this.
     */
    fun linesFor(heightDp: Int, fontScale: Double = 1.0): Int {
        if (heightDp <= 0) {
            return MAX_LINES
        }
        val forText = heightDp - 2 * PADDING_DP - LABEL_DP
        return floor(forText / lineDp(fontScale)).toInt().coerceIn(1, MAX_LINES)
    }

    /** The height a box needs to show [lines] lines whole, rounded up to the dp. */
    fun heightFor(lines: Int, fontScale: Double = 1.0): Int =
        kotlin.math.ceil(lines * lineDp(fontScale)).toInt() + 2 * PADDING_DP + LABEL_DP
}
