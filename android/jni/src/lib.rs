//! JNI shim over `sortes`, for the Android app.
//!
//! Two entry points, both returning one flat string. Cards carry embedded
//! newlines and tabs, so the separators here are ASCII control codes that
//! cannot occur in card text: unit separator between fields, record separator
//! between records. Kotlin splits on the same two.
//!
//! The encoding lives in [`encode_decks`] and [`encode_draw`], which are plain
//! functions over the library and are unit-tested below. The `extern` pair
//! around them does nothing but marshal, so the format is testable on the host
//! with no device, no emulator and no NDK.
//!
//! Every entry point catches its own panics. A panic unwinding out of a JNI
//! call is undefined behaviour, and this crate would rather return an empty
//! string than take the app process down.

// The `catch_unwind` calls below are the whole reason a panic here does not
// take the app down, and they catch nothing under `panic = "abort"`. The
// workspace root sets exactly that for `[profile.release]`, and Cargo applies a
// profile to every member, so this is not hypothetical -- it is what a plain
// `cargo build --release -p sortes-jni` did until the `android` profile
// existed. Refusing to compile is the only way to keep the guarantee honest.
#[cfg(panic = "abort")]
compile_error!("sortes-jni catches its own panics, which needs unwinding; build it with `--profile android`");

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::ptr;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint, jstring};

/// Between the fields of one record.
const US: char = '\u{1f}';
/// Between records.
const RS: char = '\u{1e}';

/// Every deck compiled in, one record each:
/// `id US name US count US blurb US provenance`.
fn encode_decks() -> String {
    sortes::decks()
        .iter()
        .map(|deck| {
            format!(
                "{}{US}{}{US}{}{US}{}{US}{}",
                deck.id,
                deck.name,
                deck.count(),
                deck.blurb,
                deck.provenance.describe()
            )
        })
        .collect::<Vec<_>>()
        .join(&RS.to_string())
}

/// `count` cards from `id`, drawn without replacement, separated by RS.
///
/// An unknown deck yields the empty string, which the caller reads as "none".
fn encode_draw(id: &str, count: usize) -> String {
    sortes::deck_by_id(id).map_or_else(String::new, |deck| deck.random_n_str(count).join(&RS.to_string()))
}

/// The whole deck in shuffled order, RS-separated.
///
/// The app deals from this rather than asking for a card at a time, so a run of
/// draws does not repeat until the deck is spent. The cursor lives on the
/// Kotlin side: keeping a `Shoe` alive here would mean global mutable state
/// behind a lock, in a library that has none, for a sequence the caller is
/// already holding.
fn encode_shuffled(id: &str) -> String {
    sortes::deck_by_id(id).map_or_else(String::new, |deck| {
        deck.shoe().draw_n(deck.count()).join(&RS.to_string())
    })
}

/// The id of `card` within `deck_id`, or the empty string if it is not there.
fn encode_card_id(deck_id: &str, card: &str) -> String {
    sortes::deck_by_id(deck_id)
        .and_then(|deck| deck.id_of(card))
        .map_or_else(String::new, |id| id.to_string())
}

/// `deck_id US card` for a saved id, or the empty string.
///
/// An id that no longer resolves is the answer, not a failure: the card was
/// reworded, or its deck is not in this build. The app says so rather than
/// showing something else.
fn encode_card_by_id(id: &str) -> String {
    id.parse()
        .ok()
        .and_then(sortes::card_by_id)
        .map_or_else(String::new, |(deck, card)| format!("{}{US}{card}", deck.id))
}

/// Hand a Rust string to the JVM, or null if the JVM will not take it.
fn into_java(env: &mut JNIEnv, text: &str) -> jstring {
    env.new_string(text).map_or(ptr::null_mut(), |s| s.into_raw())
}

/// See [`encode_decks`].
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_feridottir_sortes_Native_decks(mut env: JNIEnv, _class: JClass) -> jstring {
    let built = catch_unwind(encode_decks).unwrap_or_default();
    into_java(&mut env, &built)
}

