use crate::*;
use std::{fmt::Debug, hash::Hash};

/// A combat occurring as a specific battle phase with the given forces attacking and defending.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
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
    /// A `ProbDist` of the attackers who could have survived the combat.
    pub surviving_attackers: ProbDist<Force<TUnit>>,
    /// A `ProbDist` of the defenders who could have survived the combat.
    pub surviving_defenders: ProbDist<Force<TUnit>>,
    /// The probability that the combat occurrs at all.
    pub probability: Probability,
}
