# oblique-sortes

Eight decks of cards for when the work will not move. Brian Eno and Peter Schmidt's [Oblique Strategies](https://en.wikipedia.org/wiki/Oblique_Strategies) is one of them; the other seven are for the end of a day, for writing under constraint, for finitude, for attention, for assembling a story, and for a problem that has stopped.

Named for the *sortes Vergilianae*, the practice of opening Virgil at random and taking what you land on as counsel. That is the whole mechanism here.

## Installing

Not on crates.io — install from git.

```toml
[dependencies]
oblique-sortes = { git = "https://github.com/haziaferi/oblique-sortes" }
```

```rust
use sortes::Deck; // the library is `sortes`, not `oblique_sortes`
```

For the command-line tool:

```bash
cargo install --git https://github.com/haziaferi/oblique-sortes
```

The binary is `sortes`. The package is called `oblique-sortes` because upstream holds `oblique` on crates.io.

The list is a curated amalgam of the card text from several editions of the deck. Where editions worded the same strategy differently, one wording is kept, so that drawing several cards does not hand you the same idea twice. The deck's two non-instruction cards are kept, as the deck itself has them: the blank white card and Pae White's graphic metacard. Cards that have several lines of text contain embedded newline and tab characters. Print them as-is and they will render the way the card reads:

```text
>  sortes
Destroy
	-nothing
	-the most important thing
```

## Usage

```rust
let strategy = sortes::random();
println!("The next move: {}", strategy);
```

If you are using the cli:

```text
>  sortes
Honor thy error as a hidden intention

>  sortes examen
What worked so well today that you failed to notice it?

>  sortes constraints -n 2
Cut the part you had to explain.
Write the whole thing as one sentence.

>  sortes --any
A weed doing well.

>  sortes stuck --seed 1979
Whose problem is this actually?

>  sortes --list
* oblique       156  Oblique Strategies
  examen         92  Examen
  ...

>  sortes --about dramatis
Dramatis (dramatis)
Situations, casts and beats to assemble a story from.
104 cards
Text from the public domain: Georges Polti (Ray trans. 1916), ...

>  sortes attention --all
A weed doing well.
Attend to the pause, not the note.
...

>  sortes oblique --find repetition
Emphasize repetitions
Repetition is a form of change
```

`sortes --help` prints the rest. With no deck named it draws from the first deck compiled in,
which is `oblique` whenever that feature is on — so the bare command behaves as it always has.
`--any` picks the deck for you; `--seed N` makes a draw reproducible, for as long as the decks
hold the same cards. `--find TEXT` selects rather than draws: it prints every card that matches,
ignoring case, and prints nothing and still succeeds when none does — an empty result is an
answer, and an unknown deck is the thing that actually fails.

Shell completions for bash, zsh and fish live in [`completions/`](./completions). They read the
deck names from `sortes --list` rather than keeping a copy, so a single-deck build completes a
single deck.

## Decks

| Deck | Cards | What it is |
|---|---|---|
| `oblique` | 156 | Eno and Schmidt's lateral nudges, for when the work is stuck. |
| `examen` | 92 | Questions for the end of a day. Written for this crate; the register comes from the traditions of daily self-examination, not from their texts. |
| `constraints` | 100 | Rules to write against. Oulipo, Dada and Surrealist procedures, each restated in its own words. |
| `absurd` | 86 | Freedom, finitude, and the stories you tell to avoid both. Written for this crate, in the existentialist register; it addresses you directly and does not soften. |
| `attention` | 93 | Where to put it, for a minute. The terse deck, nearest the Oblique cards that are only a noun. Points outward: no card says "you", none asks a question, none runs past sixty characters. |
| `memento` | 76 | Finitude, stated plainly. Where `absurd` provokes, this one only states — no questions, no instructions. It withholds consolation, as the tradition does, without being bleak for its own sake. |
| `dramatis` | 104 | Situations, casts and beats to assemble a story from. The one deck taken from sources rather than written for the crate: Polti, Propp and Aristotle, all public domain. Cards are labels, not sentences. |
| `stuck` | 90 | Moves for when the work will not move. Shares the Oblique deck's territory and works the opposite way: where `oblique` sidesteps a problem, this one attacks it. Every card asks or instructs. |

