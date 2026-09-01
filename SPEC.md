# SPEC: multi-deck expansion of `oblique`

Status: **Complete. Parts 1–3 applied, all eight decks built (797 cards), and the CLI
shipped.** What remains is listed under *Still open* at the end, and is deferred by choice
rather than unfinished.

## Context

`oblique` is a Rust library crate plus a three-line CLI. Its entire content is one compile-time
constant — `CANONICAL_STRATEGIES: &[&str]` in `src/lib.rs` — holding the Oblique Strategies deck
amalgamated from several editions of Eno and Schmidt's cards. Six `#[must_use]` functions wrap
it: `strategies_as_slice()`, `strategies()`, `random()`, `random_str()`, `random_n(n)`,
`count()`. One dependency (`fastrand`). No UI, no persistence, no config, no grouping, no card
identity, no i18n. A production-grade release pipeline is already attached
(`.github/workflows/release.yml`).

The goal is to make it a **multi-deck library of the same shape**: several standalone themed
decks, each selectable in parallel with the Eno deck and drawn by the identical random
mechanism. Two things drive the work — a card-by-card curation pass over the original 176
(**done**, below), and a slate of new decks in the self-reflection / existentialism /
creative-writing registers, each with its sources and licence position settled before a line is
written.

Three structural facts constrain everything below:

1. **Cards are a compile-time `const`.** `strategies_as_slice()` and `count()` are `const fn`.
   Runtime-loaded or translated decks are off the table without breaking that, so decks must be
   compiled in — which makes Cargo features the natural selection mechanism.
2. **There is no card identity.** The slice index is the only handle and it is positional.
   Anything needing stable references (favourites, sharing, per-card attribution) has to invent
   one.
3. **`fastrand` is called through its thread-local global** with no seed hook, so a reproducible
   or no-repeat draw would need the RNG plumbed through the API. Out of scope here; noted as a
   deferred decision.

---

## Part 1 — Curation of the original 176 (APPLIED)

**Result: 176 → 156.** The only cut was collapsing variant wordings. Nothing was removed on
grounds of taste, register, or subject matter — no second-guessing Eno and Schmidt about what
belongs in a creative deck.

### The governing principle

Two coherent philosophies were on the table: treat the deck as a **quoted artefact** (don't
touch it; the README defended the amalgam) or as a **tool** (redundancy degrades the draw). The
resolution avoids choosing: cut only what is not a *distinct prompt*. Duplicate ideas fail that
test. Studio cards, opaque cards, and British spellings do not.

### Cut: 19 variant clusters, 40 cards → 20 (−20)

`no_duplicate_entries` only catches exact repeats, so every one of these shipped, and a
`random_n(3)` could hand you two wordings of one idea. Kept wording in bold.

| Cluster | Kept | Cut | Reason |
|---|---|---|---|
| desire | **Abandon desire** | Disconnect from desire | Matches the deck's other "abandon" cards. Closest call in the set. |
| transitions | **Are there sections? Consider transitions** | Consider transitions | The cut card is a strict subset of the kept one |
| ambiguity ↔ specifics | **Change ambiguities to specifics** + **Change specifics to ambiguities** | Remove ambiguities and convert to specifics; Remove specifics and convert to ambiguities | Four cards, two ideas; terser pair keeps both directions |
| consistency | **Change nothing and continue with immaculate consistency** | Change nothing and continue consistently | "Immaculate" is the distinctive word |
| anchor | **Find a safe part and use it as an anchor** | Define an area as 'safe' and use it as an anchor | The quoted 'safe' is an editorial artefact |
| formulas | **Discover your formulas and abandon them** | Discover the recipes you are using and abandon them | Terser, second person |
| talents | **Don't be frightened to display your talents** | Display your talent | Kept card carries the fear; cut card is the flattened instruction |
| sudden action | **Make a sudden, destructive unpredictable action; incorporate** | Do something sudden, destructive and unpredictable | "; incorporate" is the second move |
| easiness | **Don't be afraid of things because they're easy to do** | Don't avoid what is easy | Kept card carries the fear |
| give away | **Give the game away** | Give the name away | Reads as transcription drift, not a second card — see note |
| error | **Honor thy error as a hidden intention** | Honor thy mistake as a hidden intention | The canonical citation |
| perfection | **Make what's perfect more human** | Humanize something free of error | Plainer; the cut card is stilted |
| missing | **Is something missing?** | Is there something missing? | Terser |
| work | **Simply a matter of work** | It is simply a matter of work | Matches the noun-phrase register |
| insignificance | **Towards the insignificant** | Move towards the unimportant | Terser, more distinctive |
| brick | **Not building a wall; making a brick** | Not building a wall but making a brick | Tighter, more commonly cited |
| forgetting | **The most important thing is the thing most easily forgotten** | The most easily forgotten thing is the most important | More widely cited form |
| own ideas | **You don't have to be ashamed of using your own ideas** | Use your own ideas | Kept card carries the content |
| thinking | **What are you really thinking about just now?** | What were you really thinking about just now? | Present tense is the live question |

