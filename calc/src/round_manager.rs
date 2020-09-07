use crate::*;

pub struct RoundManager<
    TCombatType: CombatType,
    TUnit: Unit,
    THit: Hit<TUnit>,
    TRollSelector: RollSelector<TCombatType, TUnit, THit>,
    TSurvivorSelector: SurvivorSelector<TUnit, THit>,
> {
    combat_manager: CombatManager<TCombatType, TUnit, THit, TRollSelector, TSurvivorSelector>,
    sequence: CombatSequence<TCombatType>,
    pruner: Pruner,
    round_index: usize,
    last_round: RoundResult<TCombatType, TUnit>,
    last_probability: Probability,
    probability_run_count: usize,
}

impl<TCombatType, THit, TUnit, TRollSelector, TSurvivorSelector>
    RoundManager<TCombatType, TUnit, THit, TRollSelector, TSurvivorSelector>
where
    TCombatType: CombatType,
    TUnit: Unit,
    THit: Hit<TUnit>,
    TRollSelector: RollSelector<TCombatType, TUnit, THit>,
    TSurvivorSelector: SurvivorSelector<TUnit, THit>,
{
    pub fn new(
        combat_manager: CombatManager<TCombatType, TUnit, THit, TRollSelector, TSurvivorSelector>,
        sequence: CombatSequence<TCombatType>,
        attackers: Force<TUnit>,
        defenders: Force<TUnit>,
    ) -> Self {
        let round_index = 0;
        let combat_type = sequence.combat_at(round_index + 1);
        RoundManager {
            combat_manager,
            sequence,
            pruner: Default::default(),
            round_index,
            last_round: RoundResult {
                pending: ProbDist {
                    outcomes: vec![Prob {
                        item: Combat {
                            attackers: attackers.clone(),
                            defenders: defenders.clone(),
                            combat_type,
                        },
                        p: Probability::one(),
                    }],
                },
                completed: ProbDist::new(),
                pruned: ProbDist::new(),
                surviving_attackers: ProbDist {
                    outcomes: vec![Prob {
                        item: attackers,
                        p: Probability::one(),
                    }],
                },
                surviving_defenders: ProbDist {
                    outcomes: vec![Prob {
                        item: defenders,
                        p: Probability::one(),
                    }],
                },
                stalemate: false,
            },
            last_probability: Probability::zero(),
            probability_run_count: 0,
        }
    }

    #[allow(clippy::float_cmp)]
    pub fn advance_round(&mut self) -> &RoundResult<TCombatType, TUnit> {
        self.round_index += 1;
        let next_combat_type = self.sequence.combat_at(self.round_index + 1);
        let mut result = RoundResult::default();
        for combat in &self.last_round.pending.outcomes {
            let combat_result = self.combat_manager.resolve(combat, next_combat_type);
            result.add(combat_result, &mut self.pruner);
        }

        // We check if the current probability and the last probability are *exactly* the same;
        // if so, this may mean that we're reaching a stalemate: a point where neither side can
        // hit each other. If this happens 4 times in a row, we consider ourselves to be
        // stalemated and mark the result accordingly.
        const STALEMATE_THRESHOLD: usize = 4;
        let total_probability = result.total_probability();
        if total_probability == self.last_probability {
            self.probability_run_count += 1;
            result.stalemate = self.probability_run_count >= STALEMATE_THRESHOLD;
        } else {
            self.probability_run_count = 0;
            self.last_probability = total_probability;
        }

        self.last_round = result;
        &self.last_round
    }

    pub fn round_index(&self) -> usize {
        self.round_index
    }

    pub fn last_round(&self) -> &RoundResult<TCombatType, TUnit> {
        &self.last_round
    }

    pub fn is_complete(&self) -> bool {
        self.last_round.is_complete()
    }

    pub fn set_prune_threshold(&mut self, p: Probability) {
        self.pruner.threshold = p;
    }

    pub fn pruned_count(&self) -> usize {
        self.pruner.count
    }

    pub fn pruned_p(&self) -> Probability {
        self.pruner.sum
    }
}
