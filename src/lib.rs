//! Brian Eno and Peter Schmidt's [Oblique Strategies](https://en.wikipedia.org/wiki/Oblique_Strategies)
//! in a library, for those moments when the work is stuck and a dilemma needs a
//! lateral nudge.
//!
//! The crate holds one deck per module under [`mod@decks`], each drawn by the
//! same mechanism. The top-level functions draw from the Oblique Strategies
//! deck, so they read the way they always have; use [`decks()`] or
//! [`deck_by_id()`] to reach any of the others.
//!
//! Cards that carry several lines of text contain embedded newline and tab
//! characters; print them as-is and they will render the way the card reads.
//! Two cards in the Oblique Strategies deck are not instructions at all — the
//! blank white card and Pae White's graphic metacard — so card text is not
//! always a prompt.
//!
//! # Examples
//!
//! ```
//! let deck = sortes::decks().first().expect("a deck is always compiled in");
//! println!("The next move: {}", deck.random());
//! ```

pub mod decks;

#[cfg(not(any(
    feature = "oblique",
    feature = "examen",
    feature = "constraints",
    feature = "absurd",
    feature = "attention",
    feature = "memento",
    feature = "dramatis",
    feature = "stuck"
)))]
compile_error!("at least one deck feature must be enabled; the default is `oblique`");

/// Where a deck's text comes from, and who to credit for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provenance {
    /// Text by a named author, reproduced with attribution.
    Attributed(&'static str),
    /// Written for this crate.
    Original,
    /// Drawn from public-domain sources, named here.
    PublicDomain(&'static str),
    /// A restatement, in original words, of a technique that is itself not
    /// copyrightable. The source of the technique is named here.
    Technique(&'static str),
}

impl Provenance {
    /// The credit line for this provenance, if it has one.
    ///
    /// # Examples
    ///
    /// ```
    /// use sortes::Provenance;
    /// assert_eq!(Provenance::Original.attribution(), None);
    /// assert!(Provenance::Attributed("Eno").attribution().is_some());
    /// ```
    #[must_use]
    pub const fn attribution(&self) -> Option<&'static str> {
        match self {
            Self::Attributed(who) | Self::PublicDomain(who) | Self::Technique(who) => Some(who),
            Self::Original => None,
        }
    }
}

/// A deck of cards, drawn at random.
///
/// # Examples
///
/// ```
/// let deck = sortes::decks().first().expect("a deck is always compiled in");
/// assert!(!deck.random().is_empty());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Deck {
    /// Short, stable, lowercase identifier. Unique across the crate.
    pub id: &'static str,
    /// Human-readable name.
    pub name: &'static str,
    /// One line on what the deck is for.
    pub blurb: &'static str,
    /// Where the text came from.
    pub provenance: Provenance,
    /// The cards themselves, sorted by their first line.
    pub cards: &'static [&'static str],
}

impl Deck {
    /// Return this deck's cards as a slice of &str. Does not allocate.
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// assert_eq!(deck.cards().len(), deck.count());
    /// ```
    #[must_use]
    pub const fn cards(&self) -> &'static [&'static str] {
        self.cards
    }

    /// Returns the number of cards in this deck.
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// assert!(deck.count() > 0);
    /// ```
    #[must_use]
    pub const fn count(&self) -> usize {
        self.cards.len()
    }

    /// Return a randomly-selected card, as a static &str.
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// assert!(!deck.random_str().is_empty());
    /// ```
    #[must_use]
    pub fn random_str(&self) -> &'static str {
        self.cards[fastrand::usize(..self.cards.len())]
    }

    /// Return a randomly-selected card.
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// assert!(!deck.random().is_empty());
    /// ```
    #[must_use]
    pub fn random(&self) -> String {
        self.random_str().to_string()
    }

    /// Return multiple randomly-selected cards, drawn without replacement.
    ///
    /// Returns up to `count` cards. If `count` exceeds the size of the deck,
    /// returns the whole deck. Each card is drawn at most once.
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// assert!(deck.random_n(3).len() <= 3);
    /// ```
    #[must_use]
    pub fn random_n(&self, count: usize) -> Vec<String> {
        let count = count.min(self.cards.len());
        let mut indices = (0..self.cards.len()).collect::<Vec<_>>();
        fastrand::shuffle(&mut indices);
        indices
            .into_iter()
            .take(count)
            .map(|i| self.cards[i].to_string())
            .collect()
    }
}

