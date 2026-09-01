//! Brian Eno and Peter Schmidt's [Oblique Strategies](https://en.wikipedia.org/wiki/Oblique_Strategies)
//! in a library, for those moments when the work is stuck and a dilemma needs a
//! lateral nudge.
//!
//! The list here is a curated amalgam of the card text from several editions of
//! the deck, so it includes variant wordings of some strategies (and one blank
//! white card, as the deck itself does). Cards that carry several lines of text
//! contain embedded newline and tab characters; print them as-is and they will
//! render the way the card reads.
//!
//! # Examples
//!
//! ```
//! let strategy = oblique::random();
//! println!("The next move: {}", strategy);
//! ```

const CANONICAL_STRATEGIES: &[&str] = &[
    "(Organic) machinery",
    "A line has two sides",
    "A very small object\n\tIts center",
    "Abandon desire",
    "Abandon normal instructions",
    "Abandon normal instruments",
    "Accept advice",
    "Accretion",
    "Adding on",
    "Allow an easement (an easement is the abandonment of a stricture)",
    "Always first steps",
    "Always give yourself credit for having more than personality",
    "Are there sections? Consider transitions",
    "Ask people to work against their better judgement",
    "Ask your body",
    "Assemble some of the elements in a group and treat the group",
    "Balance the consistency principle with the inconsistency principle",
    "Be dirty",
    "Be extravagant",
    "Be less critical",
    "Breathe more deeply",
    "Bridges\n\t-build\n\t-burn",
    "Cascades",
    "Change ambiguities to specifics",
    "Change instrument roles",
    "Change nothing and continue consistently",
    "Change nothing and continue with immaculate consistency",
    "Change specifics to ambiguities",
    "Children\n\t-speaking\n\t-singing",
    "Cluster analysis",
    "Consider different fading systems",
    "Consider transitions",
    "Consult other sources\n\t-promising\n\t-unpromising",
    "Convert a melodic element into a rhythmic element",
    "Courage!",
    "Cut a vital connection",
    "Decorate, decorate",
    "Define an area as 'safe' and use it as an anchor",
    "Destroy\n\t-nothing\n\t-the most important thing",
    "Discard an axiom",
    "Disciplined self-indulgence",
    "Disconnect from desire",
    "Discover the recipes you are using and abandon them",
    "Discover your formulas and abandon them",
    "Display your talent",
    "Distorting time",
    "Do nothing for as long as possible",
    "Do something boring",
    "Do something sudden, destructive and unpredictable",
    "Do the last thing first",
    "Do the washing up",
    "Do the words need changing?",
    "Do we need holes?",
    "Don't avoid what is easy",
    "Don't be afraid of things because they're easy to do",
    "Don't be frightened of cliches",
    "Don't be frightened to display your talents",
    "Don't break the silence",
    "Don't stress one thing more than another",
    "Emphasize differences",
    "Emphasize repetitions",
    "Emphasize the flaws",
    "Faced with a choice, do both",
    "Feed the recording back out of the medium",
    "Fill every beat with something",
    "Find a safe part and use it as an anchor",
    "Get your neck massaged",
    "Ghost echoes",
    "Give the game away",
    "Give the name away",
    "Give way to your worst impulse",
    "Go outside. Shut the door.",
    "Go slowly all the way round the outside",
    "Go to an extreme, come part way back",
    "Honor thy error as a hidden intention",
    "Honor thy mistake as a hidden intention",
    "How would someone else do it?",
    "How would you have done it?",
    "Humanize something free of error",
    "Idiot glee (?)",
    "Imagine the piece as a set of disconnected events",
    "In total darkness, or in a very large room, very quietly",
    "Infinitesimal gradations",
    "Intentions\n\t-nobility of\n\t-humility of\n\t-credibility of",
    "Into the impossible",
    "Is it finished?",
    "Is something missing?",
    "Is the information correct?",
    "Is the style right?",
    "Is there something missing?",
    "It is quite possible (after all)",
    "It is simply a matter of work",
    "Just carry on",
    "Left channel, right channel, center channel",
    "Listen to the quiet voice",
    "Look at the order in which you do things",
    "Look closely at the most embarrassing details & amplify them",
    "Lost in useless territory",
    "Lowest common denominator",
    "Magnify the most difficult details",
    "Make a blank valuable by putting it in an exquisite frame",
    "Make a sudden, destructive unpredictable action; incorporate",
    "Make an exhaustive list of everything you might do & do the last thing on the list",
    "Make it more sensual",
    "Make what's perfect more human",
    "Mechanicalize something idiosyncratic",
    "Move towards the unimportant",
    "Mute and continue",
    "Not building a wall but making a brick",
    "Not building a wall; making a brick",
    "Once the search has begun, something will be found",
    "Only a part, not the whole",
    "Only one element of each kind",
    "Overtly resist change",
    "Pae White's non-blank graphic metacard",
    "Put in earplugs",
    "Question the heroic",
    "Reevaluation (a warm feeling)",
    "Remember those quiet evenings",
    "Remove a restriction",
    "Remove ambiguities and convert to specifics",
    "Remove specifics and convert to ambiguities",
    "Repetition is a form of change",
    "Retrace your steps",
    "Reverse",
    "Short circuit (example; a man eating peas with the idea that they will improve his virility shovels them straight into his lap)",
    "Simple subtraction",
    "Simply a matter of work",
    "Slow preparation, fast execution",
    "Spectrum analysis",
    "State the problem as clearly as possible",
    "Take a break",
    "Take away the elements in order of apparent non-importance",
    "Take away the important parts",
    "Tape your mouth",
    "The inconsistency principle",
    "The most easily forgotten thing is the most important",
    "The most important thing is the thing most easily forgotten",
    "The tape is now the music",
    "Think\n\t-inside the work\n\t-outside the work",
    "Think of the radio",
    "Tidy up",
    "Towards the insignificant",
    "Trust in the you of now",
    "Try faking it",
    "Turn it upside down",
    "Twist the spine",
    "Use 'unqualified' people",
    "Use an old idea",
    "Use an unacceptable color",
    "Use cliches",
    "Use fewer notes",
    "Use filters",
    "Use something nearby as a model",
    "Use your own ideas",
    "Voice your suspicions",
    "Water",
    "What are the sections sections of?\n\tImagine a caterpillar moving",
    "What are you really thinking about just now?",
    "What context would look right?",
    "What is the reality of the situation?",
    "What is the simplest solution?",
    "What mistakes did you make last time?",
    "What to increase? What to reduce? What to maintain?",
    "What were you really thinking about just now?",
    "What would your closest friend do?",
    "What wouldn't you do?",
    "When is it for?",
    "Where is the edge?",
    "Which parts can be grouped?",
    "Work at a different speed",
    "Would anyone want it?",
    "You are an engineer",
    "You can only make one dot at a time",
    "You don't have to be ashamed of using your own ideas",
    "[blank white card]",
];

