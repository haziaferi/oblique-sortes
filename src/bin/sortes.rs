//! The `sortes` command line.
//!
//! Argument parsing is hand-rolled against `std::env::args()`. A parser crate
//! would be more comfortable and would undo the release profile this crate is
//! tuned for, so the whole grammar lives in [`parse`], which is a pure function
//! over the arguments and is unit-tested below.

use std::process::ExitCode;

use sortes::Deck;

const USAGE: &str = "\
sortes - decks of cards for when the work will not move

Usage:
    sortes [DECK] [-n COUNT] [--seed N]
    sortes --any [-n COUNT] [--seed N]
    sortes [DECK] --all
    sortes [DECK] --find TEXT
    sortes --list
    sortes --about [DECK]

Arguments:
    DECK             Deck to draw from. Defaults to the first one built in.

Options:
    -n, --count N    Draw N cards, without replacement.
        --seed N     Draw reproducibly: the same seed deals the same cards.
        --any        Draw from a deck picked at random.
    -a, --all        Print every card in the deck, in order.
    -f, --find TEXT  Print every card containing TEXT, ignoring case.
    -l, --list       List the decks built into this binary.
        --about      Describe a deck and say where its text came from.
    -h, --help       Print this message.
    -V, --version    Print the version.

A seed deals the same cards only while the decks hold the same cards; a card
added or reworded in a later release reshuffles every seed with it.
";

/// Which deck to work on: one named, one at random, or whichever is first in.
#[derive(Debug, PartialEq, Eq)]
enum Target {
    /// The deck with this id.
    Named(String),
    /// A deck chosen at random from those compiled in.
    Any,
    /// The first deck compiled in.
    Default,
}

/// What the arguments asked for.
#[derive(Debug, PartialEq, Eq)]
enum Command {
    Draw {
        deck: Target,
        count: usize,
        seed: Option<u64>,
    },
    All {
        deck: Target,
    },
    Find {
        deck: Target,
        needle: String,
    },
    List,
    About {
        deck: Target,
    },
    Help,
    Version,
}

fn main() -> ExitCode {
    let parsed = parse(std::env::args().skip(1));
    let command = match parsed {
        Ok(command) => command,
        Err(message) => {
            eprintln!("sortes: {message}\n\n{USAGE}");
            return ExitCode::FAILURE;
        }
    };

    match run(command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("sortes: {message}");
            ExitCode::FAILURE
        }
    }
}

/// Turn the arguments into a [`Command`], or explain why they do not make one.
fn parse<I: IntoIterator<Item = String>>(args: I) -> Result<Command, String> {
    let mut deck: Option<String> = None;
    let mut count: Option<usize> = None;
    let mut seed: Option<u64> = None;
    let mut list = false;
    let mut about = false;
    let mut all = false;
    let mut any = false;
    let mut find: Option<String> = None;
    let mut args = args.into_iter();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => return Ok(Command::Help),
            "-V" | "--version" => return Ok(Command::Version),
            "-l" | "--list" => list = true,
            "--about" => about = true,
            "-a" | "--all" => all = true,
            "--any" => any = true,
            "-f" | "--find" => {
                let needle = args
                    .next()
                    .ok_or_else(|| format!("{arg} needs something to look for"))?;
                find = Some(needle);
            }
            "-n" | "--count" => {
                let raw = args.next().ok_or_else(|| format!("{arg} needs a number after it"))?;
                let parsed = raw.parse::<usize>().map_err(|_| format!("not a whole number: {raw}"))?;
                count = Some(parsed);
            }
            "--seed" => {
                let raw = args.next().ok_or_else(|| format!("{arg} needs a number after it"))?;
                let parsed = raw.parse::<u64>().map_err(|_| format!("not a whole number: {raw}"))?;
                seed = Some(parsed);
            }
            flag if flag.starts_with('-') => return Err(format!("unknown option: {flag}")),
            name => {
                if let Some(first) = &deck {
                    return Err(format!("one deck at a time; got {first} and {name}"));
                }
                deck = Some(name.to_string());
            }
        }
    }

    // Which deck, said once. `--any` and a name are two answers to one
    // question.
    if any && deck.is_some() {
        return Err("--any picks the deck; do not name one as well".to_string());
    }
    let target = match deck {
        Some(id) => Target::Named(id),
        None if any => Target::Any,
        None => Target::Default,
    };

    // Modes are exclusive. Each is checked against the ones already ruled in,
    // so no pair is reported twice and no pair is missed.
    if list && about {
        return Err("--list and --about do not go together".to_string());
    }
    if list && all {
        return Err("--list and --all do not go together".to_string());
    }
    if about && all {
        return Err("--about and --all do not go together".to_string());
    }
    if list && (target != Target::Default || count.is_some() || seed.is_some()) {
        return Err("--list takes no other arguments".to_string());
    }
    if about && (count.is_some() || seed.is_some()) {
        return Err("--about takes neither a count nor a seed".to_string());
    }
    if about && any {
        return Err("--about needs a deck it can name".to_string());
    }
    // --all prints the deck entire, in order, so both would be ignored rather
    // than obeyed; saying so beats printing something the caller did not ask
    // for.
    if all && (count.is_some() || seed.is_some()) {
        return Err("--all prints the whole deck in order; it takes neither a count nor a seed".to_string());
    }
    // --find selects rather than draws, so a count and a seed have nothing to
    // act on, and the other modes are each a different question entirely.
    if find.is_some() && (count.is_some() || seed.is_some()) {
        return Err("--find prints what it matches; it takes neither a count nor a seed".to_string());
    }
    if find.is_some() && (list || about || all) {
        return Err("--find does not go with --list, --about or --all".to_string());
    }
    if find.is_some() && any {
        return Err("--find needs a deck it can name".to_string());
    }

    if list {
        Ok(Command::List)
    } else if about {
        Ok(Command::About { deck: target })
    } else if all {
        Ok(Command::All { deck: target })
    } else if let Some(needle) = find {
        Ok(Command::Find { deck: target, needle })
    } else {
        Ok(Command::Draw {
            deck: target,
            count: count.unwrap_or(1),
            seed,
        })
    }
}