**Open item — "Give the game away" / "Give the name away."** These differ by one letter and read
like OCR or transcription drift between editions. "game" was kept. If "name" can be confirmed in
a physical edition, it should be restored as its own card.

**Rejected as a cluster:** "Look closely at the most embarrassing details & amplify them" and
"Magnify the most difficult details". *Embarrassing* ≠ *difficult*. Both kept.

### Kept, deliberately

- **Non-instruction cards (2)** — `"Pae White's non-blank graphic metacard"` and
  `"[blank white card]"`. Retained for deck authenticity. **Consequence: card text is not always
  a prompt.** Any consumer that renders a card as an instruction must special-case these; the
  module docs and README now say so.
- **Studio-specific cards (13)** — "Change instrument roles", "Consider different fading
  systems", "Convert a melodic element into a rhythmic element", "Don't break the silence",
  "Feed the recording back out of the medium", "Fill every beat with something", "Left channel,
  right channel, center channel", "Mute and continue", "Put in earplugs", "Spectrum analysis",
  "The tape is now the music", "Think of the radio", "Use fewer notes". Part of the deck's
  character, and most read as metaphor outside a studio. **Not** moved to a `studio` deck.
- **Cards with inline glosses (2)** — "Allow an easement (an easement is the abandonment of a
  stricture)" and the 130-character "Short circuit (example; a man eating peas…)". Kept whole.
  The gloss should move to a note field once `Card` gains one.
- **Opaque cards (3)** — "Idiot glee (?)", "Lost in useless territory", "Lowest common
  denominator". Obliqueness is the point.
- **Source orthography** — US and UK spellings coexist ("Honor", "color" against "judgement");
  "cliches" is unaccented. Left untouched: you don't spell-correct a quoted source. New decks
  get one house convention each.

### Formatting invariant to carry forward

Eight cards are multi-line, in two sub-shapes — bullet (`\n\t-build\n\t-burn`) and plain
continuation (`\n\tIts center`). The `entries_are_tidy` test enforces the tab convention; it
generalises to every new deck and should.

### Remaining small question

"Idiot glee (?)" — the `(?)` is an editor's uncertainty marker, not card text. Left as-is
pending a decision; removing it would not change sort position.

### The paraphrase question

Paraphrasing the Eno deck card-for-card to sidestep copyright is **not recommended**:

- Individual short phrases largely fall below the US originality threshold (Copyright Office
  Circular 33), so "Water" or "Reverse" are not the exposure.
- But a **compilation** is separately protectable in its selection, coordination and arrangement.
  A paraphrase that tracks the deck one-for-one reproduces precisely that selection — it reads
  as a derivative work of the compilation, not as independent creation.
- The status quo — ship the text with clear attribution to Eno and Schmidt, as the README does —
  is what essentially every open implementation does.

**Decision:** keep the deck attributed rather than paraphrased; spend the effort on original
decks that share the deck's *function* rather than its sequence. Judgement call, not legal
advice.

---

## Part 2 — Deck slate

Eight decks including the existing one. Each is standalone and drawn identically.

Provenance is **per-deck, not global**. Three viable modes, and the right one differs by deck:
**original** (written fresh, cleanly ISC), **public-domain** (verbatim from PD texts and PD
*translations* — the translation's date governs, not the author's death date), and
**technique-derived** (methods aren't copyrightable, only their expression; restate in original
words).

