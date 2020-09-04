use crate::QuantDist;
use std::{fmt::Debug, hash::Hash, rc::Rc};

pub trait Unit: Debug + Clone + Copy + Eq + Ord + Hash + Sized {
    fn ipc(&self) -> u32;
}

pub type Force<Unit> = Rc<QuantDist<Unit>>;