fn run(command: Command) -> Result<(), String> {
    match command {
        Command::Help => print!("{USAGE}"),
        Command::Version => println!("sortes {}", env!("CARGO_PKG_VERSION")),
        Command::List => list_decks(),
        Command::About { deck } => describe(resolve(&deck, None)?),
        // Cards are printed as they come: one carrying `\n\t` renders the way
        // it reads.
        Command::All { deck } => {
            for card in resolve(&deck, None)?.cards() {
                println!("{card}");
            }
        }
        // Prints nothing and still succeeds when nothing matches: an empty
        // result is an answer, and the shell can tell by the empty output.
        Command::Find { deck, needle } => {
            for card in resolve(&deck, None)?.find(&needle) {
                println!("{card}");
            }
        }
        Command::Draw { deck, count, seed } => {
            let deck = resolve(&deck, seed)?;
            for card in draw(deck, count, seed) {
                println!("{card}");
            }
        }
    }
    Ok(())
}

/// `count` cards, reproducibly when a seed is given.
///
/// A shoe would go on dealing past the size of the deck; the unseeded path
/// stops there, so this one is capped to match. Whether a draw repeats itself
/// should not depend on whether it can be replayed.
fn draw(deck: &'static Deck, count: usize, seed: Option<u64>) -> Vec<&'static str> {
    match seed {
        Some(seed) => deck.shoe_with_seed(seed).draw_n(count.min(deck.count())),
        None => deck.random_n_str(count),
    }
}

/// Settle a [`Target`] on one of the decks compiled in.
///
/// `seed` is used only by [`Target::Any`], so that a seeded run picks the same
/// deck as well as the same cards.
fn resolve(target: &Target, seed: Option<u64>) -> Result<&'static Deck, String> {
    let decks = sortes::decks();
    let none_built_in = || "no decks are built into this binary".to_string();

    match target {
        Target::Named(id) => sortes::deck_by_id(id).ok_or_else(|| {
            let built_in: Vec<&str> = decks.iter().map(|deck| deck.id).collect();
            format!("no deck called {id}. Built in: {}", built_in.join(", "))
        }),
        Target::Default => decks.first().copied().ok_or_else(none_built_in),
        Target::Any => {
            if decks.is_empty() {
                return Err(none_built_in());
            }
            let index = match seed {
                Some(seed) => fastrand::Rng::with_seed(seed).usize(..decks.len()),
                None => fastrand::usize(..decks.len()),
            };
            decks.get(index).copied().ok_or_else(none_built_in)
        }
    }
}

fn list_decks() {
    let decks = sortes::decks();
    let width = decks.iter().map(|deck| deck.id.len()).max().unwrap_or(0);
    for (index, deck) in decks.iter().enumerate() {
        let marker = if index == 0 { '*' } else { ' ' };
        println!("{marker} {:width$}  {:>4}  {}", deck.id, deck.count(), deck.name);
    }
    println!("\n* drawn from when no deck is named");
}

