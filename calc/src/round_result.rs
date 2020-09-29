use crate::{
    Combat, CombatResult, CombatType, Force, Prob, ProbDist, ProbDistBuilder, Probability, Pruner,
    Unit,
};

#[derive(Debug)]
pub struct RoundResult<TCombatType: CombatType, TUnit: Unit> {
    pub index: usize,
    pub pending: ProbDist<Combat<TCombatType, TUnit>>,
    pub completed: ProbDist<Combat<TCombatType, TUnit>>,
    pub pruned: ProbDist<Combat<TCombatType, TUnit>>,
    pub surviving_attackers: ProbDist<Force<TUnit>>,
    pub surviving_defenders: ProbDist<Force<TUnit>>,
    pub total_probability: Probability,
    pub pruned_count: usize,
    pub pruned_p: Probability,
    pub stalemate: bool,
}

impl<TCombatType: CombatType, TUnit: Unit> Default for RoundResult<TCombatType, TUnit> {
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

impl<TCombatType: CombatType, TUnit: Unit> RoundResult<TCombatType, TUnit> {
    pub fn new(
        combat_type: TCombatType,
        attackers: Force<TUnit>,
        defenders: Force<TUnit>,
    ) -> RoundResult<TCombatType, TUnit> {
        RoundResult {
            pending: vec![Prob {
                item: Combat {
                    attackers: attackers.clone(),
                    defenders: defenders.clone(),
                    combat_type,
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
    pub fn is_complete(&self) -> bool {
        self.pending.is_empty() || self.stalemate
    }

    pub fn total_probability(&self) -> Probability {
        self.total_probability
    }
}

#[derive(Debug)]
pub struct RoundResultBuilder<TCombatType: CombatType, TUnit: Unit> {
    // TODO: Why are these public?
    pub index: usize,
    pub pending: ProbDistBuilder<Combat<TCombatType, TUnit>>,
    pub completed: ProbDistBuilder<Combat<TCombatType, TUnit>>,
    pub pruned: ProbDistBuilder<Combat<TCombatType, TUnit>>,
    pub surviving_attackers: ProbDistBuilder<Force<TUnit>>,
    pub surviving_defenders: ProbDistBuilder<Force<TUnit>>,
}

impl<TCombatType: CombatType, TUnit: Unit> RoundResultBuilder<TCombatType, TUnit> {
    pub fn new(round_index: usize) -> Self {
        RoundResultBuilder {
            index: round_index,
            pending: ProbDistBuilder::default(),
            completed: ProbDistBuilder::default(),
            pruned: ProbDistBuilder::default(),
            surviving_attackers: ProbDistBuilder::default(),
            surviving_defenders: ProbDistBuilder::default(),
        }
    }

    pub fn build(self, pruned_count: usize, pruned_p: Probability) -> RoundResult<TCombatType, TUnit> {
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
            pruned_count,
            pruned_p,
            stalemate: false,
        }
    }

    pub fn add(&mut self, combat_result: CombatResult<TCombatType, TUnit>, pruner: &mut Pruner) {
        let attackers = combat_result.surviving_attackers.outcomes();
        let defenders = combat_result.surviving_defenders.outcomes();
        for attacker in attackers {
            for defender in defenders {
                let p = combat_result.probability * attacker.p * defender.p;
                let combat = Combat {
                    attackers: attacker.item.clone(),
                    defenders: defender.item.clone(),
                    combat_type: combat_result.next_combat_type,
                };

                let combat = Prob { item: combat, p };
                if pruner.prune(&combat) {
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
