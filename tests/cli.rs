//! End-to-end tests for the `sortes` binary.
//!
//! The unit tests beside [`parse`] prove the grammar; these prove the program —
//! what reaches stdout, what reaches stderr, and what the exit code says. They
//! run the binary Cargo just built, through `CARGO_BIN_EXE_sortes`.
//!
//! Nothing here names a deck by hand. The binary is built under whatever
//! feature set the test run enables, so every expectation is derived from
//! `sortes::decks()` and the tests hold under a single-deck build as readily as
//! under `--all-features`.

use std::process::{Command, Output};

/// The binary under test, as Cargo built it for this run.
const BIN: &str = env!("CARGO_BIN_EXE_sortes");

/// Run the binary and return what it did.
fn run(args: &[&str]) -> Output {
    Command::new(BIN)
        .args(args)
        .output()
        .expect("the binary Cargo just built should be runnable")
}

/// Stdout as text, with line endings normalised so a Windows runner agrees
/// with a Linux one.
fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone())
        .expect("output is card text, which is ASCII")
        .replace("\r\n", "\n")
}

fn stderr(output: &Output) -> String {
    String::from_utf8(output.stderr.clone())
        .expect("messages are ASCII")
        .replace("\r\n", "\n")
}

/// Cards as the binary prints them: one after another, each ended by a newline.
fn as_printed(cards: &[&str]) -> String {
    cards.iter().map(|card| format!("{card}\n")).collect()
}

fn first_deck() -> &'static sortes::Deck {
    sortes::decks().first().copied().expect("a deck is always compiled in")
}

#[test]
fn a_bare_run_prints_one_card_from_the_default_deck() {
    let output = run(&[]);
    assert!(output.status.success());
    let printed = stdout(&output);
    let card = printed
        .strip_suffix('\n')
        .expect("a card is printed with a newline after it");
    assert!(
        first_deck().cards().contains(&card),
        "printed something not in the deck: {card:?}"
    );
}

/// The whole point of the seed: the same one deals the same cards, and the
/// binary prints exactly what the library dealt — tabs, newlines and all.
#[test]
fn a_seeded_draw_matches_the_library_byte_for_byte() {
    let deck = first_deck();
    let expected = as_printed(&deck.shoe_with_seed(4242).draw_n(5));
    let output = run(&[deck.id, "-n", "5", "--seed", "4242"]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), expected);
}

#[test]
fn the_same_seed_survives_two_processes() {
    let deck = first_deck();
    let once = run(&[deck.id, "-n", "4", "--seed", "17"]);
    let twice = run(&[deck.id, "-n", "4", "--seed", "17"]);
    assert_eq!(stdout(&once), stdout(&twice));
    assert!(!stdout(&once).is_empty());
}

#[test]
fn a_seeded_any_is_reproducible_too() {
    let once = run(&["--any", "-n", "3", "--seed", "88"]);
    let twice = run(&["--any", "-n", "3", "--seed", "88"]);
    assert!(once.status.success());
    assert_eq!(stdout(&once), stdout(&twice));
}

#[test]
fn any_draws_from_some_deck_that_is_built_in() {
    let output = run(&["--any"]);
    assert!(output.status.success());
    let printed = stdout(&output);
    let card = printed
        .strip_suffix('\n')
        .expect("a card is printed with a newline after it");
    assert!(
        sortes::decks().iter().any(|deck| deck.cards().contains(&card)),
        "--any printed a card no deck holds: {card:?}"
    );
}

#[test]
fn all_prints_the_deck_entire_and_in_order() {
    for deck in sortes::decks() {
        let output = run(&[deck.id, "--all"]);
        assert!(output.status.success());
        assert_eq!(
            stdout(&output),
            as_printed(deck.cards()),
            "--all differed for {}",
            deck.id
        );
    }
}

#[test]
fn find_prints_the_cards_that_match_and_nothing_else() {
    let deck = first_deck();
    // A word taken from a real card, so there is certainly a match.
    let sample = deck.cards().first().expect("a deck is never empty");
    let word = sample.split_whitespace().next().expect("a card has words");

    let output = run(&[deck.id, "--find", word]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), as_printed(&deck.find(word)));
    assert!(
        !stdout(&output).is_empty(),
        "{word:?} should have matched at least one card"
    );
}

#[test]
fn find_ignores_case() {
    let deck = first_deck();
    let word = deck
        .cards()
        .first()
        .and_then(|card| card.split_whitespace().next())
        .expect("a card has words");
    let lower = run(&[deck.id, "--find", &word.to_lowercase()]);
    let upper = run(&[deck.id, "--find", &word.to_uppercase()]);
    assert_eq!(stdout(&lower), stdout(&upper));
}

/// No matches is an answer, not a failure. An unknown deck is the failure, and
/// the two must not look the same to a caller.
#[test]
fn find_matching_nothing_still_succeeds() {
    let output = run(&["--find", "no card says this and none ever will"]);
    assert!(output.status.success());
    assert!(stdout(&output).is_empty());
    assert!(stderr(&output).is_empty());
}

