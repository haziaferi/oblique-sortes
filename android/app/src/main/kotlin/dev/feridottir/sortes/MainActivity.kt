package dev.feridottir.sortes

import android.app.Activity
import android.app.AlertDialog
import android.content.ActivityNotFoundException
import android.content.ClipData
import android.content.ClipboardManager
import android.content.Intent
import android.graphics.Typeface
import android.os.Bundle
import android.util.TypedValue
import android.view.Gravity
import android.view.View
import android.view.ViewGroup.LayoutParams.MATCH_PARENT
import android.view.ViewGroup.LayoutParams.WRAP_CONTENT
import android.view.WindowInsets
import android.widget.AdapterView
import android.widget.ArrayAdapter
import android.widget.Button
import android.widget.EditText
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.Spinner
import android.widget.TextView
import android.widget.Toast

/**
 * One screen: pick a deck, draw from it, read what came up.
 *
 * Views are built in code rather than XML because the layout is a single
 * column and the app carries no AndroidX; the only runtime dependency is the
 * Kotlin stdlib.
 */
class MainActivity : Activity() {

    private companion object {
        /** Which deck was showing. */
        const val STATE_DECK = "deck"

        /** What was on the card. A draw is a one-off; losing it loses the card. */
        const val STATE_CARD = "card"

        /** The shuffled pass the shoe is dealing from, and how far into it. */
        const val STATE_PASS = "pass"
        const val STATE_CURSOR = "cursor"

        /** Anything smaller is below the platform minimum touch target. */
        const val TOUCH_TARGET_DP = 48

        /** Secondary text, dimmed enough to recede and no further. */
        const val DIM = 0.72f

        /** Middle dot, between a deck name and its card count. */
        const val DOT = "·"
    }

    private lateinit var decks: List<Deck>
    private lateinit var cardView: TextView
    private lateinit var blurbView: TextView
    private lateinit var provenanceView: TextView

    /** Its label says whether the card on screen is kept, so it is held onto. */
    private lateinit var keepButton: Button

    private var selected: Int = 0

    /** What the card view is showing, kept so it can be shared and restored. */
    private var current: String = ""

    /**
     * The deck the cards come from, shuffled, with a cursor into it.
     *
     * Rebuilt whenever the deck changes: a shoe belongs to one deck, and
     * carrying a cursor across decks would deal from the wrong pass.
     */
    private var shoe: CardShoe? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // The screen carries its own title; the platform one would say it twice.
        actionBar?.hide()

        decks = try {
            loadDecks()
        } catch (error: UnsatisfiedLinkError) {
            setContentView(failureView("The native library did not load.\n\n$error"))
            return
        }

        if (decks.isEmpty()) {
            setContentView(failureView("The native library loaded but reports no decks."))
            return
        }

        // A rotation, a night-mode switch or a trip through the background all
        // rebuild this Activity. A drawn card is a one-off and cannot be drawn
        // again, so it is restored rather than replaced.
        savedInstanceState?.let { saved ->
            selected = saved.getInt(STATE_DECK, 0).coerceIn(decks.indices)
            current = saved.getString(STATE_CARD).orEmpty()
        }

        setContentView(buildUi())

        // The pass is restored along with the card, so a rotation does not
        // start the deck over and hand back cards already seen.
        val pass = savedInstanceState?.getStringArrayList(STATE_PASS)
        if (pass != null) {
            shoeFor(decks[selected]).restore(pass, savedInstanceState.getInt(STATE_CURSOR, 0))
        }

