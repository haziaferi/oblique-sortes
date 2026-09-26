# SPEC: oblique-sortes

Status: **Complete. Parts 1–3 applied, all eight decks built (797 cards), and the CLI
shipped.** What remains is listed under *Still open* at the end: what is deferred by choice,
what is genuinely open and waiting, and what no device walk has reached yet.

**A note on names.** This document was written against the crate as received, which was called
`oblique`. It now ships as the package **`oblique-sortes`**, with the library and binary both
named **`sortes`** — upstream holds `oblique` on crates.io, and the name over-claimed anyway,
since only 156 of 797 cards are Oblique Strategies. Where `oblique` still appears below it means
one of three other things, none of which moved: the deck id, the Cargo feature that compiles
that deck in, or the `decks::oblique` module.

## Context

The starting point was ceejbot's `oblique`, a Rust library crate plus a three-line CLI. Its
entire content was one compile-time constant — `CANONICAL_STRATEGIES: &[&str]` in `src/lib.rs` —
holding the Oblique Strategies deck amalgamated from several editions of Eno and Schmidt's
cards. Six `#[must_use]` functions wrapped it: `strategies_as_slice()`, `strategies()`,
`random()`, `random_str()`, `random_n(n)`, `count()`. One dependency (`fastrand`). No UI, no
persistence, no config, no grouping, no card identity, no i18n. A production-grade release
pipeline was already attached (`.github/workflows/release.yml`).

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

**Result: 176 → 156.** The only cut was collapsing variant wordings. No card was removed on
grounds of taste, register, or subject matter — no second-guessing Eno and Schmidt about what
belongs in a creative deck. Register does decide *which* of a collapsed pair survives, and the
table below gives that reason openly; what it never decides is whether a pair collapses at all,
which is the "not a distinct prompt" test and nothing else.

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
  stricture)" and the 127-character "Short circuit (example; a man eating peas…)". Kept whole.
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
  is what the implementations looked at while deciding this all did, this crate's own upstream
  included. No survey was run; it is an impression from a handful, not a count.

**Decision:** keep the deck attributed rather than paraphrased; spend the effort on original
decks that share the deck's *function* rather than its sequence. Judgement call, not legal
advice.

---

## Part 2 — Deck slate

Eight decks including the existing one. Each is standalone and drawn identically.

Provenance is **per-deck, not global**. Four modes, and the right one differs by deck:
**original** (written fresh, cleanly ISC), **public-domain** (verbatim from PD texts and PD
*translations* — the translation's date governs, not the author's death date),
**technique-derived** (methods aren't copyrightable, only their expression; restate in original
words), and **attributed** — text that is neither original nor public domain, carried with its
authors named. Only `oblique` is attributed, and it is why the mode exists: the plan was written
with three and the deck it was written for needed a fourth.

