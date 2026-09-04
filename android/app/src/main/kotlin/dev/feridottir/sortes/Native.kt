package dev.feridottir.sortes

/**
 * The JNI surface over the `sortes` crate.
 *
 * Every call returns one flat string. Cards carry embedded newlines and tabs,
 * so the separators are ASCII control codes that cannot occur in card text;
 * they match the constants in `android/jni/src/lib.rs`, which has tests
 * asserting that no card, name, blurb or provenance line contains either.
 */
internal object Native {
    /** Unit separator, between the fields of one record. */
    const val US: Char = '\u001F'

    /** Record separator, between records. */
    const val RS: Char = '\u001E'

    init {
        System.loadLibrary("sortes_jni")
    }

    external fun decks(): String

    external fun draw(deckId: String, count: Int): String

    external fun shuffled(deckId: String): String

    external fun cardId(deckId: String, card: String): String

    external fun cardById(cardId: String): String
}

/** One deck, as the native side describes it. */
internal data class Deck(
    val id: String,
    val name: String,
    val count: Int,
    val blurb: String,
    val provenance: String,
)

/** A card found by its saved id. */
internal data class FoundCard(val deckId: String, val card: String)

/** The fields one deck record carries, in order. */
private const val DECK_FIELDS = 5

/**
 * Every deck compiled into the native library.
 *
 * A record that does not carry all five fields is dropped rather than indexed
 * into. The Rust side cannot produce one today and a test says so, but the two
 * sides ship as separate artefacts and a mismatched pair should show a short
 * deck list rather than take the app down on its first frame.
 */
internal fun loadDecks(): List<Deck> = parseDecks(Native.decks())

/** The decoding half of [loadDecks], split out so it can be tested off-device. */
internal fun parseDecks(encoded: String): List<Deck> =
    encoded
        .split(Native.RS)
        .filter(String::isNotEmpty)
        .mapNotNull { record ->
            val fields = record.split(Native.US)
            if (fields.size < DECK_FIELDS) {
                null
            } else {
                Deck(
                    id = fields[0],
                    name = fields[1],
                    count = fields[2].toIntOrNull() ?: 0,
                    blurb = fields[3],
                    provenance = fields[4],
                )
            }
        }

/** `count` cards from `deckId`, drawn without replacement within the one call. */
internal fun drawCards(deckId: String, count: Int): List<String> = splitCards(Native.draw(deckId, count))

/** The whole deck in shuffled order: one pass of a [CardShoe]. */
internal fun shuffledDeck(deckId: String): List<String> = splitCards(Native.shuffled(deckId))

/** The splitting half of the draw calls, split out so it can be tested off-device. */
internal fun splitCards(encoded: String): List<String> = encoded.split(Native.RS).filter(String::isNotEmpty)

/** This card's stable id, or `null` if the deck does not hold it. */
internal fun cardIdOf(deckId: String, card: String): String? = Native.cardId(deckId, card).ifEmpty { null }

/**
 * The card a saved id refers to, or `null` if nothing answers to it any more.
 *
 * An id stops resolving when its card is reworded or its deck leaves the build.
 * That is what the id is for, so the caller says so rather than substituting.
 */
internal fun cardById(cardId: String): FoundCard? = parseFoundCard(Native.cardById(cardId))

/** The decoding half of [cardById], split out so it can be tested off-device. */
internal fun parseFoundCard(encoded: String): FoundCard? {
    val separator = encoded.indexOf(Native.US)
    if (separator <= 0 || separator == encoded.length - 1) {
        return null
    }
    return FoundCard(encoded.substring(0, separator), encoded.substring(separator + 1))
}