/// See [`encode_draw`].
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_feridottir_sortes_Native_draw(
    mut env: JNIEnv,
    _class: JClass,
    deck_id: JString,
    count: jint,
) -> jstring {
    let requested = usize::try_from(count).unwrap_or(0);

    let drawn = catch_unwind(AssertUnwindSafe(|| {
        let id: String = env.get_string(&deck_id).map(Into::into).unwrap_or_default();
        encode_draw(&id, requested)
    }))
    .unwrap_or_default();

    into_java(&mut env, &drawn)
}

/// See [`encode_shuffled`].
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_feridottir_sortes_Native_shuffled(
    mut env: JNIEnv,
    _class: JClass,
    deck_id: JString,
) -> jstring {
    let shuffled = catch_unwind(AssertUnwindSafe(|| {
        let id: String = env.get_string(&deck_id).map(Into::into).unwrap_or_default();
        encode_shuffled(&id)
    }))
    .unwrap_or_default();

    into_java(&mut env, &shuffled)
}

/// See [`encode_card_id`].
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_feridottir_sortes_Native_cardId(
    mut env: JNIEnv,
    _class: JClass,
    deck_id: JString,
    card: JString,
) -> jstring {
    let id = catch_unwind(AssertUnwindSafe(|| {
        let deck: String = env.get_string(&deck_id).map(Into::into).unwrap_or_default();
        let card: String = env.get_string(&card).map(Into::into).unwrap_or_default();
        encode_card_id(&deck, &card)
    }))
    .unwrap_or_default();

    into_java(&mut env, &id)
}

/// See [`encode_card_by_id`].
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_feridottir_sortes_Native_cardById(
    mut env: JNIEnv,
    _class: JClass,
    card_id: JString,
) -> jstring {
    let found = catch_unwind(AssertUnwindSafe(|| {
        let id: String = env.get_string(&card_id).map(Into::into).unwrap_or_default();
        encode_card_by_id(&id)
    }))
    .unwrap_or_default();

    into_java(&mut env, &found)
}

#[cfg(test)]
mod tests {
    use super::{RS, US, encode_card_by_id, encode_card_id, encode_decks, encode_draw, encode_shuffled};

    /// The whole protocol rests on this: a separator that occurs inside a field
    /// would split that field in two on the Kotlin side, silently. Nothing else
    /// enforces it, so this test does.
    #[test]
    fn no_text_the_shim_sends_contains_a_separator() {
        for deck in sortes::decks() {
            for text in [deck.id, deck.name, deck.blurb] {
                assert!(!text.contains(US), "{text:?} contains a unit separator");
                assert!(!text.contains(RS), "{text:?} contains a record separator");
            }
            let provenance = deck.provenance.describe();
            assert!(!provenance.contains(US) && !provenance.contains(RS));
            for card in deck.cards() {
                assert!(!card.contains(US), "card contains a unit separator: {card:?}");
                assert!(!card.contains(RS), "card contains a record separator: {card:?}");
            }
        }
    }

    /// The same decode Kotlin's `loadDecks` performs, so a change to the
    /// encoding that the app could not read fails here first.
    #[test]
    fn decks_decode_to_what_the_library_holds() {
        let encoded = encode_decks();
        let records: Vec<&str> = encoded.split(RS).filter(|record| !record.is_empty()).collect();
        assert_eq!(records.len(), sortes::decks().len());

        for (record, deck) in records.iter().zip(sortes::decks()) {
            let fields: Vec<&str> = record.split(US).collect();
            assert_eq!(fields.len(), 5, "the app reads exactly five fields per deck");
            assert_eq!(fields[0], deck.id);
            assert_eq!(fields[1], deck.name);
            assert_eq!(fields[2].parse::<usize>().expect("the count is a number"), deck.count());
            assert_eq!(fields[3], deck.blurb);
            assert_eq!(fields[4], deck.provenance.describe());
        }
    }

