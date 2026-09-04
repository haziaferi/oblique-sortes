package dev.feridottir.sortes

import android.app.Activity
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
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.Spinner
import android.widget.TextView

/**
 * One screen: pick a deck, draw from it, read what came up.
 *
 * Views are built in code rather than XML because the layout is a single
 * column and the app carries no AndroidX; the only runtime dependency is the
 * Kotlin stdlib.
 */
class MainActivity : Activity() {

    private lateinit var decks: List<Deck>
    private lateinit var cardView: TextView
    private lateinit var blurbView: TextView
    private lateinit var provenanceView: TextView
    private var selected: Int = 0

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

        setContentView(buildUi())
        drawInto(1)
    }

    private fun buildUi(): View {
        val column = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(dp(20), dp(16), dp(20), dp(16))
        }

        column.addView(text("sortes", 30f, bold = true))
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

        column.addView(buttonRow())

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
        val labels = decks.map { "${it.name}  ·  ${it.count} cards" }
        return Spinner(this).apply {
            adapter = ArrayAdapter(this@MainActivity, android.R.layout.simple_spinner_dropdown_item, labels)
            setSelection(0)
            onItemSelectedListener = object : AdapterView.OnItemSelectedListener {
                override fun onItemSelected(parent: AdapterView<*>?, view: View?, position: Int, id: Long) {
                    selected = position
                    drawInto(1)
                }

                override fun onNothingSelected(parent: AdapterView<*>?) = Unit
            }
        }
    }

    private fun buttonRow(): View {
        val row = LinearLayout(this).apply { orientation = LinearLayout.HORIZONTAL }
        listOf(1, 3, 5).forEach { n ->
            val label = if (n == 1) "Draw" else "Draw $n"
            row.addView(
                Button(this).apply {
                    text = label
                    setOnClickListener { drawInto(n) }
                },
                LinearLayout.LayoutParams(0, WRAP_CONTENT, 1f).apply { marginEnd = dp(6) },
            )
        }
        return row
    }

    private fun drawInto(count: Int) {
        val deck = decks[selected]
        blurbView.text = deck.blurb
        provenanceView.text = deck.provenance
        // Cards are shown as they come: a card carrying `\n\t` renders with its
        // continuation lines indented, the way the physical card reads.
        cardView.text = drawCards(deck.id, count).joinToString("\n\n")
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
            if (dim) alpha = 0.65f
        }

    private fun dp(value: Int): Int = (value * resources.displayMetrics.density).toInt()
}
