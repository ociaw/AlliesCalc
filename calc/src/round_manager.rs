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
    sequence: CombatSequence<TBattlePhase>,
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
    /// Constructs a new `RoundManager` with the given `CombatManager`, `CombatSequence`,
    /// attacking force, and defending force.
    pub fn new(
        combat_manager: CombatManager<TBattlePhase, TUnit, THit, TRollSelector, TSurvivorSelector>,
        sequence: CombatSequence<TBattlePhase>,
        attackers: Force<TUnit>,
        defenders: Force<TUnit>,
    ) -> Self {
        let round_index = 0;
        let battle_phase = sequence.combat_at(round_index + 1);
        RoundManager {
            combat_manager,
            sequence,
            prune_threshold: Default::default(),
            round_index,
            last_round: RoundResult::new(battle_phase, attackers, defenders),
            last_probability: Probability::zero(),
            probability_run_count: 0,
        }
    }

    /// Computes the next round of the battle and returns the result.
    #[allow(clippy::float_cmp)]
    pub fn advance_round(&mut self) -> &RoundResult<TBattlePhase, TUnit> {
        self.round_index += 1;
        let next_battle_phase = self.sequence.combat_at(self.round_index + 1);
        let pruner = Pruner::new(self.prune_threshold);
        let mut result = RoundResultBuilder::new(self.round_index, next_battle_phase, pruner);
        for combat in self.last_round.pending.outcomes() {
            let combat_result = self.combat_manager.resolve(combat);
            result.add(combat_result);
        }

        let mut result = result.build();
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
