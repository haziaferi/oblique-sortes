fn main() {
    if let Some(deck) = sortes::decks().first() {
        println!("The next move: {}", deck.random());
    }
}
