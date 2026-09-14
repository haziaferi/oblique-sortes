package dev.feridottir.sortes

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

/**
 * The decoding half of the JNI boundary, tested on the JVM with no device.
 *
 * The Rust side has tests proving it produces this shape; these prove the app
 * reads that shape back, and — more to the point — that it does something
 * sensible with a shape it was not promised. The two sides ship as separate
 * artefacts, so "the encoder cannot produce that" is not the same as "the
 * decoder will never see it".
 *
 * Nothing here loads the native library: every function under test takes the
 * already-encoded string, which is why `Native.kt` splits the calls in two.
 */
class DecodingTest {

    private val us = Native.US
    private val rs = Native.RS

    private fun record(vararg fields: String) = fields.joinToString(us.toString())

    @Test
    fun `a well formed deck list decodes field for field`() {
        val encoded = listOf(
            record("oblique", "Oblique Strategies", "156", "Lateral nudges.", "Text by Eno."),
            record("examen", "Examen", "92", "End of a day.", "Written for this crate."),
        ).joinToString(rs.toString())

        val decks = parseDecks(encoded)

        assertEquals(2, decks.size)
        assertEquals(Deck("oblique", "Oblique Strategies", 156, "Lateral nudges.", "Text by Eno."), decks[0])
        assertEquals("examen", decks[1].id)
        assertEquals(92, decks[1].count)
    }

    @Test
    fun `an empty deck list is no decks, not one empty deck`() {
        assertEquals(emptyList<Deck>(), parseDecks(""))
    }

    /** The app would rather show a short list than die on its first frame. */
    @Test
    fun `a record short of its five fields is dropped, not indexed into`() {
        val encoded = listOf(
            record("oblique", "Oblique Strategies", "156", "Lateral nudges.", "Text by Eno."),
            record("truncated", "Truncated", "12"),
        ).joinToString(rs.toString())

        val decks = parseDecks(encoded)

        assertEquals(1, decks.size)
        assertEquals("oblique", decks[0].id)
    }

    /** A count that is not a number is a zero, not a crash. */
    @Test
    fun `an unreadable card count becomes zero`() {
        val decks = parseDecks(record("odd", "Odd", "not a number", "Blurb.", "Provenance."))
        assertEquals(1, decks.size)
        assertEquals(0, decks[0].count)
    }

    /** Cards carry newlines and tabs; only the separators may split them. */
    @Test
    fun `a multi line card survives being split`() {
        val card = "Destroy\n\t-nothing\n\t-the most important thing"
        val cards = splitCards(listOf(card, "Another card").joinToString(rs.toString()))

        assertEquals(2, cards.size)
        assertEquals(card, cards[0])
        assertTrue(cards[0].contains('\n'))
        assertTrue(cards[0].contains('\t'))
    }

    @Test
    fun `no cards is an empty list`() {
        assertEquals(emptyList<String>(), splitCards(""))
    }

    @Test
    fun `a found card splits into its deck and its text`() {
        val found = parseFoundCard("oblique" + us + "Honor thy error as a hidden intention")

        assertEquals("oblique", found?.deckId)
        assertEquals("Honor thy error as a hidden intention", found?.card)
    }

    /** A card whose own text contains no separator, but has one after it. */
    @Test
    fun `only the first separator divides the record`() {
        val found = parseFoundCard("deck" + us + "a card" + us + "with more after it")

        assertEquals("deck", found?.deckId)
        assertEquals("a card" + us + "with more after it", found?.card)
    }

    /**
     * An id that no longer resolves comes back empty, and that has to read as
     * "gone" rather than as a card with no text.
     */
    @Test
    fun `an unresolved id is null`() {
        assertNull(parseFoundCard(""))
    }

    @Test
    fun `a malformed found card is null rather than half a card`() {
        assertNull(parseFoundCard("no separator at all"))
        assertNull(parseFoundCard(us + "no deck id"))
        assertNull(parseFoundCard("no card text" + us))
    }

    // ---- searching, which is also how the deck is read whole ----------------
    //
    // A pass arrives from the native side shuffled, so these hand it one in an
    // order no answer should depend on.
    private val pass = listOf("Water", "Ghost echoes", "Accretion", "Repetition is a form of change")

    /**
     * The rule the crate documents and `tests/cli.rs` enforces: an empty needle
     * matches every card. The app answered "Nothing matched" to that input until
     * this test existed, which made it the one place the app contradicted the
     * library.
     */
    @Test
    fun `an empty needle matches every card`() {
        assertEquals(pass.sorted(), cardsMatching(pass, ""))
    }

    /** A box holding only spaces is an empty box, whatever the keyboard put in it. */
    @Test
    fun `a blank needle matches every card`() {
        assertEquals(pass.sorted(), cardsMatching(pass, "   "))
    }

    @Test
    fun `matching ignores case`() {
        assertEquals(listOf("Ghost echoes"), cardsMatching(pass, "GHOST"))
        assertEquals(listOf("Ghost echoes"), cardsMatching(pass, "ghost"))
    }

    /**
     * The pass comes across shuffled and the deck has an order, so matches are
     * sorted back into it rather than shown as they were dealt.
     */
    @Test
    fun `matches come back in the deck's order, not the pass's`() {
        assertEquals(listOf("Accretion", "Ghost echoes", "Repetition is a form of change", "Water"),
                     cardsMatching(pass, ""))
    }

    /**
     * A multi-line card sorts by its first line, because the newline that opens
     * its continuation is below every character a first line can end on. That is
     * what lets ordinary string order stand in for the deck's own.
     */
    @Test
    fun `a multi-line card sorts by its first line`() {
        val cards = listOf("Think of the radio", "Think\n\t-inside the work", "Water")
        assertEquals(listOf("Think\n\t-inside the work", "Think of the radio", "Water"),
                     cardsMatching(cards, ""))
    }

    @Test
    fun `a needle nothing carries matches nothing`() {
        assertTrue(cardsMatching(pass, "no card says this and none ever will").isEmpty())
    }
}
