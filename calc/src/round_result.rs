use crate::{
    BattlePhase, Combat, CombatResult, Force, Prob, ProbDist, ProbDistBuilder, Probability, Pruner,
    Unit,
};

/// An aggregate of all all the combat that occurred in a round.
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct RoundResult<TBattlePhase: BattlePhase, TUnit: Unit> {
    pub index: usize,
    pub pending: ProbDist<Combat<TBattlePhase, TUnit>>,
    pub completed: ProbDist<Combat<TBattlePhase, TUnit>>,
    pub pruned: ProbDist<Combat<TBattlePhase, TUnit>>,
    pub surviving_attackers: ProbDist<Force<TUnit>>,
    pub surviving_defenders: ProbDist<Force<TUnit>>,
    pub total_probability: Probability,
    pub pruned_count: usize,
    pub pruned_p: Probability,
    pub stalemate: bool,
}

impl<TBattlePhase: BattlePhase, TUnit: Unit> Default for RoundResult<TBattlePhase, TUnit> {
    fn default() -> Self {
        RoundResult {
            index: 0,
            pending: ProbDist::default(),
            completed: ProbDist::default(),
            pruned: ProbDist::default(),
            surviving_attackers: ProbDist::default(),
            surviving_defenders: ProbDist::default(),
            total_probability: Probability::zero(),
            pruned_count: 0,
            pruned_p: Probability::zero(),
            stalemate: false,
        }
    }
}

impl<TBattlePhase: BattlePhase, TUnit: Unit> RoundResult<TBattlePhase, TUnit> {
    /// Constructs a new initial `RoundResult` with the first battle phase, attackers, and defenders.
    ///
    /// This creates a round result representing the result of the pre-battle round. It contains one
    /// pending combat, which is constructed from the `attackers` and `defenders` arguments.
    pub fn new_initial(
        first_phase: TBattlePhase,
        attackers: Force<TUnit>,
        defenders: Force<TUnit>,
    ) -> RoundResult<TBattlePhase, TUnit> {
        RoundResult {
            pending: vec![Prob {
                item: Combat {
                    attackers: attackers.clone(),
                    defenders: defenders.clone(),
                    battle_phase: first_phase,
                },
                p: Probability::one(),
            }]
            .into(),
            surviving_attackers: vec![Prob {
                item: attackers,
                p: Probability::one(),
            }]
            .into(),
            surviving_defenders: vec![Prob {
                item: defenders,
                p: Probability::one(),
            }]
            .into(),
            total_probability: Probability::one(),
            ..Default::default()
        }
    }

    /// Indicates whethes or not this round completes the battle.
    pub fn is_complete(&self) -> bool {
        self.pending.is_empty() || self.stalemate
    }

    /// The probability that this round is reached in battle.
    pub fn total_probability(&self) -> Probability {
        self.total_probability
    }
}

/// A builder to incrementally construct a round result.
#[derive(Debug)]
pub struct RoundResultBuilder<TBattlePhase: BattlePhase, TUnit: Unit> {
    index: usize,
    pending: ProbDistBuilder<Combat<TBattlePhase, TUnit>>,
    completed: ProbDistBuilder<Combat<TBattlePhase, TUnit>>,
    pruned: ProbDistBuilder<Combat<TBattlePhase, TUnit>>,
    surviving_attackers: ProbDistBuilder<Force<TUnit>>,
    surviving_defenders: ProbDistBuilder<Force<TUnit>>,
    next_battle_phase: TBattlePhase,
    pruner: Pruner,
}

impl<TBattlePhase: BattlePhase, TUnit: Unit> RoundResultBuilder<TBattlePhase, TUnit> {
    // Constructs a new `RoundResultBuilder`.
    pub fn new(round_index: usize, next_battle_phase: TBattlePhase, pruner: Pruner) -> Self {
        RoundResultBuilder {
            index: round_index,
            pending: ProbDistBuilder::default(),
            completed: ProbDistBuilder::default(),
            pruned: ProbDistBuilder::default(),
            surviving_attackers: ProbDistBuilder::default(),
            surviving_defenders: ProbDistBuilder::default(),
            next_battle_phase,
            pruner,
        }
    }

    /// Consumes this builder and returns a new RoundResult.
    pub fn build(self) -> RoundResult<TBattlePhase, TUnit> {
        let pending = self.pending.build();
        let completed = self.completed.build();
        let pruned = self.pruned.build();
        let total_probability = pending
            .outcomes()
            .iter()
            .chain(completed.outcomes())
            .chain(pruned.outcomes())
            .map(|o| o.p)
            .sum();
        RoundResult {
            index: self.index,
            pending,
            completed,
            pruned,
            surviving_attackers: self.surviving_attackers.build(),
            surviving_defenders: self.surviving_defenders.build(),
            total_probability,
            pruned_count: self.pruner.count,
            pruned_p: self.pruner.sum,
            stalemate: false,
        }
    }

    /// Adds the combat result to this RoundResult builder.
    pub fn add(&mut self, combat_result: CombatResult<TBattlePhase, TUnit>) {
        let attackers = combat_result.surviving_attackers.outcomes();
        let defenders = combat_result.surviving_defenders.outcomes();
        for attacker in attackers {
            for defender in defenders {
                let p = combat_result.probability * attacker.p * defender.p;
                let combat = Combat {
                    attackers: attacker.item.clone(),
                    defenders: defender.item.clone(),
                    battle_phase: self.next_battle_phase,
                };

                let combat = Prob { item: combat, p };
                if self.pruner.prune(&combat) {
                    // Only track up to 100 pruned outcomes - otherwise they can get out of control.
                    if self.pruned.len() < 100 {
                        self.pruned.add_prob(combat);
                    }
                } else if combat.item.completed() {
                    self.completed.add_prob(combat);
                } else {
                    self.pending.add_prob(combat);
                }
            }
        }
        for attacker in attackers {
            self.surviving_attackers.add(
                attacker.item.clone(),
                attacker.p * combat_result.probability,
            );
        }
        for defender in defenders {
            self.surviving_defenders.add(
                defender.item.clone(),
                defender.p * combat_result.probability,
            );
        }
    }
}
