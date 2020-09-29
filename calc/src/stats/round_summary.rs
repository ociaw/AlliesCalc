use std::ops::Sub;
use crate::*;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RoundSummary {
    pub index: usize,
    pub attacker: RoundSideSummary,
    pub defender: RoundSideSummary,
    pub draw_p: Probability,
    pub pruned_p: Probability,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RoundDelta {
    pub subtrahend_index: usize,
    pub minuend_index: usize,
    pub attacker_delta: RoundSideDelta,
    pub defender_delta: RoundSideDelta,
    pub draw_p: Probability,
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

impl<TCombatType: CombatType, TUnit: Unit> From<&RoundResult<TCombatType, TUnit>> for RoundSummary {
    fn from(result: &RoundResult<TCombatType, TUnit>) -> RoundSummary {
        RoundSummary {
            index: result.index,
            attacker: RoundSideSummary::from_round_result(result, Side::Attacker),
            defender: RoundSideSummary::from_round_result(result, Side::Defender),
            draw_p: sum_win_p(result.completed.outcomes(), None),
            pruned_p: result.pruned_p
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RoundSideDelta {
    pub ipc: Stat,
    pub unit_count: Stat,
    pub strength: Stat,
    pub win_p: Probability
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RoundSideSummary {
    pub ipc: Stat,
    pub unit_count: Stat,
    pub strength: Stat,
    pub win_p: Probability,
}

impl RoundSideSummary {
    pub fn from_round_result<TCombatType: CombatType, TUnit: Unit>(result: &RoundResult<TCombatType, TUnit>, side: Side) -> Self
    {
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
            let (ipc_sum, unit_count_sum, strength_sum) = force.outcomes()
                .iter()
                .fold((0, 0, 0), |acc, quant| {
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

fn sum_win_p<TCombatType: CombatType, TUnit: Unit>(outcomes: &[Prob<Combat<TCombatType, TUnit>>], side: Option<Side>) -> Probability {
    outcomes.iter()
        .filter(|prob| prob.item.winner() == side)
        .map(|prob| prob.p)
        .sum()
}
