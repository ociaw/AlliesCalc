use std::{fmt::Debug, hash::Hash};

/// Represents a hit that can damage or kill certain units.
pub trait Hit<TUnit: crate::Unit>: Debug + Clone + Copy + Eq + Ord + Hash + Sized {
    /// Returns whether or not this hit can hit `unit`.
    fn hits(self, unit: TUnit) -> bool;
}