        if (current.isEmpty()) draw(1) else show(current)
    }

    override fun onPause() {
        super.onPause()
        // The widget has no screen to ask on, so it draws from whichever deck
        // was last read here. Written on the way out rather than on every pick.
        if (::decks.isInitialized && decks.isNotEmpty()) {
            rememberDeck(this, decks[selected].id)
        }
    }

    override fun onSaveInstanceState(outState: Bundle) {
        super.onSaveInstanceState(outState)
        outState.putInt(STATE_DECK, selected)
        outState.putString(STATE_CARD, current)
        shoe?.let { open ->
            outState.putStringArrayList(STATE_PASS, ArrayList(open.pass()))
            outState.putInt(STATE_CURSOR, open.cursor())
        }
    }

    /** The shoe for [deck], rebuilt if the deck has changed since the last draw. */
    private fun shoeFor(deck: Deck): CardShoe =
        shoe?.takeIf { it.deckId == deck.id } ?: CardShoe(deck.id, ::shuffledDeck).also { shoe = it }

    private fun buildUi(): View {
        val column = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(dp(20), dp(16), dp(20), dp(16))
        }

        column.addView(text("sortes", 30f, bold = true).apply { isAccessibilityHeading = true })
        column.addView(
            text("Cards for when the work will not move.", 14f, dim = true).apply {
                setPadding(0, dp(2), 0, dp(16))
            },
        )
        column.addView(deckSpinner())

        blurbView = text("", 13f, dim = true).apply { setPadding(0, dp(10), 0, 0) }
        column.addView(blurbView)

        cardView = text("", 20f).apply {
            typeface = Typeface.MONOSPACE
            setLineSpacing(dp(5).toFloat(), 1f)
            setTextIsSelectable(true)
            gravity = Gravity.CENTER_VERTICAL
            // A draw replaces the card in place, with no navigation to notice.
            // Announcing it is the only way a screen reader hears the new card.
            accessibilityLiveRegion = View.ACCESSIBILITY_LIVE_REGION_POLITE
            // The button row holds all it can, and long-press is where a copy
            // lives on Android anyway. Selection copy still works alongside it.
            setOnLongClickListener {
                copy()
                true
            }
        }
        // The card takes whatever room is left, so the buttons sit at the foot
        // of the screen instead of floating in the middle of it. It scrolls on
        // its own for the long cards and for a five-card draw.
        column.addView(
            ScrollView(this).apply {
                addView(cardView, LinearLayout.LayoutParams(MATCH_PARENT, WRAP_CONTENT))
                isFillViewport = true
                setPadding(0, dp(20), 0, dp(20))
            },
            LinearLayout.LayoutParams(MATCH_PARENT, 0, 1f),
        )

        column.addView(buttonRows())

        provenanceView = text("", 11f, dim = true).apply { setPadding(0, dp(16), 0, 0) }
        column.addView(provenanceView)

        // targetSdk 35+ makes the app edge-to-edge whether it asks or not, so
        // the window no longer insets itself. Pad by the system bars, or the
        // title sits under the status bar and the buttons under the gestures.
        column.setOnApplyWindowInsetsListener { view, insets ->
            val bars = insets.getInsets(WindowInsets.Type.systemBars() or WindowInsets.Type.displayCutout())
            view.setPadding(bars.left + dp(20), bars.top + dp(16), bars.right + dp(20), bars.bottom + dp(16))
            insets
        }
        return column
    }

    private fun deckSpinner(): Spinner {
        val labels = decks.map { "${it.name}  ${DOT}  ${it.count} cards" }
        return Spinner(this).apply {
            adapter = ArrayAdapter(this@MainActivity, android.R.layout.simple_spinner_dropdown_item, labels)
            contentDescription = getString(R.string.deck_label)
            minimumHeight = dp(TOUCH_TARGET_DP)
            setSelection(selected)
            onItemSelectedListener = object : AdapterView.OnItemSelectedListener {
                override fun onItemSelected(parent: AdapterView<*>?, view: View?, position: Int, id: Long) {
                    // A Spinner reports the selection set during layout as
                    // though the user had made it. That fire always matches the
                    // index already held, and a real pick never does -- so this
                    // returns before the card views exist, and before a card
                    // restored across a rotation could be drawn over.
                    if (position == selected) return
                    selected = position
                    draw(1)
                }

                override fun onNothingSelected(parent: AdapterView<*>?) = Unit
            }
        }
    }

    /**
     * Two rows: drawing on top, what to do with what was drawn beneath.
     *
     * Four buttons is what fits across a phone at this text size, which is why
     * copying is a long-press on the card rather than a fifth.
     */
    private fun buttonRows(): View {
        val rows = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL }
        rows.addView(
            row(
                getString(R.string.draw) to { draw(1) },
                getString(R.string.draw_n, 3) to { draw(3) },
                getString(R.string.draw_n, 5) to { draw(5) },
            ),
        )
        keepButton = Button(this).apply {
            minimumHeight = dp(TOUCH_TARGET_DP)
            setOnClickListener { keep() }
        }
        rows.addView(
            row(
                getString(R.string.share) to ::share,
                getString(R.string.find) to ::find,
                getString(R.string.saved) to ::showSaved,
                existing = keepButton,
            ),
        )
        return rows
    }

    /** One row of equally-weighted buttons. */
    private fun row(vararg actions: Pair<String, () -> Unit>, existing: Button? = null): View {
        val row = LinearLayout(this).apply { orientation = LinearLayout.HORIZONTAL }
        val weighted = { LinearLayout.LayoutParams(0, WRAP_CONTENT, 1f).apply { marginEnd = dp(6) } }
        actions.forEach { (label, action) ->
            row.addView(
                Button(this).apply {
                    text = label
                    minimumHeight = dp(TOUCH_TARGET_DP)
                    minimumWidth = dp(TOUCH_TARGET_DP)
                    setOnClickListener { action() }
                },
                weighted(),
            )
        }
        existing?.let { row.addView(it, weighted()) }
        return row
    }

    /**
     * Deal [count] cards and show them.
     *
     * Through the shoe rather than a fresh draw each time, so tapping Draw
     * repeatedly works through the deck instead of offering the same card
     * again a few taps later.
     */
    private fun draw(count: Int) =
        show(shoeFor(decks[selected]).draw(count).joinToString("\n\n"))

    /**
     * Put [cards] on screen and remember them.
     *
     * Cards are shown as they come: one carrying a newline and tab renders with
     * its continuation lines indented, the way the physical card reads.
     */
    private fun show(cards: String) {
        current = cards
        val deck = decks[selected]
        blurbView.text = deck.blurb
        provenanceView.text = deck.provenance
        cardView.text = cards

        // The label is the only thing that says whether this card is kept, so
        // it has to follow the card rather than the last tap.
        val id = currentCardId()
        keepButton.isEnabled = id != null
        keepButton.text = getString(if (id != null && isKept(this, id)) R.string.kept else R.string.keep)
    }

    /** Hand the card to whatever the device can send text with. */
    private fun share() {
        if (current.isEmpty()) return
        val send = Intent(Intent.ACTION_SEND).apply {
            type = "text/plain"
            putExtra(Intent.EXTRA_TEXT, current)
        }
        try {
            startActivity(Intent.createChooser(send, getString(R.string.share_chooser)))
        } catch (_: ActivityNotFoundException) {
            // A device with nothing that accepts text is unusual but possible,
            // and is not a reason to take the app down.
            Toast.makeText(this, R.string.share_unavailable, Toast.LENGTH_SHORT).show()
        }
    }

    /**
     * Keep the card on screen, or stop keeping it.
     *
     * Only a single card can be kept: a multi-card draw is several cards under
     * one id, and there would be no honest id to save for it.
     */
    private fun keep() {
        val id = currentCardId() ?: return
        val nowKept = toggleKept(this, id)
        keepButton.text = getString(if (nowKept) R.string.kept else R.string.keep)
        Toast.makeText(this, if (nowKept) R.string.kept_toast else R.string.unkept_toast, Toast.LENGTH_SHORT).show()
    }

    /**
     * The id of the card on screen, or `null` if what is showing is not one
     * card of the current deck.
     */
    private fun currentCardId(): String? =
        current.takeIf { it.isNotEmpty() }?.let { card -> cardIdOf(decks[selected].id, card) }

    /**
     * Search the current deck, and offer what matches.
     *
     * An empty box matches every card, so this is also how the deck is read
     * whole — the app's answer to the command line's `--all`. There is no fifth
     * button for it because the search already does it, and because the row
     * holds four.
     */
    private fun find() {
        val deck = decks[selected]
        val input = EditText(this).apply {
            hint = getString(R.string.find_hint)
            setSingleLine()
        }
        AlertDialog.Builder(this)
            .setTitle(getString(R.string.find_in, deck.name))
            .setView(input)
            .setPositiveButton(R.string.find) { _, _ ->
                val matches = deck.matching(input.text.toString())
                if (matches.isEmpty()) {
                    Toast.makeText(this, R.string.no_matches, Toast.LENGTH_SHORT).show()
                } else {
                    offer(resources.getQuantityString(R.plurals.matches, matches.size, matches.size), matches)
                }
            }
            .setNegativeButton(android.R.string.cancel, null)
            .show()
    }

    /**
     * Every card of this deck containing [needle], ignoring case.
     *
     * Filtered in Kotlin rather than behind a sixth native call: the app already
     * has a way to ask for every card of a deck, and searching a list it holds
     * costs one JNI crossing instead of a new one to maintain. The matching
     * itself is [cardsMatching], which the JVM tests reach and this Activity
     * does not.
     */
    private fun Deck.matching(needle: String): List<String> = cardsMatching(shuffledDeck(id), needle)

    /**
     * The cards that were kept, resolved through their ids.
     *
     * An id that no longer answers is shown as such rather than skipped: the
     * card was reworded or its deck left the build, and quietly dropping it
     * would look like the app had lost it.
     *
     * Kept cards are held in one list across every deck, so each is labelled
     * with the deck it came from. The card alone does not say, and two decks
     * cover neighbouring ground on purpose.
     */
    private fun showSaved() {
        val kept = keptCards(this)
        if (kept.isEmpty()) {
            Toast.makeText(this, R.string.saved_empty, Toast.LENGTH_SHORT).show()
            return
        }
        val found = kept.map(::cardById)
        val cards = found.map { it?.card ?: getString(R.string.card_gone) }
        val labels = found.zip(cards) { card, text ->
            val deck = card?.deckId?.let(::deckNamed)
            if (deck == null) text else getString(R.string.saved_entry, text, deck)
        }
        offer(getString(R.string.saved), cards, labels)
    }

    /** The name of the deck with this id, or `null` if this build has no such deck. */
    private fun deckNamed(deckId: String): String? = decks.firstOrNull { it.id == deckId }?.name

    /**
     * Show a list of cards; picking one puts it on the card view.
     *
     * [labels] is what the list shows and [cards] is what a pick puts on the
     * screen. The two differ for kept cards, where the deck is worth naming and
     * is not part of the card.
     */
    private fun offer(title: String, cards: List<String>, labels: List<String> = cards) {
        AlertDialog.Builder(this)
            .setTitle(title)
            .setItems(labels.toTypedArray()) { _, which -> show(cards[which]) }
            .setNegativeButton(android.R.string.cancel, null)
            .show()
    }

    /** Put the card on the clipboard, and say so. */
    private fun copy() {
        if (current.isEmpty()) return
        val clipboard = getSystemService(CLIPBOARD_SERVICE) as? ClipboardManager ?: return
        clipboard.setPrimaryClip(ClipData.newPlainText(getString(R.string.app_name), current))
        // Android 13 and up shows its own copy confirmation; a second one would
        // say it twice.
        if (android.os.Build.VERSION.SDK_INT < android.os.Build.VERSION_CODES.TIRAMISU) {
            Toast.makeText(this, R.string.copied, Toast.LENGTH_SHORT).show()
        }
    }

    private fun failureView(message: String): View =
        text(message, 16f).apply {
            gravity = Gravity.CENTER
            setPadding(dp(24), dp(64), dp(24), dp(24))
        }

    private fun text(value: String, sizeSp: Float, bold: Boolean = false, dim: Boolean = false): TextView =
        TextView(this).apply {
            text = value
            setTextSize(TypedValue.COMPLEX_UNIT_SP, sizeSp)
            if (bold) setTypeface(typeface, Typeface.BOLD)
            if (dim) alpha = DIM
        }

    private fun dp(value: Int): Int = (value * resources.displayMetrics.density).toInt()
}