Each deck lives in its own module under `sortes::decks` and is compiled in behind its own
feature, so you pay only for the decks you enable. `default = ["oblique"]`; `full` enables every
deck. A build with no deck feature at all fails with a message saying so.

The top-level functions draw from the Oblique Strategies deck and read the way they always have.
To reach a deck directly:

```rust
let deck = sortes::deck_by_id("oblique").expect("compiled in");
println!("{}", deck.random());

for deck in sortes::decks() {
    println!("{} ({} cards)", deck.name, deck.count());
}
```

`random()` asks the thread-local generator each time, so a run of draws may repeat a card and
cannot be replayed. A `Shoe` keeps a shuffled order and a cursor into it instead: every card
comes up once before any comes up twice, the card ending one pass never opens the next, and a
shoe built from a seed deals the same sequence every time.

```rust
let deck = sortes::deck_by_id("oblique").expect("compiled in");

let mut shoe = deck.shoe();
for _ in 0..deck.count() {
    println!("{}", shoe.draw()); // the whole deck, no card twice
}

// Reproducible: the same seed, the same cards, in the same order.
assert_eq!(deck.shoe_with_seed(1979).draw_n(5), deck.shoe_with_seed(1979).draw_n(5));
```

The generator lives inside the shoe, not in `Deck` — decks stay compile-time constants, and no
dependency of this crate reaches its public API. A seed holds only while the decks hold the same
cards; one added or reworded in a later release reshuffles every seed with it.

Every card has a stable id, derived from its deck and its own text rather than its position, so
something that saves a card can find it again. Rewording a card changes its id, which is the
honest answer: whatever saved the old wording should notice rather than quietly show the new one.

```rust
use sortes::CardId;

let deck = sortes::deck_by_id("oblique").expect("compiled in");
let card = deck.cards()[0];
let id = deck.id_of(card).expect("the card is in the deck");

// Written down, read back, and found again — in any build that has the deck.
let written = id.to_string();
let id = written.parse::<CardId>().expect("we just wrote it");
assert_eq!(sortes::card_by_id(id), Some((*deck, card)));
```

`Deck::find` searches a deck. Card text is ASCII throughout, so the match is case-blind and
allocates nothing:

```rust
let deck = sortes::deck_by_id("oblique").expect("compiled in");
for card in deck.find("repetition") {
    println!("{card}");
}
```

## Android

The same decks as an app: `minSdk 30`, `targetSdk 36`, arm64-v8a.

```bash
just apk        # gradle drives cargo; this is the whole build
just install    # and put it on a connected device
```

`android/jni` is a workspace member holding the JNI shim, so the library itself
stays platform-free and `unsafe`-free; `android/app` is one Activity with no
AndroidX and one XML layout, which is the widget's and belongs to it. Pick a
deck, draw one card or several, search it, keep the ones worth keeping, share
one or long-press to copy it. Draws come from a shoe, so working through a deck
does not hand back cards already seen, and a drawn card survives a rotation
because it cannot be drawn again. There is a home-screen widget that shows one
card and redraws it when tapped. See [android/README.md](./android/README.md).

## Credit

Forked from [ceejbot/oblique](https://github.com/ceejbot/oblique) by C J Silverio, which is where the Oblique Strategies deck, the API shape and the release pipeline all come from. Brian Eno and Peter Schmidt are the originators and copyright holders of the Oblique Strategies card text; the `dramatis` deck reproduces public-domain text from Georges Polti, Vladimir Propp and Aristotle. Every other deck was written for this crate. `SPEC.md` records where each one came from and why.

## See also

[gsv-culture-ships](https://github.com/ceejbot/gsv-culture-ships), the upstream library's structural twin for Iain M. Banks's Culture ship names.

## LICENSE

[ISC](./LICENSE.md), as upstream. Go ahead and steal this one, o freeloading megacorps.
