//! The [`Algorithm`] interface, as well as the implementations.

mod tile;

pub use tile::Tile;

use crate::DataType;

/// Defines an algorithm (and optional execution parameters) for sieve execution.
pub trait Algorithm: Copy {
    /// The identification of the algorithm, used for printing.
    const ID_STR: &'static str;
}




