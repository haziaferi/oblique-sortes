//! The decks. One module per deck; each exposes a `DECK` constant.
//!
//! Every deck is compiled in behind its own feature, so a consumer who wants
//! only one of them pays nothing for the rest. At least one deck feature must
//! be enabled; see the crate root.

#[cfg(feature = "absurd")]
pub mod absurd;
#[cfg(feature = "attention")]
pub mod attention;
#[cfg(feature = "constraints")]
pub mod constraints;
#[cfg(feature = "dramatis")]
pub mod dramatis;
#[cfg(feature = "examen")]
pub mod examen;
#[cfg(feature = "memento")]
pub mod memento;
#[cfg(feature = "oblique")]
pub mod oblique;
#[cfg(feature = "stuck")]
pub mod stuck;
