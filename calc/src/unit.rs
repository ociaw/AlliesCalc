use crate::QuantDist;
use std::{fmt::Debug, hash::Hash, rc::Rc};

pub trait Unit: Debug + Clone + Copy + Eq + Ord + Hash + Sized + core::fmt::Display {
    fn ipc(self) -> u32;

    fn attack(self) -> u8;

    fn defense(self) -> u8;
}

pub type Force<Unit> = Rc<QuantDist<Unit>>;