fn describe(deck: &Deck) {
    println!("{} ({})", deck.name, deck.id);
    println!("{}", deck.blurb);
    println!("{} cards", deck.count());
    println!("{}", deck.provenance.describe());
}

#[cfg(test)]
mod tests {
    use super::{Command, Target, parse, resolve};

    fn p(args: &[&str]) -> Result<Command, String> {
        parse(args.iter().map(|arg| (*arg).to_string()))
    }

    fn named(id: &str) -> Target {
        Target::Named(id.to_string())
    }

    fn draw(deck: Target, count: usize) -> Command {
        Command::Draw {
            deck,
            count,
            seed: None,
        }
    }

    #[test]
    fn no_arguments_draws_one_from_the_default_deck() {
        assert_eq!(p(&[]), Ok(draw(Target::Default, 1)));
    }

    #[test]
    fn a_bare_word_names_a_deck() {
        assert_eq!(p(&["examen"]), Ok(draw(named("examen"), 1)));
    }

    #[test]
    fn count_can_be_short_or_long() {
        assert_eq!(p(&["-n", "3"]), Ok(draw(Target::Default, 3)));
        assert_eq!(p(&["--count", "3"]), Ok(draw(Target::Default, 3)));
    }

    #[test]
    fn deck_and_count_together_in_either_order() {
        assert_eq!(p(&["examen", "-n", "3"]), Ok(draw(named("examen"), 3)));
        assert_eq!(p(&["-n", "3", "examen"]), Ok(draw(named("examen"), 3)));
    }

    #[test]
    fn listing() {
        assert_eq!(p(&["--list"]), Ok(Command::List));
        assert_eq!(p(&["-l"]), Ok(Command::List));
    }

    #[test]
    fn about_takes_an_optional_deck() {
        assert_eq!(p(&["--about", "examen"]), Ok(Command::About { deck: named("examen") }));
        assert_eq!(p(&["--about"]), Ok(Command::About { deck: Target::Default }));
    }

    #[test]
    fn help_and_version_win_immediately() {
        assert_eq!(p(&["--help"]), Ok(Command::Help));
        assert_eq!(p(&["-h"]), Ok(Command::Help));
        assert_eq!(p(&["-V"]), Ok(Command::Version));
        assert_eq!(p(&["examen", "--help"]), Ok(Command::Help));
    }

    #[test]
    fn count_needs_a_number() {
        assert!(p(&["-n"]).is_err());
        assert!(p(&["-n", "three"]).is_err());
        assert!(p(&["-n", "-1"]).is_err());
    }

    #[test]
    fn zero_is_allowed_and_draws_nothing() {
        assert_eq!(p(&["-n", "0"]), Ok(draw(Target::Default, 0)));
    }

    #[test]
    fn unknown_options_are_rejected() {
        assert!(p(&["--nope"]).is_err());
        assert!(p(&["-x"]).is_err());
    }

    #[test]
    fn only_one_deck_at_a_time() {
        assert!(p(&["examen", "absurd"]).is_err());
    }

    #[test]
    fn a_seed_rides_along_with_a_draw() {
        assert_eq!(
            p(&["examen", "--seed", "42"]),
            Ok(Command::Draw {
                deck: named("examen"),
                count: 1,
                seed: Some(42),
            })
        );
        assert_eq!(
            p(&["--seed", "7", "-n", "3"]),
            Ok(Command::Draw {
                deck: Target::Default,
                count: 3,
                seed: Some(7),
            })
        );
    }

    #[test]
    fn a_seed_needs_a_whole_number() {
        assert!(p(&["--seed"]).is_err());
        assert!(p(&["--seed", "abc"]).is_err());
        assert!(p(&["--seed", "-1"]).is_err());
    }

    #[test]
    fn any_picks_the_deck() {
        assert_eq!(p(&["--any"]), Ok(draw(Target::Any, 1)));
        assert_eq!(
            p(&["--any", "-n", "2", "--seed", "5"]),
            Ok(Command::Draw {
                deck: Target::Any,
                count: 2,
                seed: Some(5),
            })
        );
    }

    #[test]
    fn any_and_a_named_deck_are_two_answers_to_one_question() {
        assert!(p(&["--any", "examen"]).is_err());
        assert!(p(&["examen", "--any"]).is_err());
    }

