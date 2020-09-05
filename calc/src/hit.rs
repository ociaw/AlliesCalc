use std::{fmt::Debug, hash::Hash};

pub trait Hit<TUnit: crate::Unit>: Debug + Clone + Copy + Eq + Ord + Hash + Sized {
    fn hits(self, unit: TUnit) -> bool;
}