| Deck | Register | Provenance | Sources / inspirations |
|---|---|---|---|
| `oblique` | Lateral nudge, terse imperative | Canonical, attributed | Eno & Schmidt, eds. 1–5. **Default deck; unchanged API. 156 cards.** |
| `examen` ✅ | Second-person past-tense interrogative, non-judgemental | **Original** (see below) | Register drawn from — not quoting — the Ignatian examen; the Stoic evening review in Seneca's *De Ira* III.36; Marcus Aurelius on other people's faults; Epictetus on what is ours to move; the Proust Questionnaire's habit of asking a preference to learn a character. **92 cards, max 100 chars.** |
| `absurd` ✅ | Confrontational, second person throughout; finitude, freedom, self-deception | **Original** | Register from Kierkegaard (anxiety, the crowd, deciding without certainty), Nietzsche (would you take this life again), Heraclitus (nothing holds still), Ecclesiastes (being forgotten), Montaigne (how little you know). **Camus and Sartre: themes only, no formulations** — held at authoring time by a blocklist in the generator, which is not in this repo. **86 cards, max 120 chars.** |
| `constraints` ✅ | Imperative procedure | **Technique-derived** | Oulipo (lipogram, univocalism, snowball, definitional literature, nth-noun substitution); Dada and Surrealist practice (words drawn from a bag, cut-up and fold-in, the folded sheet passed on, unsteered writing); Burroughs–Gysin cut-up. Every card restates a method in its own words; **none quotes a source**. **100 cards, max 120 chars.** |
| `dramatis` ✅ | Generative; **labels, not sentences** | **PublicDomain** — the only deck written here whose text is sourced | Polti's thirty-six situations and their casts (**Ray trans. 1916**); Propp's functions (**1928 original; renderings our own, not from the in-copyright 1958 translation**); Aristotle on reversal and recognition (**Butcher 1895**). One edit: spellings normalized to the crate's US convention. **104 cards, max 90 chars.** |
| `attention` ✅ | Imperative-to-notice; closest to Eno's "Water", "Ghost echoes" | **Original** (the "PD + original" fork, resolved as recommended) | Register from Thoreau watching a pond through a year; haiku; Zhuangzi and the *Tao Te Ching* on the usefulness of what is not there; Sei Shōnagon's lists of things worth noticing. No card quotes any of them. **93 cards, max 60 chars — the tightest ceiling in the crate, and the point of the deck.** |
| `memento` ✅ | Finitude, plain and unconsoling; **declarative only** | **Original** | Register from Marcus Aurelius (only the present can be lost), Seneca (length is not the measure), Montaigne (the practice of thinking about dying), and the *ars moriendi* tradition. No card quotes any of them. **Rilke is untouched** — his letters remain in copyright, and the generator's blocklist guarded the famous mortality formulations besides. **76 cards, max 110 chars.** |
| `stuck` ✅ | Eno's territory, worked in the opposite direction: **methodical, not lateral** | **Original** | Descartes on accepting nothing unexamined, dividing a difficulty, enumerating completely (1637); Pólya on restating the problem, solving a simpler one first, working backwards, finding a solved problem that resembles this one. Methods are restatable, their wording is not — a blocklist in the generator guarded Pólya, de Bono and IDEO phrasings. **90 cards, max 80 chars.** |

### Provenance is one value per deck — "PD + original" is not expressible

Found while building `examen`. `Deck` carries a single `Provenance` and has **no per-card
fields**, so a deck mixing verbatim public-domain quotation with original writing cannot be
labelled truthfully; the quoted cards would need per-card attribution the schema does not have.

`examen` was therefore written **entirely originally**, with the traditions named in the module
doc as sources of *register*, not of text. No card quotes any of them.

The same fork faced every "PD + original" row above — `attention` and `memento`. Each had to
either go fully original, or `Deck` had to grow a per-card attribution field. **Going fully
original was recommended and is what shipped**: it keeps the schema flat and matches the decision
already taken on the Eno deck, that the effort belongs in original writing rather than in
reproducing sources. Both decks carry `Original`, and `Deck` never grew the field.

**Voice rules, per deck, enforced by test where mechanisable:** target card count for a deck
written here (aim 60–120, so a deck feels deep but curated — `oblique` is inherited at 156 and
is not held to it), max card length, permitted grammatical moods, and one orthography
convention. The Eno deck's three registers — bare imperative ("Be dirty"),
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

**No breaking change:** the six top-level functions that existed when this was written stay,
delegating to the `oblique` deck — the surface has since grown to ten, additively. `strategies_as_slice()` and `count()` stay `const fn`.

Add `decks() -> &'static [&'static Deck]` and `deck_by_id(&str) -> Option<&'static Deck>`.

### Feature flags

`default = ["oblique"]`; one feature per deck; a `full` feature enabling all. Because decks are
`const`, a consumer who wants only `examen` pays nothing for the rest.

**Corrected during implementation:** this spec previously said `decks()` "must not be `const fn`"
because it is assembled under `#[cfg]`. That was wrong — `cfg` resolves at compile time, so
`#[cfg]` on the array elements composes fine with `const fn`. `decks()` is `const fn`, which
keeps the crate's const-everywhere character.

A build with no deck feature at all raises `compile_error!` with a one-line message. The seven
top-level functions that delegate to the Oblique deck are themselves
`#[cfg(feature = "oblique")]` — without that gate the
`compile_error!` was followed by a cascade of unresolved-path errors, which is not a clean
failure.

