//! Deal a whole deck without repeating a card, then replay the same deal.
//!
//! `random()` asks the thread-local generator each time, so a run of draws may
//! show the same card twice and cannot be replayed. A `Shoe` keeps a shuffled
//! order and a cursor into it, which is what both of those want.
//!
//! Run it with `cargo run --example no-repeat`.

fn main() {
    let Some(deck) = sortes::decks().first() else {
        return;
    };

    // A pass through the shoe is the whole deck, in some order, once each.
    let mut shoe = deck.shoe();
    for card in shoe.draw_n(deck.count()) {
        println!("{card}");
    }
    println!("\n-- {} cards, none of them twice --\n", deck.count());

    // With a seed, the deal is the same every run. It holds only while the deck
    // holds the same cards: one added or reworded reshuffles every seed with
    // it.
    let seeded = deck.shoe_with_seed(1979).draw_n(3);
    assert_eq!(seeded, deck.shoe_with_seed(1979).draw_n(3));
    println!("seed 1979 always deals:");
    for card in seeded {
        println!("{card}");
    }
}
