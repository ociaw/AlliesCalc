use crate::*;

/// Manages the rounds of the battle.
pub struct RoundManager<
    TBattlePhase: BattlePhase,
    TUnit: Unit,
    THit: Hit<TUnit>,
    TRollSelector: RollSelector<TBattlePhase, TUnit, THit>,
    TSurvivorSelector: SurvivorSelector<TUnit, THit>,
> {
    combat_manager: CombatManager<TBattlePhase, TUnit, THit, TRollSelector, TSurvivorSelector>,
    sequence: PhaseSequence<TBattlePhase>,
    prune_threshold: Probability,
    round_index: usize,
    last_round: RoundResult<TBattlePhase, TUnit>,
    last_probability: Probability,
    probability_run_count: usize,
}

impl<TBattlePhase, THit, TUnit, TRollSelector, TSurvivorSelector>
    RoundManager<TBattlePhase, TUnit, THit, TRollSelector, TSurvivorSelector>
where
    TBattlePhase: BattlePhase,
    TUnit: Unit,
    THit: Hit<TUnit>,
    TRollSelector: RollSelector<TBattlePhase, TUnit, THit>,
    TSurvivorSelector: SurvivorSelector<TUnit, THit>,
{
    /// Constructs a new `RoundManager` with the given `CombatManager`, `PhaseSequence`,
    /// attacking force, and defending force.
    pub fn new(
        combat_manager: CombatManager<TBattlePhase, TUnit, THit, TRollSelector, TSurvivorSelector>,
        sequence: PhaseSequence<TBattlePhase>,
        attackers: Force<TUnit>,
        defenders: Force<TUnit>,
    ) -> Self {
        let round_index = 0;
        let first_phase = sequence.combat_at(round_index + 1);
        RoundManager {
            combat_manager,
            sequence,
            prune_threshold: Default::default(),
            round_index,
            last_round: RoundResult::new_initial(first_phase, attackers, defenders),
            last_probability: Probability::zero(),
            probability_run_count: 0,
        }
    }

    /// Computes the next round of the battle and returns the result.
    pub fn advance_round(&mut self) -> &RoundResult<TBattlePhase, TUnit> {
        self.round_processor().finish()
    }

    /// Returns a `RoundProcessor`, enabling piecemeal processing of a large round.
    pub fn round_processor(
        &mut self,
    ) -> RoundProcessor<TBattlePhase, TUnit, THit, TRollSelector, TSurvivorSelector> {
        let round_index = self.round_index + 1;
        let next_battle_phase = self.sequence.combat_at(round_index + 1);
        let pruner = Pruner::new(self.prune_threshold);
        let builder = RoundResultBuilder::new(round_index, next_battle_phase, pruner);
        RoundProcessor::new(builder, self)
    }

    #[allow(clippy::float_cmp)]
    fn finish_round(
        &mut self,
        mut result: RoundResult<TBattlePhase, TUnit>,
    ) -> &RoundResult<TBattlePhase, TUnit> {
        self.round_index += 1;
        // We check if the current probability and the last probability are *exactly* the same;
        // if so, this may mean that we're reaching a stalemate: a point where neither side can
        // hit each other. If this happens 4 times in a row, we consider ourselves to be
        // stalemated and mark the result accordingly.
        const STALEMATE_THRESHOLD: usize = 4;
        if result.total_probability() == self.last_probability {
            self.probability_run_count += 1;
            result.stalemate = self.probability_run_count >= STALEMATE_THRESHOLD;
        } else {
            self.probability_run_count = 0;
            self.last_probability = result.total_probability();
        }

        self.last_round = result;
        &self.last_round
    }

    /// Gets the current round index.
    pub fn round_index(&self) -> usize {
        self.round_index
    }

    /// Gets the result of the last round that was computed.
    pub fn last_round(&self) -> &RoundResult<TBattlePhase, TUnit> {
        &self.last_round
    }

    /// Indicates whether or not the battle is complete.
    pub fn is_complete(&self) -> bool {
        self.last_round.is_complete()
    }

    /// Sets the pruning threshold, where outcomes with a probability equal to or below are pruned.
    pub fn set_prune_threshold(&mut self, p: Probability) {
        self.prune_threshold = p;
    }
}

/// Processes round pending outcomes piecemeal.
///
/// Helpful when running on a single thread or if progress monitoring is desired.
pub struct RoundProcessor<
    'a,
    TBattlePhase: BattlePhase,
    TUnit: Unit,
    THit: Hit<TUnit>,
    TRollSelector: RollSelector<TBattlePhase, TUnit, THit>,
    TSurvivorSelector: SurvivorSelector<TUnit, THit>,
> {
    builder: RoundResultBuilder<TBattlePhase, TUnit>,
    round_manager:
        &'a mut RoundManager<TBattlePhase, TUnit, THit, TRollSelector, TSurvivorSelector>,
    processed_count: usize,
}

impl<
        'a,
        TBattlePhase: BattlePhase,
        TUnit: Unit,
        THit: Hit<TUnit>,
        TRollSelector: RollSelector<TBattlePhase, TUnit, THit>,
        TSurvivorSelector: SurvivorSelector<TUnit, THit>,
    > RoundProcessor<'a, TBattlePhase, TUnit, THit, TRollSelector, TSurvivorSelector>
{
    /// Constructs a new `RoundProcessor`.
    fn new(
        builder: RoundResultBuilder<TBattlePhase, TUnit>,
        round_manager: &'a mut RoundManager<
            TBattlePhase,
            TUnit,
            THit,
            TRollSelector,
            TSurvivorSelector,
        >,
    ) -> Self {
        Self {
            builder,
            round_manager,
            processed_count: 0,
        }
    }

    /// The number of pending outcomes that need to be processed.
    pub fn total_outcomes(&self) -> usize {
        self.outcomes().len()
    }

    /// The number of pending outcomes that have been processed.
    pub fn processed_outcomes(&self) -> usize {
        self.processed_count
    }

    /// Processes up to `limit` number of pending outcomes, until all outcomes are processed.
    /// Returns whether or not processing is complete.
    pub fn process(&mut self, limit: usize) -> bool {
        let mut count = 0;
        for combat in self
            .round_manager
            .last_round
            .pending
            .outcomes()
            .iter()
            .skip(self.processed_count)
        {
            if count >= limit {
                break;
            }
            count += 1;

            let combat_result = self.round_manager.combat_manager.resolve(combat);
            self.builder.add(combat_result);
        }
        self.processed_count += count;

        return self.processed_count == self.outcomes().len();
    }

    /// Processes the remaining outcomes, updates the parent `RoundManager`, and returns the result.
    pub fn finish(mut self) -> &'a RoundResult<TBattlePhase, TUnit> {
        self.process(self.outcomes().len() - self.processed_count);
        let result = self.builder.build();
        self.round_manager.finish_round(result)
    }

    fn outcomes(&self) -> &[Prob<Combat<TBattlePhase, TUnit>>] {
        self.round_manager.last_round.pending.outcomes()
    }
}