**Three bugs the second deck exposed.** With only one deck, `--no-default-features --features X`
could not actually be exercised. Adding `examen` made it testable, and it failed three ways —
all now fixed:

1. `src/bin/sortes.rs` (named `oblique.rs` at the time) and `examples/random-strategy.rs`
   called the feature-gated top-level functions, so neither compiled without `oblique`. Both
   now draw from `decks().first()`, which is `oblique` whenever it is enabled, so the default
   build is unchanged.
2. Nine tests exercising the top-level functions needed `#[cfg(feature = "oblique")]`.
3. Eight doctests hardcoded `deck_by_id("oblique").expect(...)` and panicked in an examen-only
   build. Doctests cannot be feature-gated, so they are now deck-agnostic — the only form true
   under every supported feature set.

The feature matrix is part of verification, not a claim. Counts below are lib unit tests /
doctests, measured on rustc 1.92; the 26 CLI tests in `src/bin/sortes.rs` are the same in
every configuration.

| Configuration | lib | doc |
|---|---:|---:|
| default, which is `oblique` alone — the same set either way | 41 | 38 |
| `examen`, `constraints`, `absurd` or `stuck` alone | 30 | 31 |
| `attention` or `memento` alone — each carries one voice test | 31 | 31 |
| `dramatis` alone — a voice test and the attribution test | 32 | 31 |
| `--all-features` | 102 | 38 |
| no deck feature | — | one clean `compile_error!` |

The gaps between the rows are the feature gating, and they add up: `oblique` alone carries
eleven more lib tests than a bare deck (ten top-level ones plus the pinned derivation) and seven
more doctests — not one per top-level function, of which there are ten; the seven are the
doctests that name a card or a count and so only run with `oblique` compiled in. `tests/cli.rs` runs the binary itself and is counted
in neither column.

Run every row when adding a deck — a deck-only build is the configuration that catches
assumptions about `oblique` being present. The numbers go stale the moment a test is added,
so treat them as a snapshot: `just features` and the CI feature-matrix job are what actually
hold the matrix.

### Card identity — deferred, deliberately

`(deck_id, index)` is the cheapest handle but is unstable across edits. Rather than adding an ID
field now, enforce **alphabetical sort order per deck** with a test; that makes indices
predictable and diffs reviewable. Revisit only if favourites, history, or sharing land later.

**Caveat recorded during curation, and wrong:** it said a naive `is_sorted()` fails on
`"Think\n\t-inside the work…"`, which "sorts by its escape sequence and lands after
`"Think of the radio"`". It lands *before* it, and always would have: a continuation opens with
`\n`, 0x0A, which is below every character a first line can end on — below the space in "Think
of", so the shorter first line wins. The deck is in plain string order today and a naive
`is_sorted()` passes on it. The sort test compares first lines anyway, which is the property
actually wanted and which the raw order happens to match; §Android records the same arithmetic
from the other side, where it is what lets Kotlin's `sorted()` reproduce a deck's own order.

### Tests

Generalise the four existing invariants into a `deck_invariants!(module, expected_count)` macro
applied to every deck — `no_empty_entries`, `entries_are_tidy` (the tab convention),
`no_duplicate_entries`, exact count. Add: unique deck ids across the registry; sorted order per
deck; per-deck max-length and orthography checks. Every public item carries a doctest today; new
ones must too.

A deck's voice rules belong in the tests, not only in its module doc. Where a rule is
machine-checkable it is checked: `attention` addresses no reader and asks nothing, `memento`
never asks, and `dramatis` cards stay labels — no terminal punctuation, no questions, no second
person. The rules that are not checkable (`memento` opening on an imperative, `stuck` asking or
instructing, `absurd` avoiding a borrowed phrasing) are named in the docs as editorial, so no
comment claims an enforcement that does not exist.

### CLI

Hand-rolled `std::env::args()` parsing, **no new dependency** — `clap` would undo the tuned
tiny-binary profile (`lto`, `codegen-units = 1`, `strip`, `panic = "abort"`).

