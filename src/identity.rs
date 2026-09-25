//! A stable handle for one card.
//!
//! `(deck_id, index)` was the only handle this crate had, and an index moves
//! whenever a card is added or removed above it — so anything that remembered a
//! card would silently come back pointing at a different one. Giving every card
//! a written id would mean editing all eight decks and keeping the ids unique
//! by hand, which is a promise the decks would eventually break.
//!
//! So an id is derived instead, from the deck's id and the card's own text. It
//! survives cards being added, removed or reordered around it, and it changes
//! when the card itself is reworded — which is the honest answer, because a
//! reworded card is a different card, and something that saved the old wording
//! should notice rather than quietly show the new one.

use std::fmt;
use std::str::FromStr;

/// The FNV-1a 64-bit offset basis and prime.
///
/// The prime is 0x100000001b3, which is eleven hex digits and so does not group
/// into fours. Written `0x1000_0000_01b3` it gains a zero and stops being the
/// FNV prime, while still hashing well enough that nothing would notice.
const OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const PRIME: u64 = 0x100_0000_01b3;

/// A stable handle for one card, derived from its deck and its text.
///
/// Two cards never share an id — the crate's tests check that across every deck
/// compiled in — and an id is the same in every build of the same card text.
///
/// It is not a checksum of the deck and it is not an index. Rewording a card
/// changes its id; adding, removing or reordering other cards does not.
///
/// # Examples
///
/// ```
/// use sortes::CardId;
///
/// let deck = sortes::decks().first().expect("a deck is always compiled in");
/// let id = deck.id_of(deck.cards()[0]).expect("the card is in the deck");
///
/// // Sixteen hex digits, and it reads back.
/// let written = id.to_string();
/// assert_eq!(written.len(), 16);
/// assert_eq!(written.parse::<CardId>(), Ok(id));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CardId(u64);

impl CardId {
    /// The id of `card` in the deck called `deck_id`.
    ///
    /// FNV-1a over the deck id, a zero byte, then the card. The zero byte is
    /// the separator: without it `("ab", "c")` and `("a", "bc")` would hash
    /// alike, and no card or deck id contains a zero byte.
    pub(crate) const fn of(deck_id: &str, card: &str) -> Self {
        let hash = Self::eat(OFFSET_BASIS, deck_id.as_bytes());
        let hash = Self::eat(hash, &[0]);
        Self(Self::eat(hash, card.as_bytes()))
    }

    /// FNV-1a over `bytes`. A loop rather than an iterator, so this stays
    /// const.
    const fn eat(mut hash: u64, bytes: &[u8]) -> u64 {
        let mut index = 0;
        while index < bytes.len() {
            hash ^= bytes[index] as u64;
            hash = hash.wrapping_mul(PRIME);
            index += 1;
        }
        hash
    }
}

impl fmt::Display for CardId {
    /// Sixteen lowercase hex digits, zero-padded, so ids sort and align.
    ///
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// let id = deck.id_of(deck.cards()[0]).expect("the card is in the deck");
    /// assert_eq!(id.to_string().len(), 16);
    /// ```
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:016x}", self.0)
    }
}

/// Why a written id could not be read back.
///
/// # Examples
///
/// ```
/// use sortes::CardId;
/// assert!("not an id".parse::<CardId>().is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseCardIdError;

impl fmt::Display for ParseCardIdError {
    /// # Examples
    ///
    /// ```
    /// use sortes::CardId;
    /// let error = "zz".parse::<CardId>().expect_err("not an id");
    /// assert!(error.to_string().contains("hex"));
    /// ```
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a card id is sixteen hex digits")
    }
}

impl std::error::Error for ParseCardIdError {}

impl FromStr for CardId {
    type Err = ParseCardIdError;

    /// Read back what [`Display`](fmt::Display) wrote.
    ///
    /// Strict about the length: a shorter string would parse as a different id
    /// and quietly find the wrong card, or more likely none at all.
    ///
    /// # Examples
    ///
    /// ```
    /// use sortes::CardId;
    ///
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// let id = deck.id_of(deck.cards()[0]).expect("the card is in the deck");
    /// assert_eq!(id.to_string().parse::<CardId>(), Ok(id));
    /// assert!("0123".parse::<CardId>().is_err());
    /// ```
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.len() != 16 {
            return Err(ParseCardIdError);
        }
        u64::from_str_radix(text, 16).map(Self).map_err(|_| ParseCardIdError)
    }
}

#[cfg(test)]
mod tests {
    use super::CardId;

    /// The one guarantee everything else rests on. A collision would hand two
    /// different cards the same handle, and whatever saved one would show the
    /// other.
    #[test]
    fn every_card_in_the_crate_has_its_own_id() {
        let mut seen: std::collections::HashMap<CardId, (&str, &str)> = std::collections::HashMap::new();
        for deck in crate::decks() {
            for card in deck.cards() {
                let id = CardId::of(deck.id, card);
                if let Some((other_deck, other_card)) = seen.insert(id, (deck.id, card)) {
                    panic!("{id} is both {other_deck}/{other_card:?} and {}/{card:?}", deck.id);
                }
            }
        }
        assert_eq!(
            seen.len(),
            crate::decks().iter().map(|deck| deck.count()).sum::<usize>()
        );
    }

    /// Pins the algorithm. Ids are written down by anything that saves a card,
    /// so changing how they are derived silently invalidates every one of them.
    /// If this test fails, the change is not a refactor.
    #[cfg(feature = "oblique")]
    #[test]
    fn the_derivation_is_pinned() {
        let deck = crate::deck_by_id("oblique").expect("compiled in");
        let card = "Honor thy error as a hidden intention";
        assert!(deck.cards().contains(&card), "the pinned card left the deck");
        assert_eq!(
            deck.id_of(card).expect("the card is in the deck").to_string(),
            "5c66739aa6ee7004",
        );
    }

    /// The deck is part of the id, so the same words in two decks are two
    /// cards.
    #[test]
    fn the_deck_is_part_of_the_id() {
        assert_ne!(CardId::of("one", "same words"), CardId::of("two", "same words"));
    }

    /// Without the separator, ("ab", "c") and ("a", "bc") would hash alike.
    #[test]
    fn the_deck_and_the_card_cannot_run_together() {
        assert_ne!(CardId::of("ab", "c"), CardId::of("a", "bc"));
    }

    #[test]
    fn ids_survive_a_round_trip_through_text() {
        for deck in crate::decks() {
            for card in deck.cards() {
                let id = CardId::of(deck.id, card);
                assert_eq!(id.to_string().parse::<CardId>(), Ok(id));
            }
        }
    }

    #[test]
    fn malformed_ids_are_rejected() {
        for bad in ["", "0123", "not a hex id!!!!", "0123456789abcdefg", "  0123456789abcd"] {
            assert!(bad.parse::<CardId>().is_err(), "{bad:?} should not parse");
        }
    }

    /// An index moves when the deck is edited; that is why this type exists.
    /// The id must not depend on where the card sits.
    #[test]
    fn the_id_does_not_depend_on_position() {
        let deck = crate::decks().first().copied().expect("a deck is always compiled in");
        let first = deck.cards()[0];
        let last = deck.cards()[deck.count() - 1];
        assert_eq!(CardId::of(deck.id, first), deck.id_of(first).expect("in the deck"));
        assert_eq!(CardId::of(deck.id, last), deck.id_of(last).expect("in the deck"));
    }
}
