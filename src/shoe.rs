//! A dealing shoe: draws that do not repeat, and draws that can be reproduced.
//!
//! [`Deck::random`] and its neighbours ask the thread-local generator for a
//! card each time, so a run of draws may show the same card twice and no run
//! can be replayed. A [`Shoe`] keeps a shuffled order and a cursor into it
//! instead: every card comes up once before any comes up twice, and a shoe
//! built with [`Deck::shoe_with_seed`] deals the same sequence every time.
//!
//! The generator lives inside the shoe rather than in the deck. `Deck` is a
//! compile-time constant and stays one; the shoe is the mutable thing, and it
//! is the only place in this crate that holds state between draws.

use crate::Deck;

/// A deck, shuffled, with a cursor into it.
///
/// Cards come out in shuffled order until the shoe is spent, then it shuffles
/// again and carries on — so a draw repeats only after every other card has
/// been dealt. The card that ends one pass never opens the next.
///
/// A shoe from [`Deck::shoe_with_seed`] is reproducible: the same seed deals
/// the same cards in the same order, for as long as the deck's contents do not
/// change.
///
/// # Examples
///
/// ```
/// let deck = sortes::decks().first().expect("a deck is always compiled in");
/// let mut shoe = deck.shoe_with_seed(1234);
/// let first = shoe.draw();
/// // The same seed deals the same card.
/// assert_eq!(deck.shoe_with_seed(1234).draw(), first);
/// ```
#[derive(Debug, Clone)]
pub struct Shoe {
    deck: Deck,
    order: Vec<&'static str>,
    next: usize,
    rng: fastrand::Rng,
}

impl Shoe {
    /// Build a shoe over `deck`, shuffled by `rng`.
    pub(crate) fn new(deck: Deck, mut rng: fastrand::Rng) -> Self {
        let mut order = deck.cards.to_vec();
        rng.shuffle(&mut order);
        Self {
            deck,
            order,
            next: 0,
            rng,
        }
    }

    /// The deck this shoe deals from.
    ///
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// assert_eq!(deck.shoe().deck().id, deck.id);
    /// ```
    #[must_use]
    pub const fn deck(&self) -> Deck {
        self.deck
    }

    /// How many cards are left before the shoe shuffles again.
    ///
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// let mut shoe = deck.shoe();
    /// assert_eq!(shoe.remaining(), deck.count());
    /// shoe.draw();
    /// assert_eq!(shoe.remaining(), deck.count() - 1);
    /// ```
    // Not `const`: `Vec::len` is const only from 1.87, and this crate says 1.85.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.order.len() - self.next
    }

    /// Shuffle the shoe and start over, so the next draw comes from a full
    /// deck.
    ///
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// let mut shoe = deck.shoe();
    /// shoe.draw();
    /// shoe.reshuffle();
    /// assert_eq!(shoe.remaining(), deck.count());
    /// ```
    pub fn reshuffle(&mut self) {
        self.rng.shuffle(&mut self.order);
        self.next = 0;
    }

    /// Deal the next card.
    ///
    /// When the shoe is spent it shuffles and carries on, so this never fails
    /// and never runs out. As elsewhere in the crate, a deck is assumed to hold
    /// at least one card; every deck compiled in does, and a test enforces it.
    ///
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// let mut shoe = deck.shoe();
    /// assert!(deck.cards().contains(&shoe.draw()));
    /// ```
    pub fn draw(&mut self) -> &'static str {
        if self.next >= self.order.len() {
            self.refill();
        }
        let card = self.order[self.next];
        self.next += 1;
        card
    }

    /// Deal `count` cards.
    ///
    /// Unlike [`Deck::random_n_str`], which stops at the size of the deck, a
    /// shoe keeps dealing: ask for more cards than the deck holds and it
    /// shuffles and continues, so the length is always `count`.
    ///
    /// # Examples
    ///
    /// ```
    /// let deck = sortes::decks().first().expect("a deck is always compiled in");
    /// let mut shoe = deck.shoe();
    /// assert_eq!(shoe.draw_n(3).len(), 3);
    /// ```
    pub fn draw_n(&mut self, count: usize) -> Vec<&'static str> {
        (0..count).map(|_| self.draw()).collect()
    }

    /// Shuffle for the next pass, keeping the seam honest.
    fn refill(&mut self) {
        let last = self.order.last().copied();
        self.reshuffle();
        // A card that ends one pass and opens the next reads as a repeat,
        // which is the one thing a shoe exists to prevent. Move it aside.
        if self.order.len() > 1 && self.order.first().copied() == last {
            let elsewhere = 1 + self.rng.usize(..self.order.len() - 1);
            self.order.swap(0, elsewhere);
        }
    }
}

#[cfg(test)]
mod tests {
    fn deck() -> &'static crate::Deck {
        crate::decks().first().copied().expect("a deck is always compiled in")
    }

    #[test]
    fn a_pass_deals_every_card_exactly_once() {
        let deck = deck();
        let mut shoe = deck.shoe();
        let pass = shoe.draw_n(deck.count());
        let unique: std::collections::HashSet<_> = pass.iter().collect();
        assert_eq!(unique.len(), deck.count(), "a card came up twice within one pass");
        for card in &pass {
            assert!(deck.cards().contains(card));
        }
    }

    #[test]
    fn the_same_seed_deals_the_same_sequence() {
        let deck = deck();
        let one = deck.shoe_with_seed(0x5EED).draw_n(40);
        let two = deck.shoe_with_seed(0x5EED).draw_n(40);
        assert_eq!(one, two);
    }

    #[test]
    fn different_seeds_deal_differently() {
        // Two shuffles of a deck this size agreeing on their first forty cards
        // is not a thing that happens; if it does, the seed is not reaching the
        // shuffle.
        let deck = deck();
        assert_ne!(deck.shoe_with_seed(1).draw_n(40), deck.shoe_with_seed(2).draw_n(40));
    }

    #[test]
    fn the_shoe_refills_rather_than_running_out() {
        let deck = deck();
        let mut shoe = deck.shoe_with_seed(7);
        let drawn = shoe.draw_n(deck.count() * 2 + 5);
        assert_eq!(drawn.len(), deck.count() * 2 + 5);
    }

    #[test]
    fn no_card_repeats_across_the_seam() {
        let deck = deck();
        // Every seed, so this is not one lucky shuffle: the pass boundary is
        // where a naive reshuffle would show the same card twice in a row.
        for seed in 0..64 {
            let mut shoe = deck.shoe_with_seed(seed);
            let drawn = shoe.draw_n(deck.count() * 3);
            for pair in drawn.windows(2) {
                assert_ne!(pair[0], pair[1], "seed {seed} dealt the same card twice running");
            }
        }
    }

    #[test]
    fn remaining_counts_down_and_wraps() {
        let deck = deck();
        let mut shoe = deck.shoe();
        assert_eq!(shoe.remaining(), deck.count());
        shoe.draw_n(deck.count());
        assert_eq!(shoe.remaining(), 0);
        shoe.draw();
        assert_eq!(shoe.remaining(), deck.count() - 1);
    }

    #[test]
    fn drawing_nothing_is_allowed() {
        assert!(deck().shoe().draw_n(0).is_empty());
    }

    #[test]
    fn a_shoe_knows_its_deck() {
        let deck = deck();
        assert_eq!(deck.shoe().deck(), *deck);
    }
}