    #[test]
    fn all_prints_a_deck_entire() {
        assert_eq!(p(&["--all"]), Ok(Command::All { deck: Target::Default }));
        assert_eq!(p(&["-a"]), Ok(Command::All { deck: Target::Default }));
        assert_eq!(p(&["examen", "--all"]), Ok(Command::All { deck: named("examen") }));
    }

    #[test]
    fn find_takes_text_and_an_optional_deck() {
        assert_eq!(
            p(&["--find", "repetition"]),
            Ok(Command::Find {
                deck: Target::Default,
                needle: "repetition".to_string(),
            })
        );
        assert_eq!(
            p(&["examen", "-f", "today"]),
            Ok(Command::Find {
                deck: named("examen"),
                needle: "today".to_string(),
            })
        );
    }

    #[test]
    fn find_needs_something_to_look_for() {
        assert!(p(&["--find"]).is_err());
        assert!(p(&["-f"]).is_err());
    }

    /// An empty needle matches every card, which is what `--all` is for, but it
    /// is a legitimate thing to ask and not an error.
    #[test]
    fn find_accepts_an_empty_needle() {
        assert_eq!(
            p(&["--find", ""]),
            Ok(Command::Find {
                deck: Target::Default,
                needle: String::new(),
            })
        );
    }

    #[test]
    fn incompatible_combinations() {
        assert!(p(&["--list", "examen"]).is_err());
        assert!(p(&["--list", "-n", "2"]).is_err());
        assert!(p(&["--list", "--about"]).is_err());
        assert!(p(&["--list", "--all"]).is_err());
        assert!(p(&["--list", "--seed", "1"]).is_err());
        assert!(p(&["--about", "examen", "-n", "2"]).is_err());
        assert!(p(&["--about", "examen", "--seed", "2"]).is_err());
        assert!(p(&["--about", "--all"]).is_err());
        assert!(p(&["--about", "--any"]).is_err());
        assert!(p(&["--all", "-n", "2"]).is_err());
        assert!(p(&["--all", "--seed", "2"]).is_err());
        assert!(p(&["--find", "x", "-n", "2"]).is_err());
        assert!(p(&["--find", "x", "--seed", "2"]).is_err());
        assert!(p(&["--find", "x", "--all"]).is_err());
        assert!(p(&["--find", "x", "--list"]).is_err());
        assert!(p(&["--find", "x", "--about"]).is_err());
        assert!(p(&["--find", "x", "--any"]).is_err());
    }

    #[test]
    fn the_default_deck_always_resolves() {
        assert!(resolve(&Target::Default, None).is_ok());
    }

    #[test]
    fn any_always_resolves_to_a_deck_that_is_built_in() {
        let deck = resolve(&Target::Any, None).expect("some deck is always compiled in");
        assert!(sortes::decks().contains(&deck));
    }

    /// A seeded `--any` has to pick the same deck every time, or a seeded draw
    /// would be reproducible in its cards and not in where they came from.
    #[test]
    fn a_seeded_any_picks_the_same_deck_every_time() {
        for seed in 0..16 {
            let first = resolve(&Target::Any, Some(seed)).expect("some deck is always compiled in");
            let again = resolve(&Target::Any, Some(seed)).expect("some deck is always compiled in");
            assert_eq!(first.id, again.id, "seed {seed} picked two different decks");
        }
    }

    #[test]
    fn an_unknown_deck_is_an_error_that_names_the_alternatives() {
        let error = resolve(&named("no-such-deck"), None).expect_err("should not resolve");
        assert!(error.contains("no-such-deck"));
        assert!(error.contains("Built in:"));
    }

    /// The seeded and unseeded paths must agree on how many cards a draw is.
    #[test]
    fn a_seeded_draw_is_capped_at_the_deck_like_an_unseeded_one() {
        let deck = resolve(&Target::Default, None).expect("a deck is always compiled in");
        assert_eq!(super::draw(deck, 3, Some(9)).len(), 3);
        assert_eq!(super::draw(deck, deck.count() + 50, Some(9)).len(), deck.count());
        assert_eq!(
            super::draw(deck, deck.count() + 50, None).len(),
            super::draw(deck, deck.count() + 50, Some(9)).len()
        );
    }

    #[test]
    fn the_same_seed_draws_the_same_cards() {
        let deck = resolve(&Target::Default, None).expect("a deck is always compiled in");
        assert_eq!(super::draw(deck, 5, Some(2026)), super::draw(deck, 5, Some(2026)));
    }
}
