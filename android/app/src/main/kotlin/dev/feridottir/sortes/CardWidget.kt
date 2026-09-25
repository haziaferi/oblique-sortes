package dev.feridottir.sortes

import android.app.PendingIntent
import android.appwidget.AppWidgetManager
import android.appwidget.AppWidgetProvider
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.os.Bundle
import android.widget.RemoteViews

/**
 * One card on the home screen. Tap the card and it draws another; tap the deck
 * name beneath it and the app opens, which is the only way from the home screen
 * into it.
 *
 * The widget draws from whichever deck the app was last reading, because it has
 * no screen of its own to ask on. It never redraws by itself: a card that
 * changes while you are looking at it is the opposite of what a card is for,
 * so `updatePeriodMillis` is zero and every redraw comes from a tap.
 */
class CardWidget : AppWidgetProvider() {

    override fun onUpdate(context: Context, manager: AppWidgetManager, widgetIds: IntArray) {
        widgetIds.forEach { id ->
            manager.updateAppWidget(id, build(context, lines(context, manager.getAppWidgetOptions(id)), redraw = true))
        }
    }

    /**
     * A resize is not a tap. The card stays where it is and only the room it
     * has changes, so this re-renders what the widget is already showing rather
     * than dealing a new card for a drag of the handle.
     */
    override fun onAppWidgetOptionsChanged(
        context: Context,
        manager: AppWidgetManager,
        widgetId: Int,
        newOptions: Bundle?,
    ) {
        manager.updateAppWidget(widgetId, build(context, lines(context, newOptions), redraw = false))
    }

    /**
     * The lines the launcher's box can hold.
     *
     * `OPTION_APPWIDGET_MIN_HEIGHT` is the smaller of the heights the host will
     * give the widget across its own orientations, so sizing to it is sizing to
     * the one that shows least. The reader's font scale is the other half of
     * the answer: the box is in dp and the card is in sp.
     */
    private fun lines(context: Context, options: Bundle?): Int =
        WidgetBox.linesFor(
            options?.getInt(AppWidgetManager.OPTION_APPWIDGET_MIN_HEIGHT, 0) ?: 0,
            context.resources.configuration.fontScale.toDouble(),
        )

    override fun onReceive(context: Context, intent: Intent) {
        super.onReceive(context, intent)
        if (intent.action == ACTION_DRAW) {
            val manager = AppWidgetManager.getInstance(context)
            val ids = manager.getAppWidgetIds(ComponentName(context, CardWidget::class.java))
            onUpdate(context, manager, ids)
        }
    }

    /**
     * The widget's views, with a card already on them and [lines] of room for
     * it. A [redraw] deals a new card; otherwise the one already showing is
     * rendered again, and only a widget that has never shown one draws.
     */
    private fun build(context: Context, lines: Int, redraw: Boolean): RemoteViews {
        val views = RemoteViews(context.packageName, R.layout.card_widget)

        // The native library is loaded on first touch of `Native`, which for a
        // widget may be before the app has ever been opened. A device whose
        // .so will not load should show a widget saying so, not crash the
        // launcher's host process.
        val drawn = try {
            if (redraw) drawOne(context) else showing(context) ?: drawOne(context)
        } catch (error: UnsatisfiedLinkError) {
            null
        }

        // What the box can hold, so the ellipsis fires on a card too long for
        // it instead of the text being cut with nothing to show for it.
        views.setInt(R.id.widget_card, "setMaxLines", lines)

        if (drawn == null) {
            views.setTextViewText(R.id.widget_card, context.getString(R.string.widget_unavailable))
            views.setTextViewText(R.id.widget_deck, "")
        } else {
            views.setTextViewText(R.id.widget_card, drawn.card)
            views.setTextViewText(R.id.widget_deck, drawn.deckName)
            views.setContentDescription(R.id.widget_card, drawn.card)
        }

        views.setOnClickPendingIntent(R.id.widget_card, drawIntent(context))
        // The deck name is the way in. A contentDescription replaces the text
        // for a screen reader rather than adding to it, so it has to carry the
        // deck's name as well as what the tap does.
        views.setOnClickPendingIntent(R.id.widget_deck, openIntent(context))
        views.setContentDescription(
            R.id.widget_deck,
            if (drawn == null) context.getString(R.string.widget_open)
            else context.getString(R.string.widget_open_deck, drawn.deckName),
        )
        return views
    }

    /** A card, and the deck it came from. */
    private data class Drawn(val card: String, val deckName: String)

    /**
     * A card, from the deck the app was last reading, that is not the one
     * already on the widget.
     *
     * The widget cannot hold a shoe between updates -- each one is a fresh
     * process, in the launcher's own -- so it asks for two cards and takes the
     * one that is not showing. That covers the repeat anyone would notice: the
     * same card twice in a row from one tap to the next.
     */
    private fun drawOne(context: Context): Drawn? {
        val decks = loadDecks()
        if (decks.isEmpty()) {
            return null
        }
        val deck = rememberedDeck(context, decks) ?: decks.first()
        val showing = widgetCard(context)
        val drawn = drawCards(deck.id, 2)
        val card = drawn.firstOrNull { it != showing } ?: drawn.firstOrNull() ?: return null
        rememberWidgetCard(context, card)
        return Drawn(card, deck.name)
    }

    /** The card the widget is already showing, or `null` if it has none yet. */
    private fun showing(context: Context): Drawn? {
        val card = widgetCard(context) ?: return null
        val decks = loadDecks()
        if (decks.isEmpty()) {
            return null
        }
        return Drawn(card, (rememberedDeck(context, decks) ?: decks.first()).name)
    }

    private fun drawIntent(context: Context): PendingIntent {
        val intent = Intent(context, CardWidget::class.java).setAction(ACTION_DRAW)
        // IMMUTABLE because nothing is ever filled in by the receiver; the flag
        // is required from Android 12 and this app targets well past that.
        return PendingIntent.getBroadcast(context, 0, intent, PendingIntent.FLAG_IMMUTABLE)
    }

    private fun openIntent(context: Context): PendingIntent {
        // A request code of its own. PendingIntents that compare equal by intent
        // and code are one object; keeping the codes apart means the two this
        // widget hands out never can be, whatever the intents come to hold.
        val intent = Intent(context, MainActivity::class.java)
        return PendingIntent.getActivity(context, 1, intent, PendingIntent.FLAG_IMMUTABLE)
    }

    internal companion object {
        /**
         * Sent by the widget to itself when tapped. The manifest's `<receiver>`
         * filter carries the same string, and a test holds the two together:
         * if they drift apart the tap is delivered to nothing and the widget
         * simply stops redrawing, with no error anywhere.
         */
        const val ACTION_DRAW = "dev.feridottir.sortes.DRAW"
    }
}
