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
mod identity;
mod shoe;

pub use identity::{CardId, ParseCardIdError};
pub use shoe::Shoe;

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
///
/// Non-exhaustive: a mode added later must not break a consumer's `match`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
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
    pub const fn attribution(self) -> Option<&'static str> {
        match self {
            Self::Attributed(who) | Self::PublicDomain(who) | Self::Technique(who) => Some(who),
            Self::Original => None,
        }
    }

    /// A sentence saying where the text came from, ready to print.
    ///
    /// The wording lives here rather than in each consumer. Outside this crate
    /// [`Provenance`] cannot be matched exhaustively, so a mode added later
    /// would otherwise fall through to a guess in every binding that renders
    /// one; here it is a compile error in a single place.
    ///
    /// # Examples
    ///
    /// ```
    /// use sortes::Provenance;
    /// assert_eq!(Provenance::Original.describe(), "Text written for this crate.");
    /// assert!(Provenance::Attributed("Eno").describe().contains("Eno"));
    /// ```
    #[must_use]
    pub fn describe(self) -> String {
        match self {
            Self::Attributed(who) => format!("Text by {who}, reproduced with attribution."),
            Self::Original => "Text written for this crate.".to_string(),
            Self::PublicDomain(who) => format!("Text from the public domain: {who}."),
            Self::Technique(who) => format!("Methods restated in original words: {who}."),
        }
    }
}

/// A deck of cards, drawn at random.
///
/// Non-exhaustive: these are the deck's fields as they stand today, and a card
/// note or a stable card id would each add one. Build decks through this crate
/// rather than by literal, and adding a field stays a minor release.
///
/// # Examples
///
/// ```
/// let deck = sortes::decks().first().expect("a deck is always compiled in");
/// assert!(!deck.random().is_empty());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
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
    /// # Examples
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
    /// # Examples
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
    /// # Examples
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
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// assert!(!deck.random().is_empty());
    /// ```
    #[must_use]
    pub fn random(&self) -> String {
        self.random_str().to_string()
    }

    /// Return multiple randomly-selected cards, drawn without replacement, as
    /// static &str. Draws no `String` per card; a single-card draw allocates
    /// only the returned vector, and a larger one adds a deck-sized scratch.
    ///
    /// Returns up to `count` cards. If `count` exceeds the size of the deck,
    /// returns the whole deck. Each card is drawn at most once — for a run that
    /// keeps that promise across calls, use a [`Shoe`].
    ///
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// assert!(deck.random_n_str(3).len() <= 3);
    /// ```
    #[must_use]
    pub fn random_n_str(&self, count: usize) -> Vec<&'static str> {
        // The common draw is one card, and shuffling a whole deck to take the
        // top of it is work nobody asked for.
        match count {
            0 => return Vec::new(),
            1 => return vec![self.random_str()],
            _ => {}
        }
        let mut drawn = self.cards.to_vec();
        let take = count.min(drawn.len());
        // Fisher-Yates, stopped once the first `take` positions are settled.
        // A full shuffle would go on to order the cards nobody will see.
        for position in 0..take {
            let picked = position + fastrand::usize(..drawn.len() - position);
            drawn.swap(position, picked);
        }
        drawn.truncate(take);
        drawn
    }

    /// Every card containing `needle`, ignoring case, in the deck's own order.
    ///
    /// Card text is ASCII throughout the crate, so the comparison is an ASCII
    /// one and allocates nothing. An empty needle matches every card.
    ///
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// assert_eq!(deck.find("").len(), deck.count());
    /// assert!(deck.find("no card says this and none ever will").is_empty());
    /// ```
    #[must_use]
    pub fn find(&self, needle: &str) -> Vec<&'static str> {
        self.cards
            .iter()
            .copied()
            .filter(|card| contains_ignore_ascii_case(card, needle))
            .collect()
    }

    /// This card's stable id, or `None` if the card is not in this deck.
    ///
    /// See [`CardId`] for what "stable" is worth: the id is derived from the
    /// deck id and the card's text, so it survives cards being added, removed
    /// or reordered around it, and it changes when the card itself is reworded.
    ///
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// let card = deck.cards()[0];
    /// let id = deck.id_of(card).expect("the card is in the deck");
    /// assert_eq!(deck.card_by_id(id), Some(card));
    /// assert_eq!(deck.id_of("not a card in any deck"), None);
    /// ```
    #[must_use]
    pub fn id_of(&self, card: &str) -> Option<CardId> {
        self.cards
            .iter()
            .find(|held| **held == card)
            .map(|held| CardId::of(self.id, held))
    }

    /// The card with this id, or `None` if this deck holds no such card.
    ///
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// let id = deck.id_of(deck.cards()[0]).expect("the card is in the deck");
    /// assert!(deck.card_by_id(id).is_some());
    /// ```
    #[must_use]
    pub fn card_by_id(&self, id: CardId) -> Option<&'static str> {
        self.cards.iter().copied().find(|card| CardId::of(self.id, card) == id)
    }

    /// A [`Shoe`] over this deck, shuffled by the thread-local generator.
    ///
    /// Use it when a run of draws should not repeat a card; use
    /// [`Deck::shoe_with_seed`] when the run should also be reproducible.
    ///
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// let mut shoe = deck.shoe();
    /// assert!(deck.cards().contains(&shoe.draw()));
    /// ```
    #[must_use]
    pub fn shoe(&self) -> Shoe {
        Shoe::new(*self, fastrand::Rng::new())
    }

    /// A [`Shoe`] over this deck, shuffled from `seed`.
    ///
    /// The same seed deals the same cards in the same order, for as long as the
    /// deck's contents do not change. That last clause is the whole caveat: a
    /// card added or reworded in a later release reshuffles every seed with it.
    ///
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// assert_eq!(deck.shoe_with_seed(99).draw_n(5), deck.shoe_with_seed(99).draw_n(5));
    /// ```
    #[must_use]
    pub fn shoe_with_seed(&self, seed: u64) -> Shoe {
        Shoe::new(*self, fastrand::Rng::with_seed(seed))
    }

    /// Return multiple randomly-selected cards, drawn without replacement.
    ///
    /// Returns up to `count` cards. If `count` exceeds the size of the deck,
    /// returns the whole deck. Each card is drawn at most once.
    ///
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// assert!(deck.random_n(3).len() <= 3);
    /// ```
    #[must_use]
    pub fn random_n(&self, count: usize) -> Vec<String> {
        self.random_n_str(count).into_iter().map(str::to_string).collect()
    }
}