```
sortes                  # random card from the default deck
sortes examen           # random card from a named deck
sortes examen -n 3      # three, without replacement, either order
sortes --list           # deck ids, card counts, names; * marks the default
sortes --about examen   # blurb, count, and where the text came from
sortes --help           # the grammar
sortes --version
```

Shipped as sketched, plus `--help`/`--version` and `-l`/`-n`/`--count` aliases. The default
deck is `decks().first()`, not a hardcoded `oblique`, so the binary works under any feature
set: an `examen`-only build lists and draws `examen`, and asking that build for a deck it does
not have — `sortes oblique` — fails with `no deck called oblique. Built in: examen`. Usage
errors and unknown decks exit 1; the unknown-deck message names the decks that *are* compiled
in.

Preserve `println!` of the raw string so the `\n\t` convention keeps rendering for free.

### Files

- ✅ `src/lib.rs` — constant moved out; six functions kept as delegates; `Deck`, `Provenance`,
  `decks()`, `deck_by_id()` added.
- ✅ `src/decks/mod.rs` — registry and `#[cfg]` wiring.
- ✅ `src/decks/oblique.rs` — the curated 156, moved verbatim (extracted mechanically, not
  retyped).
- ✅ `Cargo.toml` — `[features]`, plus the rename: package `oblique-sortes`, `[lib]` and
  `[[bin]]` both `sortes`.
- ✅ `src/decks/{examen,absurd,constraints,dramatis,attention,memento,stuck}.rs` — all seven
  built.
- ✅ `src/bin/sortes.rs` — argument parsing, hand-rolled. Renamed from `src/bin/oblique.rs`.
- ✅ `src/shoe.rs` — `Shoe`: draws that do not repeat, and draws that can be replayed. The only
  state the crate holds between calls, and the only place a generator is owned rather than
  borrowed from the thread-local one.
- ✅ `tests/cli.rs` — the binary run end to end: stdout, stderr and exit codes. The unit tests
  beside `parse` cover the grammar; nothing covered the program until this.
- ✅ `examples/no-repeat.rs` — the `Shoe`, dealing a whole deck and then replaying a seed.
- ✅ `completions/` — bash, zsh and fish. Deck names are read from `sortes --list` rather than
  copied, so a single-deck build completes a single deck.
- ✅ `android/app/proguard-rules.pro` — the release build shrinks, and R8 matches the JNI
  methods by name.
- ✅ `src/identity.rs` — `CardId`, and the derivation pinned by a test.
- ✅ `android/app/src/test/kotlin/` — the app's first tests. The decoding and the shoe are plain
  Kotlin over strings, so they run on the JVM with no device; `Native.kt` splits each native call
  from the parsing around it for exactly that reason.
- ✅ `tools/spec_check.py` — every file, symbol and number this document, `README.md`,
  `android/README.md` and `completions/README.md` state, checked against the tree on every push.
- ✅ `tools/r8_check.py` — the shipped DEX read back, every JNI method's name found in it. The
  same check CI runs, as a script so it runs locally too.
- ✅ `android/app/src/test/kotlin/dev/feridottir/sortes/ManifestAgreementTest.kt` — the manifest,
  the widget's provider XML and the Kotlin held to the facts they each state twice.
- ✅ `android/app/src/main/kotlin/dev/feridottir/sortes/WidgetBox.kt` — how much of a card the
  widget's box can hold, as arithmetic over a height rather than a comment in the provider XML.
  The third file the JVM suite reaches, and the reason a widget may now be made small.
