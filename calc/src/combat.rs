use crate::*;
use std::{fmt::Debug, hash::Hash};

pub trait CombatType: Debug + Clone + Copy + Eq + Ord + Hash + Sized {
    fn prebattle() -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombatSequence<TCombatType: CombatType> {
    start: Vec<TCombatType>,
    cycle: Vec<TCombatType>,
}

impl<TCombatType: CombatType> CombatSequence<TCombatType> {
    pub fn new(start: Vec<TCombatType>, cycle: Vec<TCombatType>) -> CombatSequence<TCombatType> {
        if cycle.is_empty() {
            panic!("Cycle must not be empty.");
        }

        CombatSequence { start, cycle }
    }

    pub fn start(&self) -> &[TCombatType] {
        &self.start
    }

    pub fn cycle(&self) -> &[TCombatType] {
        &self.cycle
    }

    pub fn combat_at(&self, index: usize) -> TCombatType {
        if index == 0 {
            return CombatType::prebattle();
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Side {
    Attacker,
    Defender,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Combat<TCombatType, TUnit>
where
    TCombatType: CombatType,
    TUnit: Unit,
{
    pub combat_type: TCombatType,
    pub attackers: Force<TUnit>,
    pub defenders: Force<TUnit>,
}

impl<TCombatType, TUnit> Combat<TCombatType, TUnit>
where
    TCombatType: CombatType,
    TUnit: Unit,
{
    pub fn winner(&self) -> Option<Side> {
        match (self.attackers.is_empty(), self.defenders.is_empty()) {
            (true, false) => Some(Side::Defender),
            (false, true) => Some(Side::Attacker),
            _ => None,
        }
    }

    pub fn completed(&self) -> bool {
        self.attackers.is_empty() || self.defenders.is_empty()
    }
}

#[derive(Debug)]
pub struct CombatContext<TCombatType, TUnit>
where
    TCombatType: CombatType,
    TUnit: Unit,
{
    pub combat_type: TCombatType,
    pub attackers: Force<TUnit>,
    pub defenders: Force<TUnit>,
    pub defending: bool,
}

impl<TCombatType, TUnit> CombatContext<TCombatType, TUnit>
where
    TCombatType: CombatType,
    TUnit: Unit,
{
    pub fn from_combat(combat: &Combat<TCombatType, TUnit>, defending: bool) -> Self {
        Self {
            combat_type: combat.combat_type,
            attackers: combat.attackers.clone(),
            defenders: combat.defenders.clone(),
            defending,
        }
    }

    pub fn friendlies(&self) -> &QuantDist<TUnit> {
        if self.defending {
            &self.defenders
        } else {
            &self.attackers
        }
    }

    pub fn hostiles(&self) -> &QuantDist<TUnit> {
        if self.defending {
            &self.attackers
        } else {
            &self.defenders
        }
    }
}

#[derive(Debug)]
pub struct CombatResult<TCombatType, TUnit>
where
    TCombatType: CombatType,
    TUnit: Unit,
{
    pub combat_type: TCombatType,
    pub next_combat_type: TCombatType,
    pub surviving_attackers: ProbDist<Force<TUnit>>,
    pub surviving_defenders: ProbDist<Force<TUnit>>,
    pub probability: Probability,
}
