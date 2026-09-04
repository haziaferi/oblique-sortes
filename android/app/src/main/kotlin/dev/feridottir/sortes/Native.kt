package dev.feridottir.sortes

/**
 * The JNI surface over the `sortes` crate.
 *
 * Both calls return one flat string. Cards carry embedded newlines and tabs,
 * so the separators are ASCII control codes that cannot occur in card text;
 * they match the constants in `rust/src/lib.rs`.
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
}

/** One deck, as the native side describes it. */
internal data class Deck(
    val id: String,
    val name: String,
    val count: Int,
    val blurb: String,
    val provenance: String,
)

/** Every deck compiled into the native library. */
internal fun loadDecks(): List<Deck> =
    Native.decks()
        .split(Native.RS)
        .filter(String::isNotEmpty)
        .map { record ->
            val fields = record.split(Native.US)
            Deck(
                id = fields[0],
                name = fields[1],
                count = fields[2].toIntOrNull() ?: 0,
                blurb = fields[3],
                provenance = fields[4],
            )
        }

/** `count` cards from `deckId`, drawn without replacement. */
internal fun drawCards(deckId: String, count: Int): List<String> =
    Native.draw(deckId, count)
        .split(Native.RS)
        .filter(String::isNotEmpty)
