use std::{fmt::Debug, hash::Hash};

pub trait Hit: Debug + Clone + Copy + Eq + Ord + Hash + Sized {}