| Deck | Register | Provenance | Sources / inspirations |
|---|---|---|---|
| `oblique` | Lateral nudge, terse imperative | Canonical, attributed | Eno & Schmidt, eds. 1–5. **Default deck; unchanged API. 156 cards.** |
| `examen` ✅ | Second-person past-tense interrogative, non-judgemental | **Original** (see below) | Register drawn from — not quoting — the Ignatian examen; the Stoic evening review in Seneca's *De Ira* III.36; Marcus Aurelius on other people's faults; Epictetus on what is ours to move; the Proust Questionnaire's habit of asking a preference to learn a character. **92 cards, max 100 chars.** |
| `absurd` ✅ | Confrontational, second person throughout; finitude, freedom, self-deception | **Original** | Register from Kierkegaard (anxiety, the crowd, deciding without certainty), Nietzsche (would you take this life again), Heraclitus (nothing holds still), Ecclesiastes (being forgotten), Montaigne (how little you know). **Camus and Sartre: themes only, no formulations — enforced by a generator blocklist, not just intended.** **86 cards, max 120 chars.** |
| `constraints` ✅ | Imperative procedure | **Technique-derived** | Oulipo (lipogram, univocalism, snowball, definitional literature, nth-noun substitution); Dada and Surrealist practice (words drawn from a bag, cut-up and fold-in, the folded sheet passed on, unsteered writing); Burroughs–Gysin cut-up. Every card restates a method in its own words; **none quotes a source**. **100 cards, max 120 chars.** |
| `dramatis` ✅ | Generative; **labels, not sentences** | **PublicDomain** — the only sourced deck | Polti's thirty-six situations and their casts (**Ray trans. 1916**); Propp's functions (**1928 original; renderings our own, not from the in-copyright 1958 translation**); Aristotle on reversal and recognition (**Butcher 1895**). One edit: spellings normalized to the crate's US convention. **104 cards, max 90 chars.** |
| `attention` ✅ | Imperative-to-notice; closest to Eno's "Water", "Ghost echoes" | **Original** (the "PD + original" fork, resolved as recommended) | Register from Thoreau watching a pond through a year; haiku; Zhuangzi and the *Tao Te Ching* on the usefulness of what is not there; Sei Shōnagon's lists of things worth noticing. No card quotes any of them. **93 cards, max 60 chars — the tightest ceiling in the crate, and the point of the deck.** |
| `memento` ✅ | Finitude, plain and unconsoling; **declarative only** | **Original** | Register from Marcus Aurelius (only the present can be lost), Seneca (length is not the measure), Montaigne (the practice of thinking about dying), and the *ars moriendi* tradition. No card quotes any of them. **Rilke is untouched** — his letters remain in copyright, and the blocklist guards the famous mortality formulations besides. **76 cards, max 110 chars.** |
| `stuck` ✅ | Eno's territory, worked in the opposite direction: **methodical, not lateral** | **Original** | Descartes on accepting nothing unexamined, dividing a difficulty, enumerating completely (1637); Pólya on restating the problem, solving a simpler one first, working backwards, finding a solved problem that resembles this one. Methods are restatable, their wording is not — a blocklist guards Pólya, de Bono and IDEO phrasings. **90 cards, max 80 chars.** |

### Provenance is one value per deck — "PD + original" is not expressible

Found while building `examen`. `Deck` carries a single `Provenance` and has **no per-card
fields**, so a deck mixing verbatim public-domain quotation with original writing cannot be
labelled truthfully; the quoted cards would need per-card attribution the schema does not have.

`examen` was therefore written **entirely originally**, with the traditions named in the module
doc as sources of *register*, not of text. No card quotes any of them.

The same fork now faces every "PD + original" row above — `attention` and `memento`. Each must
either go fully original, or `Deck` must grow a per-card attribution field. **Going fully
original is recommended**: it keeps the schema flat and matches the decision already taken on
the Eno deck, that the effort belongs in original writing rather than in reproducing sources.

**Voice rules, per deck, enforced by test where mechanisable:** target card count (aim 60–120,
so a deck feels deep but curated), max card length, permitted grammatical moods, and one
orthography convention. The Eno deck's three registers — bare imperative ("Be dirty"),
interrogative ("Is it finished?"), bare noun-phrase ("Accretion") — are the model to hold each
new deck against.