impl std::fmt::Display for Deck {
    /// The deck's name and id, as `--list` and `--about` both open with.
    ///
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// assert_eq!(deck.to_string(), format!("{} ({})", deck.name, deck.id));
    /// ```
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} ({})", self.name, self.id)
    }
}

impl IntoIterator for &Deck {
    type Item = &'static str;
    type IntoIter = std::iter::Copied<std::slice::Iter<'static, &'static str>>;

    /// The deck's cards, in order. Borrows the deck; the cards outlive it.
    ///
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// assert_eq!(deck.into_iter().count(), deck.count());
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.cards.iter().copied()
    }
}

/// The card with this id, and the deck it belongs to.
///
/// Searches every deck compiled in. A card saved by id in one build and looked
/// up in another finds nothing if its deck was not compiled in, or if the card
/// itself has been reworded since — which is the point of the id, not a flaw
/// in it.
///
/// # Examples
///
/// ```
/// let deck = sortes::decks().first().expect("a deck is always compiled in");
/// let card = deck.cards()[0];
/// let id = deck.id_of(card).expect("the card is in the deck");
/// assert_eq!(sortes::card_by_id(id), Some((**deck, card)));
/// ```
#[must_use]
pub fn card_by_id(id: CardId) -> Option<(Deck, &'static str)> {
    decks()
        .iter()
        .find_map(|deck| deck.card_by_id(id).map(|card| (**deck, card)))
}

