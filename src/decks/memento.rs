//! Finitude, stated plainly.
//!
//! Every card is written for this crate. The register comes from Marcus
//! Aurelius on losing only the present, Seneca on length not being the measure
//! of a life, Montaigne on the practice of thinking about dying, and the `ars
//! moriendi` tradition of treating death as a thing one is bad at for want of
//! rehearsal. No card quotes any of them, and none touches Rilke, whose letters
//! remain in copyright.
//!
//! `absurd` also deals in finitude; the two are kept apart by mood. That deck
//! provokes - it asks and it instructs. This one only states: no card here is a
//! question and none opens with an imperative, both enforced when the deck is
//! generated.
//!
//! The deck withholds consolation, which the tradition does too. It is not
//! bleak on purpose: a good third of the cards turn back toward the ordinary
//! afternoon, because that is what Marcus and Seneca were arguing for.

use crate::{Deck, Provenance};

/// The memento deck.
pub const DECK: Deck = Deck {
    id: "memento",
    name: "Memento",
    blurb: "Finitude, stated plainly.",
    provenance: Provenance::Original,
    cards: CARDS,
};

const CARDS: &[&str] = &[
    "A grave is tended for about as long as someone remembers why.",
    "A life is mostly weather and errands, and it still counts.",
    "A long life and a short one arrive at the same place.",
    "A person who has lived a day has seen the pattern of all of it.",
    "Almost everyone who has ever lived is dead and was ordinary.",
    "An ordinary Tuesday is most of what a life is made of.",
    "Being finite makes nothing about today less real.",
    "Being forgotten is the arrangement, not a verdict.",
    "Childhood ended and no announcement was made.",
    "Dying is done badly for lack of practice.",
    "Every relationship ends in a departure or a death.",
    "Everyone now alive will be dead within about a hundred years.",
    "Everything built is either being maintained or is coming down.",
    "Funerals continue to be attended until one of them is yours.",
    "Grudges have a shelf life and it is shorter than it looks.",
    "Illness is not an interruption of a life. It is part of one.",
    "It will not be balanced out or made up for.",
    "Knowing it does not have to be constant. It has to be occasional.",
    "Libraries burn. Copies help, though not indefinitely.",
    "Most deaths happen in a room, on an unremarkable afternoon.",
    "Most of the dead are anonymous. That is the ordinary case, not a failure.",
    "Nobody is ready. Readiness was never among the options.",
    "Nobody loses a past, and nobody loses a future. Only the present is available to lose.",
    "Not promised\n\t-tomorrow\n\t-a warning\n\t-a reason",
    "Nothing about the ending makes the middle not worth having.",
    "Nothing is arranging this on anyone's behalf.",
    "Nothing is owed by the future to the present.",
    "Nothing is taken away that was not on loan.",
    "One of these ordinary mornings is the last one.",
    "Possessions are on loan and the term is not disclosed.",
    "Postponement is the only strategy that is guaranteed to fail.",
    "Roughly a third of a life is spent asleep, and that third is also the life.",
    "Someone loved will die first, or else you will.",
    "Someone will clear the house, and they will be in a hurry.",
    "Strength is on loan and the repayment is gradual.",
    "Suffering is not payment toward anything.",
    "The body keeps a schedule that was never shown to anyone.",
    "The body wears out in a roughly fixed order.",
    "The day is on a calendar that has already been printed.",
    "The dead of every era believed their era was the important one.",
    "The dead outnumber the living and always will.",
    "The dead would take this ordinary afternoon without hesitating.",
    "The ending does not supply a meaning to what came before.",
    "The famous of one century go unread in the century after next.",
    "The fear of it is larger than it, and arrives much earlier.",
    "The good hours were not the ones announced in advance.",
    "The important conversation has a deadline that is not published.",
    "The languages that carried the important books are mostly gone.",
    "The last person who will remember you is alive now, or is not yet born.",
    "The length of a life is not the measure of it.",
    "The middle of a life is only identifiable afterwards.",
    "The number of days remaining is a specific number.",
    "The paperwork after a death is done by someone who loved the dead.",
    "The people alive alongside you are alive now, and that is the whole offer.",
    "The reconciliation happens now or does not happen.",
    "The same end reaches the diligent and the idle.",
    "The shortness is the reason it counts, not an argument against it.",
    "The species is recent and the planet is not.",
    "The stars visible tonight were visible to people who are entirely gone.",
    "The thought is duller with practice, not larger.",
    "The unfairness is not an error in the system.",
    "The work outlasts the worker, and then it does not.",
    "The years already used are not available for revision.",
    "There are people already seen for the last time.",
    "There have been about a hundred billion. The number is not comforting or cruel.",
    "There is no lesson in it. It is what happens.",
    "There is time for one or two things, which is not the same as none.",
    "There will be a last time for everything done regularly.",
    "Two facts\n\t-the day exists\n\t-the date is unknown",
    "Waiting is not a preliminary to the life. It is some of it.",
    "What goes unsaid at the end stays unsaid.",
    "What is on loan\n\t-the body\n\t-the time\n\t-the people",
    "Whether the life was long or short, what is lost is the same: now.",
    "Whole cities are under fields.",
    "Within three generations no one will remember a face.",
    "Your great-great-grandparents had names you do not know.",
];
