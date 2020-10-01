use super::*;
use crate::*;

/// A summary of an entire battle.
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct BattleSummary<TBattlePhase: BattlePhase, TUnit: Unit> {
    pub prebattle: RoundSummary,
    pub round_summaries: Vec<RoundSummary>,
    pub attacker: BattleSideSummary,
    pub defender: BattleSideSummary,
    pub completed_combats: ProbDist<Combat<TBattlePhase, TUnit>>,
    pub draw_p: Probability,
    pub total_p: Probability,
    pub pruned_p: Probability,
}

impl<TBattlePhase: BattlePhase, TUnit: Unit> BattleSummary<TBattlePhase, TUnit> {
    /// Gets the number of rounds that took place in the battle.
    pub fn round_count(&self) -> usize {
        self.round_summaries.len()
    }

    /// Gets the summary for the prebattle round.
    pub fn prebattle(&self) -> &RoundSummary {
        &self.prebattle
    }

    /// Gets the summary for the last round in the battle, or None if there weren't any rounds.
    pub fn last_round(&self) -> Option<&RoundSummary> {
        self.round_summaries.last()
    }
}

/// A summary of a side in a battle.
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
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