---

## Part 3 — Architecture

Minimum change that makes decks first-class while keeping the crate's shape, its single
dependency, and its `const`-everywhere character.

### Data model

One module per deck under `src/decks/`, each exposing a private `const CARDS: &[&str]` and a
public `Deck`:

```rust
pub struct Deck {
    pub id:         &'static str,   // "oblique", "examen"
    pub name:       &'static str,
    pub blurb:      &'static str,
    pub provenance: Provenance,     // Attributed | Original | PublicDomain | Technique
    pub cards:      &'static [&'static str],
}
```

`Provenance` carries the attribution line so the CLI and docs can print it without a second
table. Deck-scoped methods mirror the existing free functions exactly — `Deck::random()`,
`random_str()`, `random_n(n)`, `count()`, `cards()` — reusing the current bodies verbatim. The
shuffle-indices approach in `random_n` is fine as-is at this scale.

**No breaking change:** the six existing top-level functions stay, delegating to the `oblique`
deck. `strategies_as_slice()` and `count()` stay `const fn`.

Add `decks() -> &'static [&'static Deck]` and `deck_by_id(&str) -> Option<&'static Deck>`.

### Feature flags

`default = ["oblique"]`; one feature per deck; a `full` feature enabling all. Because decks are
`const`, a consumer who wants only `examen` pays nothing for the rest.

**Corrected during implementation:** this spec previously said `decks()` "must not be `const fn`"
because it is assembled under `#[cfg]`. That was wrong — `cfg` resolves at compile time, so
`#[cfg]` on the array elements composes fine with `const fn`. `decks()` is `const fn`, which
keeps the crate's const-everywhere character.

A build with no deck feature at all raises `compile_error!` with a one-line message. The six
top-level Oblique functions are themselves `#[cfg(feature = "oblique")]` — without that gate the
`compile_error!` was followed by a cascade of unresolved-path errors, which is not a clean
failure.

**Three bugs the second deck exposed.** With only one deck, `--no-default-features --features X`
could not actually be exercised. Adding `examen` made it testable, and it failed three ways —
all now fixed:

1. `src/bin/oblique.rs` and `examples/random-strategy.rs` called the feature-gated top-level
   functions, so neither compiled without `oblique`. Both now draw from `decks().first()`, which
   is `oblique` whenever it is enabled, so the default build is unchanged.
2. Nine tests exercising the top-level functions needed `#[cfg(feature = "oblique")]`.
3. Eight doctests hardcoded `deck_by_id("oblique").expect(...)` and panicked in an examen-only
   build. Doctests cannot be feature-gated, so they are now deck-agnostic — the only form true
   under every supported feature set.

The feature matrix is now part of verification, not a claim: **default 20 unit / 16 doc;
`oblique` only 20 / 16; each new deck alone 11 / 10, except `dramatis` at 12 / 10 (it carries
the extra attribution test); `--all-features` 70 / 16; no deck 1 clean error.** Run every row when adding a deck — a
deck-only build is the configuration that catches assumptions about `oblique` being present.

### Card identity — deferred, deliberately

`(deck_id, index)` is the cheapest handle but is unstable across edits. Rather than adding an ID
field now, enforce **alphabetical sort order per deck** with a test; that makes indices
predictable and diffs reviewable. Revisit only if favourites, history, or sharing land later.

**Caveat found during curation:** the deck is sorted by *displayed* text, not by literal. A
naive `is_sorted()` test fails on `"Think\n\t-inside the work…"`, which sorts by its escape
sequence and lands after `"Think of the radio"`. This predates the curation cut. The sort test
must compare on the first line of each card, not the raw literal.

### Tests

Generalise the four existing invariants into a `deck_invariants!(module, expected_count)` macro
applied to every deck — `no_empty_entries`, `entries_are_tidy` (the tab convention),
`no_duplicate_entries`, exact count. Add: unique deck ids across the registry; sorted order per
deck; per-deck max-length and orthography checks. Every public item carries a doctest today; new
ones must too.

### CLI

Hand-rolled `std::env::args()` parsing, **no new dependency** — `clap` would undo the tuned
tiny-binary profile (`lto`, `codegen-units = 1`, `strip`, `panic = "abort"`).

