use crate::*;
use std::marker::PhantomData;

pub struct CombatManager<
    TCombatType: CombatType,
    TUnit: Unit,
    THit: Hit<TUnit>,
    TRollSelector: RollSelector<TCombatType, TUnit, THit>,
    TSurvivorSelector: SurvivorSelector<TUnit, THit>,
> {
    attacker_survivor_selector: TSurvivorSelector,
    defender_survivor_selector: TSurvivorSelector,
    roll_selector: TRollSelector,
    phantom_combat_type: PhantomData<TCombatType>,
    phantom_hit: PhantomData<THit>,
    phantom_unit: PhantomData<TUnit>,
}

impl<TCombatType, THit, TUnit, TRollSelector, TSurvivorSelector>
    CombatManager<TCombatType, TUnit, THit, TRollSelector, TSurvivorSelector>
where
    TCombatType: CombatType,
    TUnit: Unit,
    THit: Hit<TUnit>,
    TRollSelector: RollSelector<TCombatType, TUnit, THit>,
    TSurvivorSelector: SurvivorSelector<TUnit, THit>,
{
    pub fn new(
        attacker_survivor_selector: TSurvivorSelector,
        defender_survivor_selector: TSurvivorSelector,
        roll_selector: TRollSelector,
    ) -> Self {
        CombatManager {
            attacker_survivor_selector,
            defender_survivor_selector,
            roll_selector,
            phantom_combat_type: PhantomData,
            phantom_hit: PhantomData,
            phantom_unit: PhantomData,
        }
    }

    pub fn resolve(
        &self,
        combat: &Prob<Combat<TCombatType, TUnit>>,
        next_combat_type: TCombatType,
    ) -> CombatResult<TCombatType, TUnit> {
        let probability = combat.p;
        let combat = &combat.item;
        let attackers = &combat.attackers;
        let defenders = &combat.defenders;

        let attack_context = CombatContext::from_combat(combat, false);
        let defense_context = CombatContext::from_combat(combat, true);

        let attack_strike = self.roll_selector.get_rolls(&attack_context);
        let defense_strike = self.roll_selector.get_rolls(&defense_context);

        let attacking_hits = roll_hits(&attack_strike);
        let defending_hits = roll_hits(&defense_strike);

        let surviving_attackers = self
            .attacker_survivor_selector
            .select(attackers, &defending_hits);
        let surviving_defenders = self
            .defender_survivor_selector
            .select(defenders, &attacking_hits);

        CombatResult {
            combat_type: combat.combat_type,
            next_combat_type,
            surviving_attackers,
            surviving_defenders,
            probability,
        }
    }
}
