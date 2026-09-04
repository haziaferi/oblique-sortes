//! Moves for when the work will not move.
//!
//! This deck shares its territory with the Oblique Strategies and works in the
//! opposite direction. `oblique` sidesteps a problem: it hands you `Water`, or
//! tells you to do the washing up, and trusts the knight's move. `stuck`
//! attacks the problem head on, with the next concrete thing to try.
//!
//! Every card is written for this crate. The method behind it is old and not
//! anyone's property: Descartes on accepting nothing unexamined, dividing a
//! difficulty into parts, and enumerating so completely that nothing is left
//! out; Polya on restating the problem, solving a simpler one first, working
//! backwards from the answer, and looking for a problem already solved that
//! resembles this one. Their wording is nobody's to borrow, and none of it is
//! here.
//!
//! Every card either asks a diagnostic question or gives an instruction.
//! Nothing here is a statement to sit with; that is what the other decks are
//! for. Which of the two a card does is an editorial rule and not a checkable
//! one, so no test holds it.

use crate::{Deck, Provenance};

/// The stuck deck.
pub const DECK: Deck = Deck {
    id: "stuck",
    name: "Stuck",
    blurb: "Moves for when the work will not move.",
    provenance: Provenance::Original,
    cards: CARDS,
};

const CARDS: &[&str] = &[
    "Accept nothing here that you have not checked yourself.",
    "Are you protecting the work from being seen?",
    "Are you solving this, or avoiding something else?",
    "Are you stuck, or are you tired?",
    "Ask in order\n\t-what is wanted\n\t-what is given\n\t-what joins them",
    "Ask the person who would know. Now, not later.",
    "Ask where you are stuck\n\t-starting\n\t-deciding\n\t-finishing",
    "Ask who decided it had to be this way.",
    "Assume it is solved. What must have been true?",
    "Assume the fault is in your own work.",
    "Change one thing at a time and keep the old version.",
    "Check the boundaries before the middle.",
    "Check the thing you are most certain about.",
    "Choose whichever option keeps the most doors open.",
    "Cut the part you had to explain.",
    "Cut the piece you are least sure about.",
    "Cut the problem into parts. Solve the easiest part first.",
    "Decide what is not going in.",
    "Decide what you would need to know, then go and find it.",
    "Describe it to someone who does not know the field.",
    "Do something else on purpose, and come back.",
    "Do the next physical action, not the next decision.",
    "Do the part you already know how to do.",
    "Do the worst possible version in ten minutes.",
    "Eat something and try again in an hour.",
    "Enumerate the cases. Check that you have all of them.",
    "Estimate it. Double the estimate. Decide again.",
    "Explain it aloud to someone who is not listening.",
    "Find the sentence doing the real work. Build around it.",
    "Find the smallest version that still has the difficulty in it.",
    "Flip a coin and notice which result you hoped for.",
    "Get one other pair of eyes on it before lunch.",
    "Go for a walk without taking the problem along.",
    "Halve the search space. Then halve it again.",
    "How is this solved in a field that is not yours?",
    "How long since you stood up?",
    "Is it done and you are stalling?",
    "Is the hard part the important part?",
    "Is the requirement actually a requirement?",
    "Is this reversible? Then stop deliberating.",
    "Is this worth the time it is now taking?",
    "Leave it overnight. Read it before you read your email.",
    "List everything it could be. Cross off what it is not.",
    "List your assumptions. Test the cheapest one.",
    "Lower the standard until you can begin.",
    "Make it fail faster.",
    "Name the constraint you have not written down.",
    "Name the cost of not deciding. It is never zero.",
    "Open the file. That is the whole task for now.",
    "Pick either. If they are this close, the closeness is the answer.",
    "Print the value you assumed.",
    "Read it as a stranger in a hurry would.",
    "Read the error message again, all of it.",
    "Remove things until it works, then add them back one at a time.",
    "Reproduce it reliably before trying to fix it.",
    "Say out loud, to someone, that you are stuck.",
    "Say what done looks like, concretely.",
    "Separate the problem you have from the one you expected.",
    "Set a deadline somebody else knows about.",
    "Set a timer for fifteen minutes and stop when it rings.",
    "Ship the part that is finished.",
    "Sleep on it. This is a method, not an excuse.",
    "Solve a simpler problem first, then work up.",
    "Split it until one piece is obviously doable.",
    "Start from the answer and work backwards.",
    "Start in the middle.",
    "State the problem in one sentence. Then delete half of it.",
    "Try\n\t-a smaller case\n\t-a special case\n\t-the opposite",
    "What are you taking on trust?",
    "What changed? Start there.",
    "What happens if you simply do not do it?",
    "What is the actual question? Ask it out loud.",
    "What is the last thing that has to be true before this ends?",
    "What is the laziest thing that would work?",
    "What is the standard solution? Start from it.",
    "What would count as evidence that this is solved?",
    "What would this look like if it were easy?",
    "What would you remove if you had to remove something?",
    "What would you tell someone else to do here?",
    "When did it last work?",
    "Where have you seen this shape of problem before?",
    "Which choice is easier to undo?",
    "Which constraint is real, and which is only habit?",
    "Which part is hard? Point at it.",
    "Who has already done this? Read what they did.",
    "Whose problem is this actually?",
    "Would you regret this in a year, or only on Friday?",
    "Write down what you want, not the method you had in mind.",
    "Write the part you can already see.",
    "Write the question you would post, then post it.",
];
