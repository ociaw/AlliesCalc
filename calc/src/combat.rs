use crate::*;
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
pub struct CombatSequence<TBattlePhase: BattlePhase> {
    start: Vec<TBattlePhase>,
    cycle: Vec<TBattlePhase>,
}

impl<TBattlePhase: BattlePhase> CombatSequence<TBattlePhase> {
    /// Constructs a new `CombatSequence` with the the given `start` and `cycle`. `cycle` must not
    /// be empty.
    pub fn new(start: Vec<TBattlePhase>, cycle: Vec<TBattlePhase>) -> CombatSequence<TBattlePhase> {
        if cycle.is_empty() {
            panic!("Cycle must not be empty.");
        }

        CombatSequence { start, cycle }
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

/// The side of combat - attacker or defender.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Side {
    Attacker,
    Defender,
}

/// A combat occurring as a specific battle phase with the given forces attacking and defending.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Combat<TBattlePhase, TUnit>
where
    TBattlePhase: BattlePhase,
    TUnit: Unit,
{
    /// The phase of battle.
    pub battle_phase: TBattlePhase,
    /// The attacking force.
    pub attackers: Force<TUnit>,
    /// The defending force.
    pub defenders: Force<TUnit>,
}

impl<TBattlePhase, TUnit> Combat<TBattlePhase, TUnit>
where
    TBattlePhase: BattlePhase,
    TUnit: Unit,
{
    /// Returns the winner of the combat, or None if both sides are either undefeated or defeated.
    pub fn winner(&self) -> Option<Side> {
        match (self.attackers.is_empty(), self.defenders.is_empty()) {
            (true, false) => Some(Side::Defender),
            (false, true) => Some(Side::Attacker),
            _ => None,
        }
    }

    /// Indicates whether or not the combat is considered complete.
    pub fn completed(&self) -> bool {
        self.attackers.is_empty() || self.defenders.is_empty()
    }
}

/// Context of a combat used for selecting rolls.
#[derive(Debug)]
pub struct CombatContext<'a, TBattlePhase, TUnit>
where
    TBattlePhase: BattlePhase,
    TUnit: Unit,
{
    /// The underlying combat.
    pub combat: &'a Combat<TBattlePhase, TUnit>,
    /// The side this context represents.
    pub side: Side,
}

impl<'a, TBattlePhase, TUnit> CombatContext<'a, TBattlePhase, TUnit>
where
    TBattlePhase: BattlePhase,
    TUnit: Unit,
{
    /// Constructs a new context from a `Combat` and the `Side` of the force.
    pub fn from_combat(combat: &'a Combat<TBattlePhase, TUnit>, side: Side) -> Self {
        Self { combat, side }
    }

    /// Returns the friendly force.
    pub fn friendlies(&self) -> &QuantDist<TUnit> {
        match self.side {
            Side::Attacker => &self.combat.attackers,
            Side::Defender => &self.combat.defenders,
        }
    }

    /// Returns the hostile force.
    pub fn hostiles(&self) -> &QuantDist<TUnit> {
        match self.side {
            Side::Attacker => &self.combat.defenders,
            Side::Defender => &self.combat.attackers,
        }
    }
}

/// The result of a combat.
#[derive(Debug)]
pub struct CombatResult<TBattlePhase, TUnit>
where
    TBattlePhase: BattlePhase,
    TUnit: Unit,
{
    /// The phase of battle the combat took place in.
    pub battle_phase: TBattlePhase,
    /// The phase of battle of the next combat.
    pub next_battle_phase: TBattlePhase,
    /// A `ProbDist` of the attackers who could have survived the combat.
    pub surviving_attackers: ProbDist<Force<TUnit>>,
    /// A `ProbDist` of the defenders who could have survived the combat.
    pub surviving_defenders: ProbDist<Force<TUnit>>,
    /// The probability that the combat occurrs at all.
    pub probability: Probability,
}
