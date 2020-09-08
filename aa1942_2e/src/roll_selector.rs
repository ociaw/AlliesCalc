use crate::*;
use calc::{QuantDist, Roll};

pub struct RollSelector;

#[derive(Debug)]
struct Context {
    pub combat: CombatType,
    pub defending: bool,
    pub boost_count: u32,
    pub hostile_air_count: u32,
    pub friendly_anti_sub: bool,
    pub hostile_unsurprisable: bool,
}

impl Context {
    fn convert(combat_context: &calc::CombatContext<CombatType, Unit>) -> Context {
        Context {
            combat: combat_context.combat_type,
            defending: combat_context.defending,
            boost_count: combat_context
                .friendlies()
                .outcomes
                .iter()
                .filter(|u| u.item.is_booster())
                .map(|u| u.count)
                .sum(),
            hostile_air_count: combat_context
                .hostiles()
                .outcomes
                .iter()
                .filter(|u| u.item.is_air())
                .map(|u| u.count)
                .sum(),
            friendly_anti_sub: combat_context
                .friendlies()
                .outcomes
                .iter()
                .any(|u| u.item.is_anti_sub() && u.count > 0),
            hostile_unsurprisable: combat_context
                .hostiles()
                .outcomes
                .iter()
                .any(|u| u.item.is_unsurprisable() && u.count > 0),
        }
    }
}

impl calc::RollSelector<CombatType, Unit, Hit> for RollSelector {
    fn get_rolls(
        &self,
        context: &calc::CombatContext<CombatType, Unit>,
    ) -> calc::QuantDist<Roll<Unit, Hit>> {
        let force = context.friendlies();
        let context = Context::convert(context);
        let current_combat = context.combat;
        let mut rolls = QuantDist { outcomes: vec![] };
        for quant in &force.outcomes {
            let unit = quant.item;
            let count = quant.count;

            let unit_combat = if unit.combat_type() == CombatType::SurpriseStrike
                && context.hostile_unsurprisable
            {
                CombatType::General
            } else {
                unit.combat_type()
            };

            if current_combat != unit_combat {
                continue;
            }

            let boosted_count = match unit.boosted_strength() {
                Some(_) => core::cmp::min(context.boost_count, count),
                None => 0,
            };
            let base_count = count - boosted_count;

            let base_strength = {
                use calc::Unit;
                if context.defending {
                    unit.defense()
                } else {
                    unit.attack()
                }
            };
            let boosted_strength = unit.boosted_strength().unwrap_or(0);

            let hit = {
                let hit = unit.hit();
                if hit == Hit::NotSubmarines && context.friendly_anti_sub {
                    Hit::AllUnits
                } else {
                    hit
                }
            };

            let multiplier = if unit.combat_type() == CombatType::AntiAir {
                core::cmp::min(3, context.hostile_air_count)
            } else {
                1
            };

            rolls.add(Roll::new(base_strength, hit), base_count * multiplier);
            rolls.add(Roll::new(boosted_strength, hit), boosted_count * multiplier);
        }
        rolls
    }
}