#[test]
fn find_with_an_empty_needle_is_the_whole_deck() {
    let deck = first_deck();
    let output = run(&[deck.id, "--find", ""]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), as_printed(deck.cards()));
}

#[test]
fn a_count_of_zero_prints_nothing_and_succeeds() {
    let output = run(&["-n", "0"]);
    assert!(output.status.success());
    assert!(stdout(&output).is_empty());
}

#[test]
fn asking_for_more_than_the_deck_holds_returns_the_deck() {
    let deck = first_deck();
    let output = run(&[deck.id, "-n", "100000"]);
    assert!(output.status.success());
    // A multi-line card prints as several lines, so a card count is not a line
    // count; the deck's own lines are what to compare against.
    let expected = deck.cards().iter().map(|card| card.lines().count()).sum::<usize>();
    assert_eq!(stdout(&output).lines().count(), expected);
}

#[test]
fn listing_names_every_deck_and_marks_the_default() {
    let output = run(&["--list"]);
    assert!(output.status.success());
    let printed = stdout(&output);
    for deck in sortes::decks() {
        assert!(printed.contains(deck.id), "--list omitted {}", deck.id);
        assert!(printed.contains(deck.name), "--list omitted {}", deck.name);
    }
    assert!(printed.contains("* drawn from when no deck is named"));
    let marked = printed.lines().next().expect("--list prints at least one line");
    assert!(
        marked.starts_with('*'),
        "the first deck listed should be the marked one: {marked:?}"
    );
}

/// The three completion scripts read deck ids out of `--list` rather than
/// keeping a copy, with one rule between them: a deck line is one whose card
/// count is a number, and the marker on the default deck shifts every column by
/// one. That rule is the whole contract between `list_decks` and
/// `completions/`, and nothing but this held it — a change to the shape of
/// `--list` would leave every shell quietly offering no deck names at all.
#[test]
fn list_output_still_parses_the_way_the_completions_read_it() {
    let output = run(&["--list"]);
    assert!(output.status.success());
    let printed = stdout(&output);

    let ids: Vec<&str> = printed
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let first = fields.next()?;
            let (id, count) = if first == "*" {
                (fields.next()?, fields.next()?)
            } else {
                (first, fields.next()?)
            };
            count.parse::<usize>().ok().map(|_| id)
        })
        .collect();

    let expected: Vec<&str> = sortes::decks().iter().map(|deck| deck.id).collect();
    assert_eq!(
        ids, expected,
        "the completions would read these ids out of --list, and they are not the decks"
    );
}

#[test]
fn about_says_what_a_deck_is_and_where_it_came_from() {
    for deck in sortes::decks() {
        let output = run(&["--about", deck.id]);
        assert!(output.status.success());
        let printed = stdout(&output);
        assert!(printed.contains(deck.name));
        assert!(printed.contains(deck.id));
        assert!(printed.contains(deck.blurb));
        assert!(printed.contains(&deck.count().to_string()));
        assert!(printed.contains(&deck.provenance.describe()));
    }
}

#[test]
fn an_unknown_deck_fails_and_names_the_alternatives() {
    let output = run(&["no-such-deck"]);
    assert!(!output.status.success());
    assert!(stdout(&output).is_empty(), "a failure should print no cards");
    let message = stderr(&output);
    assert!(message.contains("no-such-deck"));
    assert!(message.contains("Built in:"));
    for deck in sortes::decks() {
        assert!(message.contains(deck.id), "the error omitted {}", deck.id);
    }
}

#[test]
fn a_usage_error_fails_and_prints_the_grammar() {
    for args in [
        vec!["--nope"],
        vec!["--all", "-n", "2"],
        vec!["--list", "--about"],
        vec!["--any", "somedeck", "--any"],
        vec!["--seed", "not-a-number"],
        vec!["-n"],
        vec!["--find"],
        vec!["--find", "x", "--all"],
        vec!["--find", "x", "-n", "2"],
    ] {
        let output = run(&args);
        assert!(!output.status.success(), "{args:?} should have failed");
        let message = stderr(&output);
        assert!(message.starts_with("sortes: "), "{args:?} gave {message:?}");
        assert!(message.contains("Usage:"), "{args:?} did not print the grammar");
    }
}

#[test]
fn help_goes_to_stdout_and_succeeds() {
    for flag in ["-h", "--help"] {
        let output = run(&[flag]);
        assert!(output.status.success());
        let printed = stdout(&output);
        assert!(printed.contains("Usage:"));
        assert!(printed.contains("--seed"));
        assert!(printed.contains("--any"));
        assert!(printed.contains("--all"));
        assert!(printed.contains("--find"));
        assert!(stderr(&output).is_empty());
    }
}

#[test]
fn version_reports_the_crate_version() {
    for flag in ["-V", "--version"] {
        let output = run(&[flag]);
        assert!(output.status.success());
        assert_eq!(stdout(&output).trim(), format!("sortes {}", env!("CARGO_PKG_VERSION")));
    }
}