```
oblique                  # random card from the default deck
oblique examen           # random card from a named deck
oblique examen -n 3      # three, without replacement, either order
oblique --list           # deck ids, card counts, names; * marks the default
oblique --about examen   # blurb, count, and where the text came from
oblique --help           # the grammar
oblique --version
```

Shipped as sketched, plus `--help`/`--version` and `-l`/`-n`/`--count` aliases. The default
deck is `decks().first()`, not a hardcoded `oblique`, so the binary works under any feature
set — an `examen`-only build lists and draws `examen`, and `oblique examen ... oblique` there
fails with `no deck called oblique. Built in: examen`. Usage errors and unknown decks exit 1;
the unknown-deck message names the decks that *are* compiled in.

Preserve `println!` of the raw string so the `\n\t` convention keeps rendering for free.

### Files

- ✅ `src/lib.rs` — constant moved out; six functions kept as delegates; `Deck`, `Provenance`,
  `decks()`, `deck_by_id()` added.
- ✅ `src/decks/mod.rs` — registry and `#[cfg]` wiring.
- ✅ `src/decks/oblique.rs` — the curated 156, moved verbatim (extracted mechanically, not
  retyped).
- ✅ `Cargo.toml` — `[features]`.
- ☐ `src/decks/{examen,absurd,constraints,dramatis,attention,memento,stuck}.rs` — new.
- ☐ `src/bin/oblique.rs` — arg parsing. Still the original one-liner.
- ☐ `README.md` — deck table, per-deck attribution, provenance policy. Currently carries only a
  short note that decks are feature-gated.

---

## Sequencing

1. ~~**Curation pass** over the 176.~~ **Done — 156 cards, decisions recorded above.**
2. ~~**Architecture refactor** with the single curated deck.~~ **Done — `Deck`, `Provenance`,
   `decks()`, `deck_by_id()`, per-deck feature flags, `deck_invariants!` test macro. The six
   top-level functions kept their doc comments byte-identical, verified by diff.**
3. ~~**One deck end-to-end** — `examen`.~~ **Done — 92 cards, `Provenance::Original`, generated
   by a script that validates every invariant before writing the file. Building it as the second
   deck is what exposed three latent bugs in the step-2 architecture; see below.**
4. **Remaining decks**, one per change, each with its sources recorded. **Done — all
   seven new decks built.** `constraints` (100), `absurd` (86), `attention` (93), `memento`
   (76), `dramatis` (104) and `stuck` (90) each landed without touching the architecture, which
   is the evidence the step-2 design was right.

   **All four `Provenance` variants are now exercised**, so none is dead code: `Attributed`
   (`oblique`), `Original` (`examen`, `absurd`, `attention`, `memento`), `Technique`
   (`constraints`), and `PublicDomain` (`dramatis`). A test asserts `dramatis` carries a credit
   line, since it is the only deck whose text is not ours.

   **Each deck's voice rules are enforced by its generator, not just described here.** The
   generator exits non-zero rather than write a bad file — on `absurd` it caught four cards that
   had drifted out of direct address, which were rewritten rather than the rule loosened.

   The rules are what keep the decks from blurring into each other. They now separate cleanly:

   | Deck | Max | Mood | Person | Also enforced |
   |---|---|---|---|---|
   | `oblique` | 127 | mixed | — | inherited, not authored |
   | `examen` | 100 | interrogative | second | US orthography |
   | `constraints` | 120 | imperative | — | no question marks; single-line cards end in a period |
   | `absurd` | 120 | mixed | **second, every card** | blocklist of in-copyright formulations |
   | `attention` | **60** | imperative / noun-phrase | **none — points outward** | no question marks |
   | `memento` | 110 | **declarative only** | any | no question marks; no imperative openers; blocklist |
   | `dramatis` | 90 | **labels, not sentences** | none | no terminal punctuation at all; no questions |
   | `stuck` | 80 | **asks or instructs, never states** | any | every card is a question or opens with an imperative |

   Two pairs were at risk of collapsing together, and each is separated by a rule rather than
   by taste:

   - **`attention` vs `absurd`** — both short and declarative. The person rule splits them:
     `absurd` must say "you" in every card, `attention` may never say it. Checked across the
     built decks: 86 of 86 `absurd` cards contain it, 0 of 93 `attention` cards do.
   - **`memento` vs `absurd`** — both deal in finitude. *Mood* splits them: `absurd` provokes,
     asking and instructing; `memento` only states. `memento` carries 0 question marks against
     `absurd`'s 15, and refuses any card opening with an imperative verb.

   **Adding a deck touches exactly five places**, and nothing else: a generator script that
   validates before emitting, `src/decks/<id>.rs`, a `pub mod` line in `src/decks/mod.rs`, a
   feature in `Cargo.toml` (plus the `full` list and the `compile_error!` cfg), an entry in
   `decks()`, and a `deck_invariants!` line with the deck's count and max length.
