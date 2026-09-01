fn main() {
    if let Some(deck) = oblique::decks().first() {
        println!("The next move: {}", deck.random());
    }
}
