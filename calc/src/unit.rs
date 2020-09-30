use crate::{QuantDist, Side};
use std::{fmt::Debug, hash::Hash, rc::Rc};

/// Represents a unit.
pub trait Unit: Debug + Clone + Copy + Eq + Ord + Hash + Sized + core::fmt::Display {
    /// Returns the cost of this unit in IPC.
    fn ipc(self) -> u32;

    /// Retuns the strength of this unit when fighting for the given side.
    fn strength(self, side: Side) -> u8;

    /// Returns the attack strength of this unit.
    fn attack(self) -> u8;

    /// Returns the defense strength of this unit.
    fn defense(self) -> u8;
}

pub type Force<Unit> = Rc<QuantDist<Unit>>;