- ✅ `README.md` — deck table, per-deck attribution, provenance policy, install instructions.
- ✅ `.github/workflows/release.yml` — `BINARY_NAME` follows `[[bin]]`, since that is what the
  pipeline uploads and hands to the Homebrew tap.

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
   (`oblique`), `Original` (`examen`, `absurd`, `attention`, `memento`, `stuck`), `Technique`
   (`constraints`), and `PublicDomain` (`dramatis`). The text of `oblique` and of `dramatis` is not ours, and
   each carries that differently: `oblique` is Eno and Schmidt's, which is what `Attributed` says,
   and `dramatis` is out of public-domain sources, which is what `PublicDomain` says. A test
   asserts only the second carries a credit line, because only the second's sources need one.

   **Each deck's voice rules were enforced by its generator at the time the deck was built.**
   The generator exited non-zero rather than write a bad file — on `absurd` it caught four
   cards that had drifted out of direct address, which were rewritten rather than the rule
   loosened. Those generators were one-off scripts run outside this repo and are not in it, so
   nothing re-runs them and nothing here can. The rules a test can judge became crate tests
   instead (see *Tests* above); the rest are editorial, and the module docs say so.

   The rules are what keep the decks from blurring into each other. They now separate cleanly:

   | Deck | Max | Mood | Person | Also enforced |
   |---|---|---|---|---|
   | `oblique` | 130 | mixed | — | inherited, not authored; the ceiling sits just over its 127-character "Short circuit" card |
   | `examen` | 100 | interrogative | second | US orthography |
   | `constraints` | 120 | imperative | — | no question marks; single-line cards end in a period |
   | `absurd` | 120 | mixed | **second, every card** | blocklist of in-copyright formulations, at authoring time |
   | `attention` | **60** | imperative / noun-phrase | **none — points outward** | no question marks |
   | `memento` | 110 | **declarative only** | any | no question marks; no imperative openers; blocklist, at authoring time |
   | `dramatis` | 90 | **labels, not sentences** | none | no terminal punctuation at all; no questions |
   | `stuck` | 80 | **asks or instructs, never states** | any | every card is a question or opens with an imperative |

   Two pairs were at risk of collapsing together, and each is separated by a rule rather than
   by taste:

   - **`attention` vs `absurd`** — both short and declarative. The person rule splits them:
     `absurd` must say "you" in every card, `attention` may never say it. Both halves are crate
     tests now. Only the `attention` half was, which left the rule that separates the two decks
     enforced in one direction and asserted in the other; the 86-of-86 count that stood in for
     the `absurd` half came from a generator this repo does not carry and could not re-run.
   - **`memento` vs `absurd`** — both deal in finitude. *Mood* splits them: `absurd` provokes,
     asking and instructing; `memento` only states. `memento` carries 0 question marks against
     `absurd`'s 15, and refuses any card opening with an imperative verb.

   **Adding a deck touches four files and nothing else**: `src/decks/<id>.rs`, a `pub mod`
   line in `src/decks/mod.rs`, a feature in `Cargo.toml` with the `full` list beside it, and
   `src/lib.rs` three times over — the `compile_error!` cfg list, an entry in `decks()`, and a
   `deck_invariants!` line with the deck's count and max length.
   How the card text gets written is your business; the seven written here came from one-off
   generators that this repo does not carry, and `oblique` came from the curation pass in
   Part 1 rather than from a generator at all.
5. ~~**CLI and README** last, once the deck set is stable.~~ **Done.** The grammar is a pure
   `parse()` function over the arguments with 26 unit tests, so the CLI is testable without
   spawning a process. `--help` and `--version` were added beyond the sketch below; a CLI that
   takes arguments and cannot explain them is not finished. No new dependency: the crate still
   has exactly one.

## Verification

```bash
cargo nextest run --locked --all-targets --all-features --future-incompat-report
```

- `cargo clippy --all-targets --all-features -- -D warnings` — note `clippy::unwrap_used` is
  `deny` at the crate level.
- `cargo +nightly fmt --check --all` — CI runs nightly rustfmt against `.rustfmt.toml`.
- `cargo test --doc` — a separate CI step, because nextest does not run doctests.
- Feature-matrix build: each deck alone under `--no-default-features --features <deck>`, and
  `--no-default-features` alone must fail with the `compile_error!` message. Both are a CI job
  and a `just features` recipe.
- Backwards compatibility: the existing doctests on `random()`, `strategies()`, `count()` must
  pass **unmodified** — that is the proof the refactor is non-breaking.
- CLI smoke: `cargo run --bin sortes -- --list`, then one draw per deck, checking multi-line
  cards still render with their tab indentation.

## Open questions

All answered. The deck slate settled at the eight below, each with the provenance the table
records. The two remaining questions about the Eno deck were never about this plan and are
carried in **Still open**, which is the live list.

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

Two kinds of thing, and they are not the same kind: decisions taken deliberately, which need
nothing from anyone, and open actions, which are waiting on something.

