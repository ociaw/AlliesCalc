use super::*;
use crate::*;
use std::ops::Sub;

/// A summary of an individual round.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RoundSummary {
    /// The index of this round.
    pub index: usize,
    /// A summary of the attackers.
    pub attacker: RoundSideSummary,
    /// A summary of the defenders.
    pub defender: RoundSideSummary,
    /// The probability of a draw during this round.
    pub draw_p: Probability,
    /// The total probability pruned during this round.
    pub pruned_p: Probability,
}

/// The delta of two round summaries.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RoundDelta {
    /// The index of the subtrahend round.
    pub subtrahend_index: usize,
    /// The index of the minuend round.
    pub minuend_index: usize,
    /// The delta of the attacker summaries.
    pub attacker_delta: RoundSideDelta,
    /// The delta of the defender summaries.
    pub defender_delta: RoundSideDelta,
    /// The delta of the draw probability.
    pub draw_p: Probability,
    /// The delta of the pruned probability.
    pub pruned_p: Probability,
}

impl Sub for RoundSummary {
    type Output = RoundDelta;

    fn sub(self, rhs: Self) -> Self::Output {
        RoundDelta {
            subtrahend_index: self.index,
            minuend_index: rhs.index,
            attacker_delta: self.attacker - rhs.attacker,
            defender_delta: self.defender - rhs.defender,
            draw_p: self.draw_p - rhs.draw_p,
            pruned_p: self.pruned_p - rhs.pruned_p,
        }
    }
}

impl<TBattlePhase: BattlePhase, TUnit: Unit> From<&RoundResult<TBattlePhase, TUnit>>
    for RoundSummary
{
    fn from(result: &RoundResult<TBattlePhase, TUnit>) -> RoundSummary {
        RoundSummary {
            index: result.index,
            attacker: RoundSideSummary::from_round_result(result, Side::Attacker),
            defender: RoundSideSummary::from_round_result(result, Side::Defender),
            draw_p: sum_win_p(result.completed.outcomes(), None),
            pruned_p: result.pruned_p,
        }
    }
}

/// The delta between two `RoundSideSummary`s.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RoundSideDelta {
    /// The remaining IPC delta.
    pub ipc: Stat,
    /// The remaining unit count delta.
    pub unit_count: Stat,
    /// The remaining strength delta.
    pub strength: Stat,
    /// The win probability delta.
    pub win_p: Probability,
}

/// A summary of a specific side in a round.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RoundSideSummary {
    /// The sum of the remaining IPC of this side.
    pub ipc: Stat,
    /// The number of remaining units of this side.
    pub unit_count: Stat,
    /// The sum of the strength of this side.
    pub strength: Stat,
    /// The probability of this side winning.
    pub win_p: Probability,
}

impl RoundSideSummary {
    /// Constructs a new side summary for the given side from the round result.
    pub fn from_round_result<TBattlePhase: BattlePhase, TUnit: Unit>(
        result: &RoundResult<TBattlePhase, TUnit>,
        side: Side,
    ) -> Self {
        let iter = match side {
            Side::Attacker => result.surviving_attackers.outcomes(),
            Side::Defender => result.surviving_defenders.outcomes(),
        };

        let win_p = sum_win_p(result.completed.outcomes(), Some(side));

        let mut total_p = Probability::zero();

        let mut ipc = Stat::default();
        let mut unit_count = Stat::default();
        let mut strength = Stat::default();

        for prob in iter {
            let force = &prob.item;
            let (ipc_sum, unit_count_sum, strength_sum) =
                force.outcomes().iter().fold((0, 0, 0), |acc, quant| {
                    let count = quant.count;
                    let unit = quant.item;
                    let ipc = acc.0 + unit.ipc() * count;
                    let unit_count = acc.1 + count;
                    let strength = acc.2 + unit.strength(side) as u32 * count;
                    (ipc, unit_count, strength)
                });
            let p = prob.p;
            total_p += p;
            ipc.add_value(ipc_sum as f64, p, total_p);
            unit_count.add_value(unit_count_sum, p, total_p);
            strength.add_value(strength_sum, p, total_p);
        }

        Self {
            ipc,
            unit_count,
            strength,
            win_p,
        }
    }
}

impl Sub for RoundSideSummary {
    type Output = RoundSideDelta;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            ipc: self.ipc - rhs.ipc,
            unit_count: self.unit_count - rhs.unit_count,
            strength: self.strength - rhs.strength,
            win_p: self.win_p - rhs.win_p,
        }
    }
}

fn sum_win_p<TBattlePhase: BattlePhase, TUnit: Unit>(
    outcomes: &[Prob<Combat<TBattlePhase, TUnit>>],
    side: Option<Side>,
) -> Probability {
    outcomes
        .iter()
        .filter(|prob| prob.item.winner() == side)
        .map(|prob| prob.p)
        .sum()
}