/// Every deck compiled into this build.
///
/// # Examples
///
/// ```
/// assert!(!sortes::decks().is_empty());
/// ```
#[must_use]
pub const fn decks() -> &'static [&'static Deck] {
    &[
        #[cfg(feature = "oblique")]
        &decks::oblique::DECK,
        #[cfg(feature = "examen")]
        &decks::examen::DECK,
        #[cfg(feature = "constraints")]
        &decks::constraints::DECK,
        #[cfg(feature = "absurd")]
        &decks::absurd::DECK,
        #[cfg(feature = "attention")]
        &decks::attention::DECK,
        #[cfg(feature = "memento")]
        &decks::memento::DECK,
        #[cfg(feature = "dramatis")]
        &decks::dramatis::DECK,
        #[cfg(feature = "stuck")]
        &decks::stuck::DECK,
    ]
}

/// Look a deck up by its id.
///
/// Returns `None` if no such deck exists, or if it was not compiled into this
/// build.
///
/// # Examples
///
/// ```
/// let first = sortes::decks().first().expect("a deck is always compiled in");
/// assert_eq!(sortes::deck_by_id(first.id), Some(*first));
/// assert!(sortes::deck_by_id("no-such-deck").is_none());
/// ```
#[must_use]
pub fn deck_by_id(id: &str) -> Option<&'static Deck> {
    decks().iter().copied().find(|deck| deck.id == id)
}

// ---------------------------------------------------------------------------
// The Oblique Strategies deck, at the top level, as it has always been.
// ---------------------------------------------------------------------------

/// Return all strategies as a slice of &str. Does not allocate.
///
/// # Examples
///
/// ```
/// let strategies = sortes::strategies_as_slice();
/// assert!(strategies.contains(&"Honor thy error as a hidden intention"));
/// ```
#[cfg(feature = "oblique")]
#[must_use]
pub const fn strategies_as_slice() -> &'static [&'static str] {
    decks::oblique::DECK.cards
}

/// Return all strategies as a vector of strings. Allocates.
///
/// # Examples
///
/// ```
/// let all_strategies = sortes::strategies();
/// assert_eq!(all_strategies.len(), sortes::count());
/// ```
#[cfg(feature = "oblique")]
#[must_use]
pub fn strategies() -> Vec<String> {
    strategies_as_slice().iter().map(|s| s.to_string()).collect()
}

/// Return a randomly-selected strategy.
///
/// # Examples
///
/// ```
/// let strategy = sortes::random();
/// assert!(!strategy.is_empty());
/// ```
#[cfg(feature = "oblique")]
#[must_use]
pub fn random() -> String {
    random_str().to_string()
}

/// Return a randomly-selected strategy, as a static &str.
///
/// ```
/// let strategy = sortes::random_str();
/// assert!(!strategy.is_empty());
/// ```
#[cfg(feature = "oblique")]
#[must_use]
pub fn random_str() -> &'static str {
    decks::oblique::DECK.random_str()
}

/// Return multiple randomly-selected strategies, drawn without replacement.
///
/// Returns up to `count` strategies. If `count` exceeds the total number of
/// strategies, returns all available strategies. Each card is drawn at most
/// once. Variant wordings of the same strategy have been collapsed to a single
/// card, so a multi-card draw returns distinct ideas.
///
/// ```
/// let strategies = sortes::random_n(3);
/// assert!(strategies.len() <= 3);
/// ```
#[cfg(feature = "oblique")]
#[must_use]
pub fn random_n(count: usize) -> Vec<String> {
    decks::oblique::DECK.random_n(count)
}

/// Returns the total number of available strategies.
///
/// ```
/// let count = sortes::count();
/// assert!(count >= 156);
/// ```
#[cfg(feature = "oblique")]
#[must_use]
pub const fn count() -> usize {
    decks::oblique::DECK.count()
}

#[cfg(test)]
mod tests {
    use super::Deck;