/// Return all strategies as a slice of &str. Does not allocate.
///
/// # Examples
///
/// ```
/// let strategies = oblique::strategies_as_slice();
/// assert!(strategies.contains(&"Honor thy error as a hidden intention"));
/// ```
#[must_use]
pub const fn strategies_as_slice() -> &'static [&'static str] {
    CANONICAL_STRATEGIES
}

/// Return all strategies as a vector of strings. Allocates.
///
/// # Examples
///
/// ```
/// let all_strategies = oblique::strategies();
/// assert_eq!(all_strategies.len(), oblique::count());
/// ```
#[must_use]
pub fn strategies() -> Vec<String> {
    strategies_as_slice().iter().map(|s| s.to_string()).collect()
}

/// Return a randomly-selected strategy.
///
/// # Examples
///
/// ```
/// let strategy = oblique::random();
/// assert!(!strategy.is_empty());
/// ```
#[must_use]
pub fn random() -> String {
    random_str().to_string()
}

/// Return a randomly-selected strategy, as a static &str.
///
/// ```
/// let strategy = oblique::random_str();
/// assert!(!strategy.is_empty());
/// ```
#[must_use]
pub fn random_str() -> &'static str {
    let strategies = strategies_as_slice();
    strategies[fastrand::usize(..strategies.len())]
}

/// Return multiple randomly-selected strategies, drawn without replacement.
///
/// Returns up to `count` strategies. If `count` exceeds the total number of
/// strategies, returns all available strategies. Each card is drawn at most
/// once, but because the deck includes variant wordings from different
/// editions, two drawn cards may read very similarly.
///
/// ```
/// let strategies = oblique::random_n(3);
/// assert!(strategies.len() <= 3);
/// ```
#[must_use]
pub fn random_n(count: usize) -> Vec<String> {
    let all = strategies_as_slice();
    let count = count.min(all.len());
    let mut indices = (0..all.len()).collect::<Vec<_>>();
    fastrand::shuffle(&mut indices);
    indices.into_iter().take(count).map(|i| all[i].to_string()).collect()
}

/// Returns the total number of available strategies.
///
/// ```
/// let count = oblique::count();
/// assert!(count >= 176);
/// ```
#[must_use]
pub const fn count() -> usize {
    CANONICAL_STRATEGIES.len()
}

#[cfg(test)]
mod tests {

    #[test]
    fn random_strategy() {
        let strategy = super::random();
        assert!(!strategy.is_empty());
    }

    #[test]
    fn all_strategies() {
        let list = super::strategies();
        assert_eq!(list.len(), super::count());
    }

    #[test]
    fn random_strategy_borrowed() {
        let strategy: &'static str = super::random_str();
        assert!(!strategy.is_empty());
    }

    #[test]
    fn all_strategies_borrowed() {
        let list: &'static [&str] = super::strategies_as_slice();
        assert_eq!(list.len(), super::count());
    }

    #[test]
    fn random_multiple() {
        let strategies = super::random_n(5);
        assert_eq!(strategies.len(), 5);
    }

    #[test]
    fn random_n_exceeds_total() {
        let strategies = super::random_n(1000);
        assert_eq!(strategies.len(), super::count());
    }

    #[test]
    fn random_n_zero() {
        let strategies = super::random_n(0);
        assert!(strategies.is_empty());
    }

    #[test]
    fn no_empty_entries() {
        assert!(super::strategies_as_slice().iter().all(|s| !s.is_empty()));
    }

    #[test]
    fn entries_are_tidy() {
        for card in super::strategies_as_slice() {
            assert_eq!(card.trim(), *card, "card has leading or trailing whitespace: {card:?}");
            assert!(
                !card.contains("  "),
                "card has a run of spaces; use \\n\\t formatting instead: {card:?}"
            );
            for line in card.lines().skip(1) {
                assert!(line.starts_with('\t'), "continuation line missing tab indent: {card:?}");
            }
        }
    }

    #[test]
    fn no_duplicate_entries() {
        let unique: std::collections::HashSet<_> = super::strategies_as_slice().iter().collect();
        assert_eq!(unique.len(), super::count());
    }

    #[test]
    fn count_strategies() {
        assert_eq!(super::count(), super::strategies_as_slice().len());
    }

    #[test]
    fn count_is_176() {
        assert_eq!(super::count(), 176);
    }
}
