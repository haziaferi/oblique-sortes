package dev.feridottir.sortes

/**
 * A shuffled deck and a cursor into it, so a run of draws does not repeat.
 *
 * The library has a `Shoe` of its own, but keeping one alive across JNI calls
 * would mean global mutable state behind a lock in a crate that has none. The
 * native side hands over a whole shuffled pass instead, and the cursor lives
 * here — in the component that already has state and already saves it across a
 * rotation.
 *
 * When a pass runs out the shoe asks for another. The card that ended one pass
 * never opens the next, which is the one repeat a shoe exists to prevent and
 * the only one a fresh shuffle could still produce.
 */
internal class CardShoe(val deckId: String, private val shuffle: (String) -> List<String>) {

    private var order: List<String> = emptyList()
    private var next: Int = 0

    /**
     * Deal the next card, or `null` if the deck turns out to be empty.
     *
     * Empty is not expected — every deck compiled in has cards — but the order
     * comes across a process boundary, and a shoe that cannot deal should say
     * so rather than throw.
     */
    fun draw(): String? {
        if (next >= order.size) {
            refill()
        }
        if (order.isEmpty()) {
            return null
        }
        return order[next++]
    }

    /** Deal up to [count] cards, crossing a reshuffle if it has to. */
    fun draw(count: Int): List<String> = (0 until count).mapNotNull { draw() }

    /**
     * Restore a cursor saved across a rotation, keeping the pass it belongs to.
     *
     * The cursor is clamped to the pass it arrives with. A `Bundle` survives the
     * process being killed and restarted, so the pair can come back from a build
     * that dealt a different deck; a cursor past the end deals a fresh pass
     * rather than nothing, and one below the start deals the pass entire.
     */
    fun restore(order: List<String>, next: Int) {
        this.order = order
        this.next = next.coerceIn(0, order.size)
    }

    /** The pass this shoe is dealing from, for saving across a rotation. */
    fun pass(): List<String> = order

    /** How far into that pass it has got, for saving alongside [pass]. */
    fun cursor(): Int = next

    private fun refill() {
        val last = order.lastOrNull()
        var fresh = shuffle(deckId)
        // A card ending one pass and opening the next reads as a repeat, which
        // is the whole thing a shoe is for. One reshuffle is enough to try; if
        // the deck holds a single card there is nothing to be done about it.
        if (fresh.size > 1 && fresh.firstOrNull() == last) {
            fresh = fresh.drop(1) + fresh.first()
        }
        order = fresh
        next = 0
    }
}
