use super::*;
use crate::*;

/// Summarizes a battle.
#[derive(Debug, Clone, PartialEq)]
pub struct Summarizer<TCombatType: CombatType, TUnit: Unit> {
    prebattle: RoundSummary,
    round_summaries: Vec<RoundSummary>,
    attacker_summary: BattleSideBuilder,
    defender_summary: BattleSideBuilder,
    completed_combats: ProbDistBuilder<Combat<TCombatType, TUnit>>,
    draw_p: Probability,
    total_p: Probability,
    pruned_p: Probability,
}

impl<TCombatType: CombatType, TUnit: Unit> Summarizer<TCombatType, TUnit> {
    /// Creates a new battle summary.
    pub fn new(prebattle: &RoundResult<TCombatType, TUnit>) -> Self {
        Self {
            prebattle: prebattle.into(),
            round_summaries: Vec::new(),
            attacker_summary: Default::default(),
            defender_summary: Default::default(),
            completed_combats: Default::default(),
            draw_p: Default::default(),
            total_p: Default::default(),
            pruned_p: Default::default(),
        }
    }

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

    /// Consumes this summarizer and constructs a new `BattleSummary`.
    pub fn summarize(self) -> BattleSummary<TCombatType, TUnit> {
        BattleSummary {
            prebattle: self.prebattle,
            round_summaries: self.round_summaries,
            attacker: self.attacker_summary.build(&self.prebattle.attacker),
            defender: self.defender_summary.build(&self.prebattle.defender),
            completed_combats: self.completed_combats.build(),
            draw_p: self.draw_p,
            total_p: self.total_p,
            pruned_p: self.pruned_p,
        }
    }

    pub fn add_round(&mut self, round: &RoundResult<TCombatType, TUnit>) {
        self.round_summaries.push(round.into());
        self.accumulate_completed(&round.completed);
        self.pruned_p += round.pruned_p;
    }

    fn accumulate_completed(&mut self, combat: &ProbDist<Combat<TCombatType, TUnit>>) {
        for combat in combat.outcomes() {
            self.completed_combats.add_prob(combat.clone());
            self.accumulate_combat(combat);
        }
    }

    fn accumulate_combat(&mut self, combat: &Prob<Combat<TCombatType, TUnit>>) {
        let p = combat.p;
        let combat = &combat.item;
        self.total_p += p;

        self.attacker_summary
            .accumulate(combat, p, self.total_p, Side::Attacker);
        self.defender_summary
            .accumulate(combat, p, self.total_p, Side::Defender);
        if combat.winner().is_none() {
            self.draw_p += p;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct BattleSideBuilder {
    pub ipc: Stat,
    pub unit_count: Stat,
    pub strength: Stat,
    pub win_p: Probability,
}

impl BattleSideBuilder {
    pub fn accumulate<TCombatType: CombatType, TUnit: Unit>(
        &mut self,
        combat: &Combat<TCombatType, TUnit>,
        p: Probability,
        total_p: Probability,
        side: Side,
    ) {
        if combat.winner() == Some(side) {
            self.win_p += p;
        }

        let force = match side {
            Side::Attacker => &combat.attackers,
            Side::Defender => &combat.defenders,
        };

        let (ipc_sum, unit_count_sum, strength_sum) =
            force.outcomes().iter().fold((0, 0, 0), |acc, quant| {
                let count = quant.count;
                let unit = quant.item;
                let ipc = acc.0 + unit.ipc() * count;
                let unit_count = acc.1 + count;
                let strength = acc.2 + unit.strength(side) as u32 * count;
                (ipc, unit_count, strength)
            });

        self.ipc.add_value(ipc_sum as f64, p, total_p);
        self.unit_count.add_value(unit_count_sum as f64, p, total_p);
        self.strength.add_value(strength_sum as f64, p, total_p);
    }

    pub fn build(self, prebattle: &RoundSideSummary) -> BattleSideSummary {
        BattleSideSummary {
            ipc: self.ipc,
            ipc_lost: prebattle.ipc - self.ipc,
            unit_count: self.unit_count,
            unit_count_lost: prebattle.unit_count - self.unit_count,
            strength: self.strength,
            strength_lost: prebattle.strength - self.strength,
            win_p: self.win_p,
        }
    }
}

impl Default for BattleSideBuilder {
    fn default() -> Self {
        Self {
            ipc: Default::default(),
            unit_count: Default::default(),
            win_p: Default::default(),
            strength: Default::default(),
        }
    }
}
