use crate::{Combat, CombatResult, CombatType, Force, Prob, ProbDist, Probability, Pruner, Unit};

#[derive(Debug)]
pub struct RoundResult<TCombatType: CombatType, TUnit: Unit> {
    pub pending: ProbDist<Combat<TCombatType, TUnit>>,
    pub completed: ProbDist<Combat<TCombatType, TUnit>>,
    pub pruned: ProbDist<Combat<TCombatType, TUnit>>,
    pub surviving_attackers: ProbDist<Force<TUnit>>,
    pub surviving_defenders: ProbDist<Force<TUnit>>,
    pub stalemate: bool,
}

impl<TCombatType: CombatType, TUnit: Unit> Default for RoundResult<TCombatType, TUnit> {
    fn default() -> Self {
        RoundResult {
            pending: ProbDist::default(),
            completed: ProbDist::default(),
            pruned: ProbDist::default(),
            surviving_attackers: ProbDist::default(),
            surviving_defenders: ProbDist::default(),
            stalemate: false,
        }
    }
}

impl<TCombatType: CombatType, TUnit: Unit> RoundResult<TCombatType, TUnit> {
    pub fn add(&mut self, combat_result: CombatResult<TCombatType, TUnit>, pruner: &mut Pruner) {
        let attackers = combat_result.surviving_attackers.outcomes;
        let defenders = combat_result.surviving_defenders.outcomes;
        for attacker in &attackers {
            for defender in &defenders {
                let p = combat_result.probability * attacker.p * defender.p;
                let combat = Combat {
                    attackers: attacker.item.clone(),
                    defenders: defender.item.clone(),
                    combat_type: combat_result.next_combat_type,
                };

                let combat = Prob { item: combat, p };
                if pruner.prune(&combat) {
                    // Only track up to 100 pruned outcomes - otherwise they can get out of control.
                    if self.pruned.outcomes.len() < 100 {
                        self.pruned.add(combat);
                    }
                } else if combat.item.completed() {
                    self.completed.add(combat);
                } else {
                    self.pending.add(combat);
                }
            }
        }
        for attacker in attackers {
            self.surviving_attackers.add(attacker * combat_result.probability);
        }
        for defender in defenders {
            self.surviving_defenders.add(defender * combat_result.probability);
        }
    }

    pub fn is_complete(&self) -> bool {
        self.pending.outcomes.is_empty() || self.stalemate
    }

    pub fn total_probability(&self) -> Probability {
        self.pending
            .outcomes
            .iter()
            .chain(self.completed.outcomes.iter())
            .chain(self.pruned.outcomes.iter())
            .map(|o| o.p)
            .sum()
    }
}