5. ~~**CLI and README** last, once the deck set is stable.~~ **Done.** The grammar is a pure
   `parse()` function over the arguments with 14 unit tests, so the CLI is testable without
   spawning a process. `--help` and `--version` were added beyond the sketch below; a CLI that
   takes arguments and cannot explain them is not finished. No new dependency: the crate still
   has exactly one.

## Verification

```bash
cargo nextest run --all-targets --all-features --future-incompat-report
```

- `cargo clippy --all-targets --all-features -- -D warnings` — note `clippy::unwrap_used` is
  `deny` at the crate level.
- `cargo +nightly fmt --check --all` — CI runs nightly rustfmt against `.rustfmt.toml`.
- `cargo test --doc`.
- Feature-matrix build: `cargo build --no-default-features --features examen`; and
  `--no-default-features` alone must fail cleanly or default sensibly.
- Backwards compatibility: the existing doctests on `random()`, `strategies()`, `count()` must
  pass **unmodified** — that is the proof the refactor is non-breaking.
- CLI smoke: `cargo run --bin oblique -- --list`, then one draw per deck, checking multi-line
  cards still render with their tab indentation.

## Open questions

- **Deck slate** — eight is a proposal, not a commitment. Which are worth building?
- **Provenance mode per deck** — the table recommends one each; `absurd` and `stuck` are
  original-authorship-heavy and therefore the slowest to produce.
- **"Give the name away"** — restore if confirmed in a physical edition.
- **"Idiot glee (?)"** — drop the editorial `(?)` or keep it.

---

## The built decks

| Deck | Cards | Longest | Questions | Provenance |
|---|---:|---:|---:|---|
| `oblique` | 156 | 127 | 23 | `Attributed` — Eno & Schmidt |
| `examen` | 92 | 68 | 83 | `Original` |
| `constraints` | 100 | 91 | 0 | `Technique` — Oulipo, Dada, Surrealist |
| `absurd` | 86 | 108 | 15 | `Original` |
| `attention` | 93 | 51 | 0 | `Original` |
| `memento` | 76 | 86 | 0 | `Original` |
| `dramatis` | 104 | 75 | 0 | `PublicDomain` — Polti, Propp, Aristotle |
| `stuck` | 90 | 69 | 30 | `Original` |
| **total** | **797** | | | |

The question counts are not a target; they fall out of the enforced voice rules, and they are
one way to see that the decks did not blur together. `examen` is nearly all questions,
`constraints` and `dramatis` have none by rule, and `stuck` sits in between because it may
either ask or instruct.

## Still open

Nothing here is unfinished work; each is a decision taken deliberately.

1. **Two questions about the Eno deck.** Restore "Give the name away" if it can be confirmed in
   a physical edition. Decide whether to drop the editorial `(?)` from "Idiot glee (?)".
2. **Card identity.** `(deck_id, index)` remains the only handle. Nothing needs a stable one
   until favourites, history or sharing arrive; alphabetical order per deck keeps indices
   predictable in the meantime.
3. **A seedable RNG.** `fastrand`'s thread-local global is still called directly, so there is no
   reproducible draw and no no-repeat-until-exhausted. Both would need the generator plumbed
   through `Deck`.
4. **A per-card note field**, which the two Eno cards carrying inline glosses want.
5. **i18n**, which the compile-time `const` blocks outright. Translating would mean giving up
   `const fn` on `strategies_as_slice()` and `count()` — the one change in this project that
   would actually break the public API.
