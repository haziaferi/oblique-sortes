package dev.feridottir.sortes

import android.content.Context

/**
 * What the app remembers between runs: the deck last read, the card the widget
 * last showed, and the cards that were kept.
 *
 * Kept cards are stored as ids rather than as text. An id stops resolving when
 * its card is reworded or its deck leaves the build, which is the behaviour
 * wanted: the app can say the card is gone instead of quietly showing a
 * different wording under the same star. Storing the text could not tell the
 * difference.
 */
private const val PREFS = "sortes"
private const val KEY_DECK = "deck"
private const val KEY_KEPT = "kept"
private const val KEY_WIDGET_CARD = "widget_card"

/** Remember [deckId] as the deck the app was last reading. */
internal fun rememberDeck(context: Context, deckId: String) {
    context.getSharedPreferences(PREFS, Context.MODE_PRIVATE).edit().putString(KEY_DECK, deckId).apply()
}

/**
 * The remembered deck, or `null` if there is none or it is no longer built in.
 *
 * A deck id can outlive the deck: the preference survives an upgrade that drops
 * a deck feature, and the caller should fall back rather than draw from nothing.
 */
internal fun rememberedDeck(context: Context, available: List<Deck>): Deck? {
    val id = context.getSharedPreferences(PREFS, Context.MODE_PRIVATE).getString(KEY_DECK, null)
    return available.firstOrNull { it.id == id }
}

/**
 * The ids of the cards that were kept, newest first.
 *
 * A `SharedPreferences` string set has no order, so the list is stored as one
 * string separated by a character no id contains — an id is sixteen hex digits.
 */
internal fun keptCards(context: Context): List<String> =
    context.getSharedPreferences(PREFS, Context.MODE_PRIVATE)
        .getString(KEY_KEPT, null)
        .orEmpty()
        .split(KEPT_SEPARATOR)
        .filter(String::isNotEmpty)

/** Whether [cardId] is among the kept cards. */
internal fun isKept(context: Context, cardId: String): Boolean = cardId in keptCards(context)

/**
 * Add [cardId] to the kept cards, or remove it if it is already there.
 *
 * Returns whether the card is kept afterwards, which is what the caller has to
 * say to the reader.
 */
internal fun toggleKept(context: Context, cardId: String): Boolean {
    val kept = keptCards(context).toMutableList()
    val nowKept = if (kept.remove(cardId)) {
        false
    } else {
        // Newest first: the list is read straight into a dialog.
        kept.add(0, cardId)
        true
    }
    context.getSharedPreferences(PREFS, Context.MODE_PRIVATE)
        .edit()
        .putString(KEY_KEPT, kept.joinToString(KEPT_SEPARATOR.toString()))
        .apply()
    return nowKept
}

/** The card the widget last showed, so the next tap does not repeat it. */
internal fun widgetCard(context: Context): String? =
    context.getSharedPreferences(PREFS, Context.MODE_PRIVATE).getString(KEY_WIDGET_CARD, null)

/** Remember what the widget is showing now. */
internal fun rememberWidgetCard(context: Context, card: String) {
    context.getSharedPreferences(PREFS, Context.MODE_PRIVATE).edit().putString(KEY_WIDGET_CARD, card).apply()
}

/** Between kept ids. An id is sixteen hex digits, so this cannot occur in one. */
private const val KEPT_SEPARATOR = ' '
