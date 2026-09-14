//! The absurd: freedom, finitude, and the stories you tell to avoid both.
//!
//! Every card is written for this crate. The register is drawn from the
//! existentialist line rather than its texts - Kierkegaard on anxiety, the
//! crowd, and deciding without certainty; Nietzsche on whether you would take
//! this life again; Heraclitus on standing in nothing that holds still;
//! Ecclesiastes on being forgotten; Montaigne on how little you actually know.
//! Camus and Sartre are in copyright, so their themes are here and none of
//! their formulations are; any card that drifted toward a borrowed phrasing
//! was rewritten before it went in.
//!
//! The deck addresses you directly and does not soften. Every card says `you`
//! in one form or another, and the crate's tests enforce it - the mirror of the
//! rule on `attention`, which may never say it. That one rule is what keeps two
//! short declarative decks from becoming one. Not softening is editorial, and
//! no test holds it.
//!
//! It is meant to push toward acting, which is what the tradition was always
//! for.

use crate::{Deck, Provenance};

/// The absurd deck.
pub const DECK: Deck = Deck {
    id: "absurd",
    name: "The Absurd",
    blurb: "Freedom, finitude, and the stories you tell to avoid both.",
    provenance: Provenance::Original,
    cards: CARDS,
};

const CARDS: &[&str] = &[
    "Anxiety is not a warning. It is the sound of your freedom. Ask what it is pointing at.",
    "Choose which to keep\n\t-your comfort\n\t-your reasons",
    "Count the summers you have left. Now answer the question again.",
    "Do the small honest thing before you attempt the large sincere one.",
    "Everyone agrees with you. That should trouble you more than it does.",
    "Everything you build will be kept up by someone who did not care to.",
    "If it counted for nothing, would you still do it? That answer matters.",
    "In three generations nobody will know your name. What do you do this afternoon?",
    "Name both\n\t-what you fear\n\t-what you would do without it",
    "Nobody is coming to give you permission.",
    "Nothing hands your life a meaning. You are going to have to.",
    "Nothing you are standing on is holding still.",
    "Some of what you call impossible is only expensive. Which part?",
    "Somebody thinks about you more than you would guess. Behave as if that is so.",
    "Someone is waiting for a sentence from you. You know which sentence.",
    "Something you do most weeks, you will do for the last time and not notice.",
    "Suppose you had to live this exact day again without end. Would you change the day, or your objection to it?",
    "The excuse you use most often: is it true?",
    "The most honest sentence about your situation is one you have not said aloud.",
    "The task will not stay done. Do it anyway, or stop saying you will.",
    "The thing you would do if you were braver is smaller than you think.",
    "The thing you would do unpaid answers a question you keep avoiding.",
    "What do you know, as against what you were told?",
    "What you are avoiding will still be there when you are older and tireder.",
    "What you call your personality may be a set of habits you never revisited.",
    "Whatever you decide, decide it as yourself and not as the type of person you are.",
    "Whose approval are you saving up? Spend it.",
    "Whose part are you playing so well that you forgot it was a part?",
    "Would you take this life again, unaltered? Answer before you start improving it.",
    "Write down your true reason. Show it to nobody.",
    "You are\n\t-what you do\n\t-not what you meant",
    "You are afraid of the wrong item on the list. Find the right one.",
    "You are angry with someone for a thing you also do.",
    "You are being reasonable, which is how you were talked out of it.",
    "You are being vague on purpose. Be specific and see what happens.",
    "You are certain. Of how much, and on whose word?",
    "You are consistent. You may only be repeating yourself.",
    "You are free in ways you have never once tested.",
    "You are halfway through something and you do not know which half.",
    "You are in a body, in a room, on a particular day. Start there.",
    "You are keeping a grudge in good repair. Ask what it is for.",
    "You are keeping something alive that died a while ago. Which?",
    "You are living a life you would not have chosen for someone you love.",
    "You are looking for a purpose the way you look for keys you are holding.",
    "You are not going back. There is no back.",
    "You are not required to resolve it today. You are required to look at it.",
    "You are not the person who started this sentence.",
    "You are older than you have ever been. Act like someone who noticed.",
    "You are paying for this with days. Check the price against the thing.",
    "You are performing for people who are performing for you.",
    "You are pretending not to know something. Name it.",
    "You are protecting yourself from a piece of information you already have.",
    "You are saving it up. For whom, and for when?",
    "You are waiting for a better version of yourself to show up and act.",
    "You are waiting to be certain. You are not going to be certain.",
    "You believe it because you have believed it for a long time.",
    "You borrowed your standards. Ask whether you would have chosen them.",
    "You call it an obligation. Name the person who would actually stop you.",
    "You cannot think your way to this. You will have to do something.",
    "You chose this. Say it out loud and see whether it holds.",
    "You do the same thing daily and call it a life. Is the objection to the repetition or to the thing?",
    "You explain yourself with a story. Ask who wrote it.",
    "You finished it and nothing happened. What did you expect to happen?",
    "You had a reason ready. That is what makes it suspect.",
    "You have\n\t-time you can count\n\t-time you cannot",
    "You have a finite number of mornings and have spent one of them here.",
    "You have been deciding not to decide. It counted as a decision.",
    "You have been kind in ways that cost you nothing. Try the other kind.",
    "You have been rehearsing. The rehearsal was the life.",
    "You have changed your mind about everything once already.",
    "You have enough information. You have had enough for a while.",
    "You have never checked the thing your whole plan rests on.",
    "You keep the name and replace everything else. Say what makes it still you.",
    "You keep your life in a drawer marked later.",
    "You know what to do. You are haggling over the price.",
    "You said you had no time. Account for yesterday.",
    "You say it is simply who you are. When did you decide that?",
    "You want the good parts kept and the rest revised. It does not come apart.",
    "You want your life to matter to someone not yet born.",
    "You will not find it by looking harder in the same place.",
    "You will not get this decade back. Say what it is for.",
    "You would not accept your own explanation from anyone else.",
    "You would not spend money the way you are spending this week.",
    "You would rather be certain and wrong than uncertain and moving.",
    "You would tell a friend the truth about this in one sentence. Say it to yourself.",
    "Your dread is about the choice, not the outcome.",
];