/// Whether `haystack` contains `needle`, comparing ASCII letters case-blind.
///
/// Every card in the crate is ASCII, so this is the whole of the matching rule
/// and it needs no allocation; `str::to_lowercase` on each card would allocate
/// once per card per search.
fn contains_ignore_ascii_case(haystack: &str, needle: &str) -> bool {
    let (haystack, needle) = (haystack.as_bytes(), needle.as_bytes());
    if needle.is_empty() {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }
    haystack
        .windows(needle.len())
        .any(|window| window.eq_ignore_ascii_case(needle))
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
    strategies_as_slice().iter().copied().map(str::to_string).collect()
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
/// # Examples
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
/// # Examples
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

/// Return multiple randomly-selected strategies, drawn without replacement, as
/// static &str. Makes one deck-sized allocation and no `String` per strategy.
///
/// Behaves as [`random_n()`] otherwise.
///
/// # Examples
///
/// ```
/// let strategies = sortes::random_n_str(3);
/// assert!(strategies.len() <= 3);
/// ```
#[cfg(feature = "oblique")]
#[must_use]
pub fn random_n_str(count: usize) -> Vec<&'static str> {
    decks::oblique::DECK.random_n_str(count)
}

/// Returns the total number of available strategies.
///
/// # Examples
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

    pub(crate) fn first_lines(deck: &Deck) -> Vec<&'static str> {
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

                #[test]
                fn draws_are_without_replacement() {
                    let drawn = deck().random_n_str(5);
                    let unique: std::collections::HashSet<_> = drawn.iter().collect();
                    assert_eq!(unique.len(), drawn.len(), "a card was drawn twice: {drawn:?}");
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
    // Three decks promise a rule about how their cards may speak that a test
    // can judge, and the checks live here rather than in prose, so a card
    // added later cannot quietly break one. The rules that need a reader
    // instead — memento's imperative openings, stuck's ask-or-instruct,
    // absurd's borrowed phrasings — are named as editorial in the module
    // docs.

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
    fn find_matches_case_blind_and_keeps_deck_order() {
        for deck in super::decks() {
            // Every card matches itself, whatever case it is asked in.
            for card in deck.cards().iter().take(5) {
                assert!(deck.find(card).contains(card));
                assert!(
                    deck.find(&card.to_uppercase()).contains(card),
                    "case-blind match failed: {card:?}"
                );
                assert!(
                    deck.find(&card.to_lowercase()).contains(card),
                    "case-blind match failed: {card:?}"
                );
            }
            // An empty needle matches everything; results keep the deck's
            // order.
            let all = deck.find("");
            assert_eq!(all.len(), deck.count());
            assert_eq!(all, deck.cards().to_vec());
            assert!(deck.find("no card says this and none ever will").is_empty());
        }
    }

    #[test]
    fn find_needle_longer_than_the_card_does_not_match() {
        let deck = super::decks().first().copied().expect("a deck is always compiled in");
        let long = "x".repeat(1000);
        assert!(deck.find(&long).is_empty());
    }

    /// The partial shuffle has to keep the guarantees the full one gave.
    #[test]
    fn small_draws_stay_uniform_and_distinct() {
        for deck in super::decks() {
            for count in [0, 1, 2, 3, 7] {
                let drawn = deck.random_n_str(count);
                assert_eq!(drawn.len(), count.min(deck.count()));
                let unique: std::collections::HashSet<_> = drawn.iter().collect();
                assert_eq!(unique.len(), drawn.len(), "a card was drawn twice: {drawn:?}");
                for card in &drawn {
                    assert!(deck.cards().contains(card));
                }
            }
            assert_eq!(deck.random_n_str(deck.count() + 10).len(), deck.count());
        }
    }

    /// A partial Fisher-Yates that stopped early would only ever return cards
    /// from the front of the deck. Over enough draws every card must appear.
    #[test]
    fn one_card_draws_reach_the_whole_deck() {
        let deck = super::decks().first().copied().expect("a deck is always compiled in");
        let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
        for _ in 0..(deck.count() * 40) {
            seen.extend(deck.random_n_str(1));
        }
        assert_eq!(
            seen.len(),
            deck.count(),
            "single-card draws never reached {} cards",
            deck.count()
        );
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

    /// The borrowed twin draws under the same rules as the owned one.
    #[cfg(feature = "oblique")]
    #[test]
    fn random_multiple_borrowed() {
        let strategies: Vec<&'static str> = super::random_n_str(5);
        assert_eq!(strategies.len(), 5);
        assert_eq!(super::random_n_str(1000).len(), super::count());
        assert!(super::random_n_str(0).is_empty());
        for strategy in super::random_n_str(5) {
            assert!(super::strategies_as_slice().contains(&strategy));
        }
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
