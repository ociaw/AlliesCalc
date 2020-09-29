use super::*;
use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BattleSummary<TCombatType: CombatType, TUnit: Unit> {
    pub prebattle: RoundSummary,
    pub round_summaries: Vec<RoundSummary>,
    pub attacker: BattleSideSummary,
    pub defender: BattleSideSummary,
    pub completed_combats: ProbDist<Combat<TCombatType, TUnit>>,
    pub draw_p: Probability,
    pub total_p: Probability,
    pub pruned_p: Probability,
}

impl<TCombatType: CombatType, TUnit: Unit> BattleSummary<TCombatType, TUnit> {
    pub fn round_count(&self) -> usize {
        self.round_summaries.len()
    }

    pub fn prebattle(&self) -> &RoundSummary {
        &self.prebattle
    }

    pub fn last_round(&self) -> Option<&RoundSummary> {
        self.round_summaries.last()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BattleSideSummary {
    pub ipc: Stat,
    pub ipc_lost: Stat,
    pub unit_count: Stat,
    pub unit_count_lost: Stat,
    pub strength: Stat,
    pub strength_lost: Stat,
    pub win_p: Probability,
}
