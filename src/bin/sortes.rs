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
    sortes [DECK] [-n COUNT]
    sortes --list
    sortes --about [DECK]

Arguments:
    DECK             Deck to draw from. Defaults to the first one built in.

Options:
    -n, --count N    Draw N cards, without replacement.
    -l, --list       List the decks built into this binary.
        --about      Describe a deck and say where its text came from.
    -h, --help       Print this message.
    -V, --version    Print the version.
";

/// What the arguments asked for.
#[derive(Debug, PartialEq, Eq)]
enum Command {
    Draw { deck: Option<String>, count: usize },
    List,
    About { deck: Option<String> },
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
    let mut list = false;
    let mut about = false;
    let mut args = args.into_iter();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => return Ok(Command::Help),
            "-V" | "--version" => return Ok(Command::Version),
            "-l" | "--list" => list = true,
            "--about" => about = true,
            "-n" | "--count" => {
                let raw = args.next().ok_or_else(|| format!("{arg} needs a number after it"))?;
                let parsed = raw.parse::<usize>().map_err(|_| format!("not a whole number: {raw}"))?;
                count = Some(parsed);
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

    if list && about {
        return Err("--list and --about do not go together".to_string());
    }
    if list && (deck.is_some() || count.is_some()) {
        return Err("--list takes no other arguments".to_string());
    }
    if about && count.is_some() {
        return Err("--about does not take a count".to_string());
    }

    if list {
        Ok(Command::List)
    } else if about {
        Ok(Command::About { deck })
    } else {
        Ok(Command::Draw {
            deck,
            count: count.unwrap_or(1),
        })
    }
}

fn run(command: Command) -> Result<(), String> {
    match command {
        Command::Help => print!("{USAGE}"),
        Command::Version => println!("sortes {}", env!("CARGO_PKG_VERSION")),
        Command::List => list_decks(),
        Command::About { deck } => describe(resolve(deck.as_deref())?),
        Command::Draw { deck, count } => {
            let deck = resolve(deck.as_deref())?;
            // Printed as-is: a card carrying `\n\t` renders the way it reads.
            for card in deck.random_n_str(count) {
                println!("{card}");
            }
        }
    }
    Ok(())
}

/// Find a deck by id, or fall back to the first one compiled in.
fn resolve(id: Option<&str>) -> Result<&'static Deck, String> {
    match id {
        Some(id) => sortes::deck_by_id(id).ok_or_else(|| {
            let built_in: Vec<&str> = sortes::decks().iter().map(|deck| deck.id).collect();
            format!("no deck called {id}. Built in: {}", built_in.join(", "))
        }),
        None => sortes::decks()
            .first()
            .copied()
            .ok_or_else(|| "no decks are built into this binary".to_string()),
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
    use super::{Command, parse, resolve};

    fn p(args: &[&str]) -> Result<Command, String> {
        parse(args.iter().map(|arg| (*arg).to_string()))
    }

    fn draw(deck: Option<&str>, count: usize) -> Command {
        Command::Draw {
            deck: deck.map(str::to_string),
            count,
        }
    }

    #[test]
    fn no_arguments_draws_one_from_the_default_deck() {
        assert_eq!(p(&[]), Ok(draw(None, 1)));
    }

    #[test]
    fn a_bare_word_names_a_deck() {
        assert_eq!(p(&["examen"]), Ok(draw(Some("examen"), 1)));
    }

    #[test]
    fn count_can_be_short_or_long() {
        assert_eq!(p(&["-n", "3"]), Ok(draw(None, 3)));
        assert_eq!(p(&["--count", "3"]), Ok(draw(None, 3)));
    }

    #[test]
    fn deck_and_count_together_in_either_order() {
        assert_eq!(p(&["examen", "-n", "3"]), Ok(draw(Some("examen"), 3)));
        assert_eq!(p(&["-n", "3", "examen"]), Ok(draw(Some("examen"), 3)));
    }

    #[test]
    fn listing() {
        assert_eq!(p(&["--list"]), Ok(Command::List));
        assert_eq!(p(&["-l"]), Ok(Command::List));
    }

    #[test]
    fn about_takes_an_optional_deck() {
        assert_eq!(
            p(&["--about", "examen"]),
            Ok(Command::About {
                deck: Some("examen".to_string())
            })
        );
        assert_eq!(p(&["--about"]), Ok(Command::About { deck: None }));
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
        assert_eq!(p(&["-n", "0"]), Ok(draw(None, 0)));
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
    fn incompatible_combinations() {
        assert!(p(&["--list", "examen"]).is_err());
        assert!(p(&["--list", "-n", "2"]).is_err());
        assert!(p(&["--list", "--about"]).is_err());
        assert!(p(&["--about", "examen", "-n", "2"]).is_err());
    }

    #[test]
    fn the_default_deck_always_resolves() {
        assert!(resolve(None).is_ok());
    }

    #[test]
    fn an_unknown_deck_is_an_error_that_names_the_alternatives() {
        let error = resolve(Some("no-such-deck")).expect_err("should not resolve");
        assert!(error.contains("no-such-deck"));
        assert!(error.contains("Built in:"));
    }
}