**Deferred by choice.**

1. **A per-card note field**, which the two Eno cards carrying inline glosses want.
2. **i18n**, which the compile-time `const` blocks outright. Translating would mean giving up
   `const fn` on `strategies_as_slice()` and `count()` — the one change in this project that
   would actually break the public API.

**Open, and waiting.**

3. **Restore "Give the name away"** if it can be confirmed in a physical edition. Waiting on a
   copy of the deck, which no amount of work here supplies.
4. **Decide whether to drop the editorial `(?)`** from "Idiot glee (?)". Waiting on a judgement
   that has not been made. Dropping it changes a card's text, and so its [`CardId`], which is
   what a kept card is stored as — the decision is cheap and its consequence is not.

### Not yet walked

The device walks closed what they reached; these were never reached, and a list beats a memory.

- **TalkBack has never been run on this app.** Every accessibility claim in this document and in
  `android/README.md` is read from the code, not heard. The widget's `contentDescription` is the
  one to hear first: a description *replaces* a RemoteViews text rather than adding to it, which
  is why the deck name had to be written into it, and only a screen reader says whether that
  reads well twice over.
- **The spinner's dropdown and the dialog list**, whose row height for 156 multi-line items the
  critique could not verify from the sources and no walk has measured since.
- **A fresh widget placement.** The widget on the phone was dragged to one row during the walk
  that produced `minResizeHeight`; what a launcher gives a widget placed after that change has
  not been seen.

### Closed since

**Card identity.** Was deferred with a condition attached — "nothing needs a stable one until
favourites, history or sharing arrive" — and the app growing kept cards is that condition being
met, so it was built rather than deferred again. Not the `(deck_id, index)` the plan weighed and
rejected, and not a written id per card either, which would have meant editing all eight decks
and keeping 797 ids unique by hand. [`CardId`] is derived: FNV-1a over the deck id, a zero byte,
and the card's own text. It survives cards being added, removed or reordered around it, and it
changes when the card is reworded — which is the behaviour wanted, because a reworded card is a
different card and whatever saved the old one should say so rather than substitute.

A test asserts one literal id, because ids get written down outside this crate and changing how
they are derived would invalidate every one of them silently. That test failing means the change
was not a refactor.


**The shim's panic guarantee was not being kept.** Not on the open list, because nobody had
noticed it. `[profile.release]` at the workspace root sets `panic = "abort"`, Cargo applies a
profile to every member, and the Gradle build asked for `--release` — so the shim's
`catch_unwind` calls caught nothing, and `android/README.md` said in writing that they did. The
shim now has its own `[profile.android]`, and a `#[cfg(panic = "abort")] compile_error!` so it
cannot be built any other way. The claim is enforced rather than asserted, which is the rule
this project holds everything else to.


**A seedable RNG.** Was: "`fastrand`'s thread-local global is still called directly, so there is
no reproducible draw and no no-repeat-until-exhausted. Both would need the generator plumbed
through `Deck`." Both now exist, and the generator was not plumbed through `Deck` — it is owned
by `Shoe`, so `Deck` stays a compile-time constant and `fastrand` stays out of the public API.
`Deck::shoe()` and `Deck::shoe_with_seed()` are the two ways in, and the CLI's `--seed` is the
same mechanism from outside. The seam between passes is handled: a card that ends one pass is
moved aside rather than opening the next, which a test checks across sixty-four seeds.

A seed is only as stable as the decks. The same seed deals the same cards for as long as the
deck holds the same cards; one added or reworded reshuffles every seed with it. That is stated
in the API docs, in `--help` and in the README rather than promised away.


**Three JNI entry points were shipping unverified against R8.** The CI step that reads the
shipped DEX was written when the surface was two calls, and it named them: `for method in decks
draw`. Adding the shoe and the card ids grew the surface to five — the search added nothing to it,
since `cardsMatching` runs in Kotlin over a list already in hand — and the loop stayed as it was — so `shuffled`, `cardId` and `cardById` were covered by nothing, while four places in
prose went on saying "two". The failure that step exists to catch is invisible at build time and
fatal on the device, so it must not depend on a list anyone has to remember to extend: it now
reads the method names out of `Native.kt`, and grows with the surface. The keep rule in
`proguard-rules.pro` was already a wildcard over the class and so had not gone stale with it,
which is why nothing broke while the check was blind.

