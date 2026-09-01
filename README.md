# Oblique Strategies, and seven more decks

Brian Eno and Peter Schmidt's [Oblique Strategies](https://en.wikipedia.org/wiki/Oblique_Strategies) in a library, for those moments when the work is stuck and a dilemma needs a lateral nudge — alongside seven other decks for the end of a day, for writing under constraint, for finitude, for attention, and for a problem that will not move.

## Installing

The crate is published as **`oblique-decks`**, because upstream holds `oblique` on crates.io. The library keeps the shorter name, so what you install and what you import differ:

```toml
[dependencies]
oblique-decks = "0.2"
```

```rust
use oblique::Deck; // not oblique_decks
```

The binary is `oblique`.

The list is a curated amalgam of the card text from several editions of the deck. Where editions worded the same strategy differently, one wording is kept, so that drawing several cards does not hand you the same idea twice. The deck's two non-instruction cards are kept, as the deck itself has them: the blank white card and Pae White's graphic metacard. Cards that have several lines of text contain embedded newline and tab characters. Print them as-is and they will render the way the card reads:

```text
>  oblique
Destroy
	-nothing
	-the most important thing
```

## Usage

```rust
let strategy = oblique::random();
println!("The next move: {}", strategy);
```

If you are using the cli:

```text
>  oblique
Honor thy error as a hidden intention

>  oblique examen
What worked so well today that you failed to notice it?

>  oblique constraints -n 2
Cut the part you had to explain.
Write the whole thing as one sentence.

>  oblique --list
* oblique       156  Oblique Strategies
  examen         92  Examen
  ...

>  oblique --about dramatis
Dramatis (dramatis)
Situations, casts and beats to assemble a story from.
104 cards
Text from the public domain: Georges Polti (Ray trans. 1916), ...
```

`oblique --help` prints the rest. With no deck named it draws from the first deck compiled in,
which is `oblique` whenever that feature is on — so the bare command behaves as it always has.

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

Each deck lives in its own module under `oblique::decks` and is compiled in behind its own
feature, so you pay only for the decks you enable. `default = ["oblique"]`; `full` enables every
deck. A build with no deck feature at all fails with a message saying so.

The top-level functions draw from the Oblique Strategies deck and read the way they always have.
To reach a deck directly:

```rust
let deck = oblique::deck_by_id("oblique").expect("compiled in");
println!("{}", deck.random());

for deck in oblique::decks() {
    println!("{} ({} cards)", deck.name, deck.count());
}
```

## Credit

Forked from [ceejbot/oblique](https://github.com/ceejbot/oblique) by C J Silverio, which is where the Oblique Strategies deck, the API shape and the release pipeline all come from. Brian Eno and Peter Schmidt are the originators and copyright holders of the Oblique Strategies card text; the `dramatis` deck reproduces public-domain text from Georges Polti, Vladimir Propp and Aristotle. Every other deck was written for this crate. `SPEC.md` records where each one came from and why.

## See also

[gsv-culture-ships](https://github.com/ceejbot/gsv-culture-ships), the upstream library's structural twin for Iain M. Banks's Culture ship names.

## LICENSE

[ISC](./LICENSE.md), as upstream. Go ahead and steal this one, o freeloading megacorps.