    pub(super) fn first_lines(deck: &Deck) -> Vec<&'static str> {
        deck.cards
            .iter()
            .map(|card| card.lines().next().unwrap_or(card))
            .collect()
    }

    /// The invariants every deck must hold, whatever its content.
    macro_rules! deck_invariants {
        ($name:ident, $deck:expr, $expected_count:expr, $max_len:expr) => {
            mod $name {
                fn deck() -> &'static crate::Deck {
                    &$deck
                }

                #[test]
                fn has_expected_count() {
                    assert_eq!(deck().count(), $expected_count);
                }

                #[test]
                fn no_empty_entries() {
                    assert!(deck().cards.iter().all(|s| !s.is_empty()));
                }

                #[test]
                fn entries_are_tidy() {
                    for card in deck().cards {
                        assert_eq!(
                            card.trim(),
                            *card,
                            "card has leading or trailing whitespace: {card:?}"
                        );
                        assert!(
                            !card.contains("  "),
                            "card has a run of spaces; use \\n\\t formatting instead: {card:?}"
                        );
                        for line in card.lines().skip(1) {
                            assert!(
                                line.starts_with('\t'),
                                "continuation line missing tab indent: {card:?}"
                            );
                        }
                    }
                }

                #[test]
                fn no_duplicate_entries() {
                    let unique: std::collections::HashSet<_> = deck().cards.iter().collect();
                    assert_eq!(unique.len(), deck().count());
                }

                #[test]
                fn sorted_by_first_line() {
                    // Multi-line cards carry a literal `\n\t`, so sorting the raw
                    // literal puts them in the wrong place. The deck is ordered by
                    // what the reader sees, which is the first line.
                    let keys = crate::tests::first_lines(deck());
                    let mut sorted = keys.clone();
                    sorted.sort_unstable();
                    assert_eq!(keys, sorted, "deck {} is not sorted by first line", deck().id);
                }

                #[test]
                fn cards_are_within_max_length() {
                    // Measured on the escaped literal, so a multi-line card is
                    // judged the way it appears in the source.
                    for card in deck().cards {
                        let escaped = card.replace('\n', "\\n").replace('\t', "\\t");
                        assert!(
                            escaped.chars().count() <= $max_len,
                            "card is longer than {} for this deck: {card:?}",
                            $max_len
                        );
                    }
                }

                #[test]
                fn draws_are_in_deck() {
                    for card in deck().random_n(5) {
                        assert!(deck().cards.contains(&card.as_str()));
                    }
                }
            }
        };
    }

    // The Eno deck's ceiling is set by the "Short circuit" card, which carries
    // its own worked example. New decks are held to a tighter line.
    #[cfg(feature = "oblique")]
    deck_invariants!(oblique, crate::decks::oblique::DECK, 156, 130);

    #[cfg(feature = "examen")]
    deck_invariants!(examen, crate::decks::examen::DECK, 92, 100);

    #[cfg(feature = "constraints")]
    deck_invariants!(constraints, crate::decks::constraints::DECK, 100, 120);

    #[cfg(feature = "absurd")]
    deck_invariants!(absurd, crate::decks::absurd::DECK, 86, 120);

    // The terse deck; the tightest ceiling in the crate is the point of it.
    #[cfg(feature = "attention")]
    deck_invariants!(attention, crate::decks::attention::DECK, 93, 60);

    #[cfg(feature = "memento")]
    deck_invariants!(memento, crate::decks::memento::DECK, 76, 110);

    #[cfg(feature = "dramatis")]
    deck_invariants!(dramatis, crate::decks::dramatis::DECK, 104, 90);

    #[cfg(feature = "stuck")]
    deck_invariants!(stuck, crate::decks::stuck::DECK, 90, 80);

    // ---- Voice rules ------------------------------------------------------
    //
    // Four decks promise a rule about how their cards may speak. The promises
    // are kept here rather than in prose, so a card added later cannot quietly
    // break one.

    /// Whether `card` uses `word` as a whole word, ignoring case.
    #[cfg(any(feature = "attention", feature = "dramatis"))]
    fn uses_word(card: &str, word: &str) -> bool {
        card.split(|c: char| !c.is_alphanumeric() && c != '\'')
            .any(|found| found.eq_ignore_ascii_case(word))
    }

    /// What a deck that points away from the reader must never say.
    #[cfg(any(feature = "attention", feature = "dramatis"))]
    const SECOND_PERSON: &[&str] = &["you", "your", "yours", "yourself", "yourselves"];

    /// `attention` points outward: no card addresses the reader, and none asks
    /// a question. Its sixty-character ceiling is checked by the invariants.
    #[cfg(feature = "attention")]
    #[test]
    fn attention_points_outward() {
        for card in crate::decks::attention::DECK.cards {
            assert!(!card.contains('?'), "attention card asks a question: {card:?}");
            for pronoun in SECOND_PERSON {
                assert!(
                    !uses_word(card, pronoun),
                    "attention card addresses the reader: {card:?}"
                );
            }
        }
    }

    /// `memento` only states. `absurd` is the deck that asks and instructs;
    /// the two are kept apart by mood, and this is the machine-checkable half
    /// of that difference.
    #[cfg(feature = "memento")]
    #[test]
    fn memento_only_states() {
        for card in crate::decks::memento::DECK.cards {
            assert!(!card.contains('?'), "memento card asks a question: {card:?}");
        }
    }

    /// `dramatis` cards are labels, not sentences: no terminal punctuation, no
    /// questions, and nothing addressed to the reader.
    #[cfg(feature = "dramatis")]
    #[test]
    fn dramatis_cards_are_labels() {
        for card in crate::decks::dramatis::DECK.cards {
            // Cards are trimmed, which `entries_are_tidy` enforces, so the
            // last character is the one the reader sees.
            let last = card.chars().next_back().expect("cards are never empty");
            assert!(
                !matches!(last, '.' | '!' | '?'),
                "dramatis card carries terminal punctuation: {card:?}"
            );
            for pronoun in SECOND_PERSON {
                assert!(
                    !uses_word(card, pronoun),
                    "dramatis card addresses the reader: {card:?}"
                );
            }
        }
    }

    /// The only deck sourced rather than authored, so it is the only one whose
    /// provenance must carry a credit line.
    #[cfg(feature = "dramatis")]
    #[test]
    fn dramatis_is_attributed() {
        let deck = crate::decks::dramatis::DECK;
        assert!(matches!(deck.provenance, crate::Provenance::PublicDomain(_)));
        assert!(deck.provenance.attribution().is_some());
    }

    #[test]
    fn at_least_one_deck() {
        assert!(!super::decks().is_empty());
    }

    #[test]
    fn deck_ids_are_unique() {
        let ids: std::collections::HashSet<_> = super::decks().iter().map(|d| d.id).collect();
        assert_eq!(ids.len(), super::decks().len());
    }

    #[test]
    fn every_deck_is_reachable_by_id() {
        for deck in super::decks() {
            assert_eq!(super::deck_by_id(deck.id), Some(*deck));
        }
    }

    #[test]
    fn unknown_deck_id() {
        assert_eq!(super::deck_by_id("no-such-deck"), None);
    }

    #[cfg(feature = "oblique")]
    #[test]
    fn random_strategy() {
        let strategy = super::random();
        assert!(!strategy.is_empty());
    }

    #[cfg(feature = "oblique")]
    #[test]
    fn all_strategies() {
        let list = super::strategies();
        assert_eq!(list.len(), super::count());
    }

    #[cfg(feature = "oblique")]
    #[test]
    fn random_strategy_borrowed() {
        let strategy: &'static str = super::random_str();
        assert!(!strategy.is_empty());
    }

    #[cfg(feature = "oblique")]
    #[test]
    fn all_strategies_borrowed() {
        let list: &'static [&str] = super::strategies_as_slice();
        assert_eq!(list.len(), super::count());
    }

    #[cfg(feature = "oblique")]
    #[test]
    fn random_multiple() {
        let strategies = super::random_n(5);
        assert_eq!(strategies.len(), 5);
    }

    #[cfg(feature = "oblique")]
    #[test]
    fn random_n_exceeds_total() {
        let strategies = super::random_n(1000);
        assert_eq!(strategies.len(), super::count());
    }

    #[cfg(feature = "oblique")]
    #[test]
    fn random_n_zero() {
        let strategies = super::random_n(0);
        assert!(strategies.is_empty());
    }

    #[cfg(feature = "oblique")]
    #[test]
    fn count_is_156() {
        assert_eq!(super::count(), 156);
    }

    #[cfg(feature = "oblique")]
    #[test]
    fn top_level_delegates_to_the_oblique_deck() {
        let deck = super::deck_by_id("oblique").expect("compiled in");
        assert_eq!(super::strategies_as_slice(), deck.cards());
        assert_eq!(super::count(), deck.count());
    }
}
