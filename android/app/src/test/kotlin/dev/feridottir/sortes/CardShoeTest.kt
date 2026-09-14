package dev.feridottir.sortes

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

/**
 * The app's shoe, tested against a stand-in shuffler rather than the native one.
 *
 * The shoe takes its shuffle as a function precisely so this is possible: no
 * device, no native library, and a shuffle whose output the test chooses, which
 * is the only way to pin down what happens at a pass boundary.
 */
class CardShoeTest {

    private val deck = (1..8).map { "card $it" }

    /** Deals the deck in a fixed order, so the seam is testable rather than lucky. */
    private fun fixedShoe(order: List<String> = deck) = CardShoe("test") { order }

    @Test
    fun `a pass deals every card exactly once`() {
        val dealt = fixedShoe().draw(deck.size)

        assertEquals(deck.size, dealt.size)
        assertEquals(deck.toSet(), dealt.toSet())
    }

    @Test
    fun `the shoe refills rather than running out`() {
        assertEquals(deck.size * 3, fixedShoe().draw(deck.size * 3).size)
    }

    /**
     * The one repeat a shoe exists to prevent, and the only one a fresh shuffle
     * could still produce: the card that ended one pass opening the next.
     */
    @Test
    fun `no card repeats across the seam`() {
        // Every pass comes back in the same order, which is the worst case: a
        // shoe that just reshuffled would deal card 1 straight after card 8.
        val dealt = fixedShoe().draw(deck.size * 3)

        dealt.zipWithNext().forEach { (before, after) ->
            assertNotEquals("the same card was dealt twice running", before, after)
        }
    }

    /**
     * The cursor is half of what `onSaveInstanceState` writes down, so where it
     * sits after a pass is spent is the thing a rotation depends on.
     */
    @Test
    fun `the cursor counts up and wraps`() {
        val shoe = fixedShoe()
        assertEquals("a shoe has no pass until its first draw", 0, shoe.cursor())

        shoe.draw()
        assertEquals(1, shoe.cursor())

        shoe.draw(deck.size - 1)
        assertEquals("a spent pass leaves the cursor at its end", deck.size, shoe.cursor())

        shoe.draw()
        assertEquals("a fresh pass starts the cursor over", 1, shoe.cursor())
    }

    @Test
    fun `an empty deck deals nothing rather than throwing`() {
        val shoe = CardShoe("empty") { emptyList() }

        assertNull(shoe.draw())
        assertTrue(shoe.draw(3).isEmpty())
    }

    @Test
    fun `a single card deck can only repeat`() {
        val shoe = CardShoe("one") { listOf("the only card") }

        assertEquals(listOf("the only card", "the only card"), shoe.draw(2))
    }

    /**
     * A rotation rebuilds the Activity. Restoring the pass and the cursor is
     * what stops the deck starting over and handing back cards already seen.
     */
    @Test
    fun `a restored shoe carries on where it left off`() {
        val first = fixedShoe()
        val before = first.draw(3)

        val restored = fixedShoe()
        restored.restore(first.pass(), first.cursor())

        assertEquals("the restored shoe holds the pass it was given", first.pass(), restored.pass())
        assertEquals(3, restored.cursor())
        val after = restored.draw(deck.size - 3)
        assertEquals("the pass was not completed exactly once", deck.toSet(), (before + after).toSet())
    }

    @Test
    fun `a cursor past the end of its pass is clamped`() {
        val shoe = fixedShoe()
        shoe.restore(deck, deck.size + 99)

        assertEquals("the cursor is clamped to the end of the pass", deck.size, shoe.cursor())
        assertEquals("it should refill rather than deal nothing", deck.size, shoe.draw(deck.size).size)
    }

    @Test
    fun `a negative cursor is clamped`() {
        val shoe = fixedShoe()
        shoe.restore(deck, -5)

        assertEquals("the cursor is clamped to the start of the pass", 0, shoe.cursor())
        assertEquals("the whole pass is still there to deal", deck.toSet(), shoe.draw(deck.size).toSet())
    }
}
