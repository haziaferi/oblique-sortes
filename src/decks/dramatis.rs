//! Situations, casts and beats: the parts a story is assembled from.
//!
//! The one deck in this crate taken from sources rather than written for it.
//! Its material is a taxonomy, and a taxonomy is worth reproducing rather than
//! paraphrasing. Everything here is public domain:
//!
//! - Georges Polti, *The Thirty-Six Dramatic Situations*, Lucile Ray's 1916
//!   translation - the thirty-six situations and the casts each one needs.
//! - Vladimir Propp, *Morphology of the Folktale*, 1928 - the sequence of
//!   functions. The renderings here are our own, made from the public-domain
//!   original rather than from a translation still in copyright.
//! - Aristotle, *Poetics*, S. H. Butcher's 1895 translation - reversal,
//!   recognition, and the shape of a plot.
//!
//! One editorial change: spellings follow the crate's US convention, so Polti's
//! `dishonour` appears here as `dishonor`.
//!
//! The cards are labels, not sentences. None carries terminal punctuation, none
//! is a question, and none addresses the reader; all three are enforced by the
//! crate's tests. The deck describes the story rather than the person holding
//! it. Draw two and make them the same story.

use crate::{Deck, Provenance};

/// The dramatis deck.
pub const DECK: Deck = Deck {
    id: "dramatis",
    name: "Dramatis",
    blurb: "Situations, casts and beats to assemble a story from.",
    provenance: Provenance::PublicDomain(
        "Georges Polti (Ray trans. 1916), Vladimir Propp (1928), Aristotle (Butcher trans. 1895)",
    ),
    cards: CARDS,
};

const CARDS: &[&str] = &[
    "A Bold Leader, an Object, and an Adversary",
    "A Culprit, a Victim, and an Interrogator",
    "A Fugitive and a Punishment",
    "A Hero, an Ideal, and the thing sacrificed",
    "A Jealous one, an Object, a Supposed Accomplice, and an Author of the error",
    "A Kinsman who hates and a Kinsman who is hated",
    "A Lover, a Beloved, and an Obstacle",
    "A Madman and a Victim",
    "A Mortal and an Immortal",
    "A Persecutor, a Suppliant, and a Power in authority",
    "A Problem, an Interrogator, and a Seeker",
    "A Solicitor, and an Adversary who refuses",
    "A Tyrant and a Conspirator",
    "A Vanquished Power and a Victorious Enemy",
    "A beginning, a middle, and an end",
    "A change from ignorance to knowledge",
    "A difficult task is set",
    "A false hero makes claims that are not his",
    "A member of the family leaves home",
    "A prohibition is issued",
    "Abduction",
    "Adultery",
    "All sacrificed for a passion",
    "Ambition",
    "An Abductor, the Abducted, and a Guardian",
    "An Ambitious one, a Thing coveted, and an Adversary",
    "An Avenger and a Criminal",
    "An Avenging Kinsman, a Guilty Kinsman, and the remembered Victim",
    "An Unfortunate, and a Master or a Misfortune",
    "An enemy loved",
    "An error of judgment in a person otherwise good",
    "An incident that arrives unexpectedly, yet on account of what came before",
    "Character revealed by what is chosen",
    "Complication and denouement",
    "Conflict with a god",
    "Crime pursued by vengeance",
    "Crimes of love",
    "Daring enterprise",
    "Deliverance",
    "Disaster",
    "Discovery of the dishonor of a loved one",
    "Enmity of kinsmen",
    "Erroneous judgment",
    "Falling prey to cruelty or misfortune",
    "Fatal imprudence",
    "Hero and villain meet in direct combat",
    "Involuntary crimes of love",
    "Loss of loved ones",
    "Madness",
    "Mistaken jealousy",
    "Murderous adultery",
    "Necessity of sacrificing loved ones",
    "Obstacles to love",
    "Obtaining",
    "Pity and terror",
    "Pursuit",
    "Recognition",
    "Recognition and reversal, arriving together",
    "Recovery of a lost one",
    "Remorse",
    "Reversal of the situation",
    "Revolt",
    "Rivalry of kinsmen",
    "Rivalry of superior and inferior",
    "Self-sacrifice for an ideal",
    "Self-sacrifice for kindred",
    "Slaying of a kinsman unrecognized",
    "Something is missing, and the lack is felt",
    "Supplication",
    "The enigma",
    "The false hero is exposed",
    "The hero arrives home unrecognized",
    "The hero departs",
    "The hero is given a new appearance",
    "The hero is led to the thing sought",
    "The hero is marked",
    "The hero is pursued",
    "The hero is recognized",
    "The hero is rescued from pursuit",
    "The hero is tested, interrogated, or attacked",
    "The hero marries, or is otherwise rewarded",
    "The hero reacts, and a helper is acquired",
    "The hero returns",
    "The misfortune is made known",
    "The original lack is remedied",
    "The probable impossible, preferred to the possible improbable",
    "The prohibition is broken",
    "The scene of suffering",
    "The seeker agrees to act",
    "The task is accomplished",
    "The turn\n\t-recognition\n\t-reversal\n\t-both at once",
    "The unity of a single action",
    "The victim is taken in, and unwittingly helps",
    "The villain attempts a deception",
    "The villain does harm to a member of the family",
    "The villain is defeated",
    "The villain is punished",
    "The villain learns something about the victim",
    "The villain makes an attempt at reconnaissance",
    "Three roles\n\t-who wants\n\t-who blocks\n\t-who decides",
    "Two Adulterers and a Betrayed Spouse",
    "Two Kinsmen and the Object of their rivalry",
    "Vengeance taken for kindred upon kindred",
    "What is missing\n\t-a person\n\t-an object\n\t-a truth",
];
