use crate::*;
use std::marker::PhantomData;

/// Manages the resolution of individual combats.
pub struct CombatManager<
    TBattlePhase: BattlePhase,
    TUnit: Unit,
    THit: Hit<TUnit>,
    TRollSelector: RollSelector<TBattlePhase, TUnit, THit>,
    TSurvivorSelector: SurvivorSelector<TUnit, THit>,
> {
    attacker_survivor_selector: TSurvivorSelector,
    defender_survivor_selector: TSurvivorSelector,
    roll_selector: TRollSelector,
    roller: Roller<TUnit, THit>,
    phantom_battle_phase: PhantomData<TBattlePhase>,
    phantom_hit: PhantomData<THit>,
    phantom_unit: PhantomData<TUnit>,
}

impl<TBattlePhase, THit, TUnit, TRollSelector, TSurvivorSelector>
    CombatManager<TBattlePhase, TUnit, THit, TRollSelector, TSurvivorSelector>
where
    TBattlePhase: BattlePhase,
    TUnit: Unit,
    THit: Hit<TUnit>,
    TRollSelector: RollSelector<TBattlePhase, TUnit, THit>,
    TSurvivorSelector: SurvivorSelector<TUnit, THit>,
{
    /// Constructs a new combat manager with the given survivor selectors and roll selectors.
    pub fn new(
        attacker_survivor_selector: TSurvivorSelector,
        defender_survivor_selector: TSurvivorSelector,
        roll_selector: TRollSelector,
    ) -> Self {
        CombatManager {
            attacker_survivor_selector,
            defender_survivor_selector,
            roll_selector,
            phantom_battle_phase: PhantomData,
            phantom_hit: PhantomData,
            phantom_unit: PhantomData,
            roller: Default::default(),
        }
    }

    /// Resolves a combat into a combat result.
    pub fn resolve(
        &mut self,
        combat: &Prob<Combat<TBattlePhase, TUnit>>,
    ) -> CombatResult<TBattlePhase, TUnit> {
        let probability = combat.p;
        let combat = &combat.item;
        let attackers = &combat.attackers;
        let defenders = &combat.defenders;

        let attack_context = CombatContext::from_combat(combat, Side::Attacker);
        let defense_context = CombatContext::from_combat(combat, Side::Defender);

        let attack_strike = self.roll_selector.get_rolls(&attack_context);
        let defense_strike = self.roll_selector.get_rolls(&defense_context);

        let defending_hits = self.roller.roll_hits(defense_strike);

        let surviving_attackers = self
            .attacker_survivor_selector
            .select(attackers, &defending_hits);

        let attacking_hits = self.roller.roll_hits(attack_strike);
        let surviving_defenders = self
            .defender_survivor_selector
            .select(defenders, &attacking_hits);

        CombatResult {
            battle_phase: combat.battle_phase,
            surviving_attackers,
            surviving_defenders,
            probability,
        }
    }
}