    #[test]
    fn a_draw_decodes_to_cards_of_the_deck_it_named() {
        for deck in sortes::decks() {
            let encoded = encode_draw(deck.id, 5);
            let drawn: Vec<&str> = encoded.split(RS).collect();
            assert_eq!(drawn.len(), 5);
            for card in &drawn {
                assert!(deck.cards().contains(card), "{:?} is not a {} card", card, deck.id);
            }
        }
    }

    #[test]
    fn a_draw_of_nothing_is_the_empty_string() {
        let deck = sortes::decks().first().copied().expect("a deck is always compiled in");
        assert!(encode_draw(deck.id, 0).is_empty());
    }

    /// The app reads an empty string as "no cards"; an unknown id must produce
    /// exactly that rather than a panic across the FFI boundary.
    #[test]
    fn an_unknown_deck_draws_nothing() {
        assert!(encode_draw("no-such-deck", 5).is_empty());
        assert!(encode_draw("", 5).is_empty());
    }

    #[test]
    fn a_draw_larger_than_the_deck_stops_at_the_deck() {
        for deck in sortes::decks() {
            let encoded = encode_draw(deck.id, deck.count() + 100);
            let drawn: Vec<&str> = encoded.split(RS).collect();
            assert_eq!(drawn.len(), deck.count());
        }
    }

    /// A shuffled pass is the whole deck, once each. That is what lets the app
    /// deal without repeating.
    #[test]
    fn a_shuffled_pass_is_the_whole_deck_exactly_once() {
        for deck in sortes::decks() {
            let encoded = encode_shuffled(deck.id);
            let dealt: Vec<&str> = encoded.split(RS).collect();
            assert_eq!(dealt.len(), deck.count());
            let unique: std::collections::HashSet<_> = dealt.iter().collect();
            assert_eq!(unique.len(), deck.count(), "{} dealt a card twice", deck.id);
            for card in &dealt {
                assert!(deck.cards().contains(card));
            }
        }
    }

    #[test]
    fn an_unknown_deck_shuffles_to_nothing() {
        assert!(encode_shuffled("no-such-deck").is_empty());
    }

    /// The two id calls are each other's inverse, for every card in the crate.
    #[test]
    fn card_ids_round_trip_through_the_boundary() {
        for deck in sortes::decks() {
            for card in deck.cards() {
                let id = encode_card_id(deck.id, card);
                assert_eq!(id.len(), 16, "{card:?} produced {id:?}");

                let found = encode_card_by_id(&id);
                let (found_deck, found_card) = found.split_once(US).expect("deck id, then the card");
                assert_eq!(found_deck, deck.id);
                assert_eq!(found_card, *card);
            }
        }
    }

    /// A saved id that no longer resolves is the app's cue to say the card is
    /// gone. It must come back empty rather than as something else.
    #[test]
    fn an_id_that_resolves_to_nothing_is_empty() {
        assert!(encode_card_by_id("0000000000000000").is_empty());
        assert!(encode_card_by_id("not an id").is_empty());
        assert!(encode_card_by_id("").is_empty());
        assert!(encode_card_id("no-such-deck", "whatever").is_empty());
        let deck = sortes::decks().first().copied().expect("a deck is always compiled in");
        assert!(encode_card_id(deck.id, "not a card in this deck").is_empty());
    }

    /// The app splits records apart and then splits fields; a multi-line card
    /// must survive both, tabs intact.
    #[test]
    fn multi_line_cards_cross_the_boundary_whole() {
        let multi_line: Vec<&&str> = sortes::decks()
            .iter()
            .flat_map(|deck| deck.cards().iter())
            .filter(|card| card.contains('\n'))
            .collect();
        assert!(
            !multi_line.is_empty(),
            "the decks hold multi-line cards; this test needs one"
        );

        for card in multi_line {
            let deck = sortes::decks()
                .iter()
                .find(|deck| deck.cards().contains(card))
                .expect("the card came from a deck");
            let encoded = encode_draw(deck.id, deck.count());
            let round_tripped: Vec<&str> = encoded.split(RS).collect();
            assert!(
                round_tripped.contains(card),
                "a multi-line card did not survive encoding: {card:?}"
            );
        }
    }
}