**Two symbols were alive only in their own tests.** `CardShoe.remaining` was read by nothing
outside the tests of `restore`; it is gone, and those tests observe `cursor()` instead, which
`onSaveInstanceState` genuinely writes down. Removing it also cost nothing in coverage — the test
named "remaining counts down and wraps" never reached the wrap, and its replacement does.
`FoundCard.deckId` was the more interesting one: it crossed the JNI boundary, round-tripped in a
test, and was then dropped. Kept cards are held in one list across every deck, so the kept-cards
dialog was showing cards from different decks with nothing to tell them apart. It labels them
now, which is what the field was carrying the deck id for all along.

**The app contradicted the library on an empty search.** `Deck::find` documents that an empty
needle matches every card, `tests/cli.rs` holds the CLI to it, and the app returned nothing and
said "Nothing matched". Fixing it also closed the app's only missing CLI verb: an empty box is
how a deck is now read whole, which is what `--all` does, so no fifth button was needed for a row
that has room for four. The matching moved out of `MainActivity` into `Native.kt` as
`cardsMatching`, because the JVM suite reaches that file and does not reach an Activity — measured,
not assumed. Sorting there reproduces the deck's own order exactly: the crate sorts by first line,
and a continuation opens with a newline, which is below every character a first line can end on.
Checked across all eight decks and all thirty-three multi-line cards.

**`just setup` and `just version` were Homebrew-only.** Both are now toolless. `setup` installs
nextest with `cargo install`, which is the one package manager the project already requires and
the same command on all three platforms; `version` does the semver arithmetic in the shell rather
than through `tomato` and `semver-bump`, which put cutting a release out of reach on Windows. The
`sed` writes to a new file and moves it rather than using `-i`, because BSD sed — which is what
macOS ships — reads the argument after `-i` as a backup suffix. All three bump kinds were run
against a copy of the real manifest and produce 0.2.1, 0.3.0 and 1.0.0, changing that one line and
nothing else.

**The first device walk, on the OnePlus 9 Pro (Android 14, 360×804dp).** Three defects no
reconstruction could show, all measured from screenshots rather than judged. Every line the
Activity built in code was `textColorSecondary`, because a bare `TextView` takes the default
text appearance: `#837274` on the `#FEEDEE` ground for the title (4.0:1), the same grey on the
card's own surface — the theme's ink at six percent over that ground, `#F1E0E1` — for the card
(3.6:1), and 2.5:1 for the three dimmed lines, which sit on the ground rather than the card and
carry the app's 0.72 alpha over it. All three against an assumed 16.7:1. `text()` now sets `textColorPrimary`; measured
after: 15.2, 13.5 and 6.3:1. The status bar drew white on the light ground — `#FFFFFF` on `#FEEDEE`, 1.1:1, which is as
close to invisible as two colours get — because edge-to-edge leaves the icon colour to the
window; it follows the ground's luminance now (15.8:1).
And in landscape the card was 10dp tall, the fixed chrome having taken the 360dp; below 480dp of
height the secondary lines yield. What the walk confirmed as built: "Saved" on one line at 72dp,
every button 48dp, the empty-search browse titled by deck and in `--all` order, Keep→Kept and the
Saved list labelling each card's deck, long-press copying (pasted back into Find), the card
surviving a night-mode recreate and a rotation, dark mode at 15.2/8.3/13.2:1.

