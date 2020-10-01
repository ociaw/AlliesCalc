use std::{fmt::Debug, hash::Hash};

/// Represents the different phases of battle.
pub trait BattlePhase: Debug + Clone + Copy + Eq + Ord + Hash + Sized {
    /// Returns the battle phase that indicates the battle hasn't begun.
    fn prebattle() -> Self;
}

/// Represents the battle phase sequence - the order in which battle phases occur.
///
/// A battle sequence has two parts - the start, and the cycle. The start only occurs once,
/// after the pre-battle phase, but at the beginning of battle. After each phase in `start`
/// has occurred, the battle phases in `cycle` will be looped through indefinitely.
///
/// For example, take a battle sequence where `start` contains `Start1` and `Start2`, and
/// `cycle` contains `Cycle1`, `Cycle2`, `Cycle3`. The battle sequence for the first 10
/// rounds will be:
///
/// 0.  Pre-Battle
/// 1.  Start1
/// 2.  Start2
/// 3.  Cycle1
/// 4.  Cycle2
/// 5.  Cycle3
/// 6.  Cycle1
/// 7.  Cycle2
/// 8.  Cycle3
/// 9.  Cycle1
/// 10. Cycle2
///
/// And so on. If `start` is empty, the sequence will proceed directly to `cycle`. `cycle`
/// must contain at least one battle phase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhaseSequence<TBattlePhase: BattlePhase> {
    start: Vec<TBattlePhase>,
    cycle: Vec<TBattlePhase>,
}

impl<TBattlePhase: BattlePhase> PhaseSequence<TBattlePhase> {
    /// Constructs a new `PhaseSequence` with the the given `start` and `cycle`. `cycle` must not
    /// be empty.
    pub fn new(start: Vec<TBattlePhase>, cycle: Vec<TBattlePhase>) -> PhaseSequence<TBattlePhase> {
        if cycle.is_empty() {
            panic!("Cycle must not be empty.");
        }

        PhaseSequence { start, cycle }
    }

    /// Returns a slice of the starting combat sequence.
    pub fn start(&self) -> &[TBattlePhase] {
        &self.start
    }

    /// Returns a slice of the cycling combat sequence.
    pub fn cycle(&self) -> &[TBattlePhase] {
        &self.cycle
    }

    /// Returns the combat phase occurring at the indicated round index.
    pub fn combat_at(&self, index: usize) -> TBattlePhase {
        if index == 0 {
            return BattlePhase::prebattle();
        }
        // Make index zero based for start
        let index = index - 1;
        if index < self.start.len() {
            return self.start[index];
        }
        // Make index zero based for cycling
        let index = index - self.start.len();
        self.cycle[index % self.cycle.len()]
    }
}
