//! Draw one card and print it.
//!
//! Uses `decks().first()` rather than naming a deck, so the example runs under
//! any feature set: with the default features that is the Oblique Strategies
//! deck, and in a build without it, whichever deck is compiled in first.
//!
//! Run it with `cargo run --example random-strategy`.

fn main() {
    if let Some(deck) = sortes::decks().first() {
        println!("The next move: {}", deck.random());
    }
}