**The widget, walked once a hand had placed it.** No `adb` can drop a widget on a launcher, so
this waited for one. What held: the deck name is exactly a 48dp box, the card renders in the
hexes `colors.xml` declares rather than the theme's — 13.27:1 and 5.41:1, measured off the
screenshot, which is the point of a palette a widget owns — a tap deals a card that is not the
one showing, and the deck name opens the app. What did not: the launcher gave it three columns
by two rows, and it could not be made smaller, because a provider that declares no `minResize*`
is resizable only down to its own minimum and that minimum was sized for the longest card. Across
all 797 cards at that width the median is three lines in a box that holds nine and a half; 85% of
cards fill under a third of it. The floor and the default are now separate: `minResizeHeight` is
the room two lines need, and `WidgetBox.linesFor` sets the card's `maxLines` from the height the
host reports, so what a small widget cannot show ends in an ellipsis instead of being cut blind.
Measuring the resized widget found the second half of it — the box is in dp and the card is in
sp, so the count is wrong for any reader who has turned the text up. A line is
`1.2 × 12sp × scale + 3dp`, the 3dp of `lineSpacingExtra` being dp and so fixed: 17.4dp at a
scale of 1, and 21.7dp at 1.3, not the 22.6 that scaling the whole line would give. The ellipsis
was watched firing on a four-line box on the device.

**The first design critique, and what it could fix without a decision.** Measured from the sources
and a rendered reconstruction, no device. Two findings were high, both in the widget and both from
the previous two days' work: at its 110dp minimum a long card was cut after two and a half lines
with no ellipsis, because `ellipsize` fires only at `maxLines`; and the deck name — since it opens
the app — was a 21dp tap target whose `contentDescription` replaced the deck's name for TalkBack.
The card now autosizes 15sp→12sp and the floor is what the longest card needs at 12sp with the
label's 48dp box — 200dp, which was then also the smallest the widget could be, as the widget
walk above found. The arithmetic has since moved into `WidgetBox`, and
`ManifestAgreementTest` holds its constants to the XML that states them. The description carries
both facts. The mechanical rest went in with it: the browse dialog titled by deck rather
than "156 matching", the failure screen as a sentence over the exception rather than the
exception alone, a fading edge on the scroll, 12sp where there was 11, two directive empty states,
spacing on a 4dp grid. Three findings were decisions, taken the same day: the card sits on a tonal
surface — the theme's text colour at six percent over its background, one 16dp radius, no shadow —
and the app name recedes to 22sp so the card is the first thing read; the list of kept cards is
"Saved", so the row no longer reads Kept cards · Kept; and long-press owns copy outright, with
`setTextIsSelectable` gone, because a handled long-click suppresses the TextView's own selection and
the comment saying the two coexisted was not true.

**Nothing held the prose to the code.** The tests hold the code to what the prose says about
cards and draws; the prose itself could say anything about the code and pass. Two audits found
the drift by hand — "two calls" after the JNI surface had five, "14 CLI tests" after there were
26, the other seven introduced by six purposes, a voice-rule table giving one deck's longest card
where every other row gave its ceiling — and a hand pass runs once. `tools/spec_check.py` is the
mechanical form, lifted from chronicle's (which is mnemo's) and given the layer neither has: it
checks the *numbers*. Every card count, deck total, length ceiling, question count, the JNI call
count, the field count, the disabled-lint count, the CLI test count and the multi-line card count
are read from the tree and compared with every place the docs state them. Files named must exist;
symbols named must appear in the sources. Its first run found the voice-rule table's `oblique`
row, which had been read past twice. It is CI's first job, before any toolchain warms up.

Two smaller harvests from the same neighbours. The DEX check moved out of the workflow YAML into
`tools/r8_check.py`, chronicle's shape for the check chronicle lifted from here — as a script it
runs locally with `just r8-check` rather than only on a push. And `ManifestAgreementTest` pins
the facts written in two places that drift with no error: the tap action in the manifest and in
`CardWidget`, `updatePeriodMillis` in the provider XML and the KDoc that promises it is zero, the
one activity, one receiver, one layout and no permissions the README describes. That is mnemo's
`WidgetSizeClassTest`, which holds its Kotlin size ladder to its provider XML for the same reason.

**The completions' contract with `--list` was prose only.** The three shell scripts read deck ids
out of `--list` rather than keeping a copy, on one shared rule — a deck line is one whose card
count is a number, and the marker on the default deck shifts the columns. `completions/README.md`
said so; nothing checked it. A change to the shape of that output would have left every shell
quietly offering no deck names at all. `tests/cli.rs` now applies that rule to the binary's real
output and asserts it recovers exactly the decks that are compiled in.
