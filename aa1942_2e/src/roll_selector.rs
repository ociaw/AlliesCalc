use crate::*;
use calc::{QuantDist, Roll};

pub struct RollSelector;

#[derive(Debug)]
struct Context {
    pub combat: CombatType,
    pub defending: bool,
    pub friendly_artillery: u32,
    pub hostile_air: u32,
    pub friendly_destroyer: bool,
    pub hostile_destroyer: bool,
}

impl Context {
    fn convert(combat_context: &calc::CombatContext<CombatType, Unit>) -> Context {
        Context {
            combat: combat_context.combat_type,
            defending: combat_context.defending,
            friendly_artillery: combat_context
                .friendlies()
                .outcomes
                .iter()
                .filter(|u| u.item == Unit::Artillery)
                .map(|u| u.count)
                .sum(),
            hostile_air: combat_context
                .hostiles()
                .outcomes
                .iter()
                .filter(|u| u.item.is_air())
                .map(|u| u.count)
                .sum(),
            friendly_destroyer: combat_context
                .friendlies()
                .outcomes
                .iter()
                .any(|u| u.item == Unit::Destroyer && u.count > 0),
            hostile_destroyer: combat_context
                .hostiles()
                .outcomes
                .iter()
                .any(|u| u.item == Unit::Destroyer && u.count > 0),
        }
    }
}

impl calc::RollSelector<CombatType, Unit, Hit> for RollSelector
{
    fn get_rolls(
        &self,
        context: &calc::CombatContext<CombatType, Unit>,
    ) -> calc::QuantDist<Roll<Unit, Hit>> {
        let force = context.friendlies();
        let context = Context::convert(context);
        let combat = context.combat;
        let mut rolls = QuantDist { outcomes: vec![] };
        for quant in &force.outcomes {
            let count = quant.count;
            match quant.item {
                Unit::Infantry => {
                    if combat == CombatType::General {
                        if context.defending {
                            rolls.add(Roll::new(2, Hit::NotSubmarines), count);
                        } else {
                            let weak_count = if count < context.friendly_artillery { 0 } else { count - context.friendly_artillery };
                            let strong_count = std::cmp::min(count, context.friendly_artillery);
                            rolls.add(Roll::new(1, Hit::NotSubmarines), weak_count);
                            rolls.add(Roll::new(2, Hit::NotSubmarines), strong_count);
                        }
                    }
                }
                Unit::Artillery => {
                    if combat == CombatType::General {
                        rolls.add(Roll::new(2, Hit::NotSubmarines), count);
                    }
                }
                Unit::Tank => {
                    if combat == CombatType::General {
                        rolls.add(Roll::new(3, Hit::NotSubmarines), count);
                    }
                }
                Unit::AntiAir => {
                    if combat == CombatType::AntiAir {
                        rolls.add(
                            Roll::new(1, Hit::OnlyAirUnits),
                            std::cmp::min(3, context.hostile_air) * count,
                        );
                    }
                }
                Unit::BombardingCruiser => {
                    if combat == CombatType::Bombardment {
                        rolls.add(Roll::new(3, Hit::NotSubmarines), count);
                    }
                }
                Unit::BombardingBattleship => {
                    if combat == CombatType::Bombardment {
                        rolls.add(Roll::new(4, Hit::NotSubmarines), count);
                    }
                }
                Unit::Fighter => {
                    if combat == CombatType::General {
                        let strength = if context.defending { 4 } else { 3 };
                        let hit = if context.friendly_destroyer {
                            Hit::AllUnits
                        } else {
                            Hit::NotSubmarines
                        };
                        rolls.add(Roll::new(strength, hit), count);
                    }
                }
                Unit::Bomber => {
                    if combat == CombatType::General {
                        let strength = if context.defending { 1 } else { 4 };
                        let hit = if context.friendly_destroyer {
                            Hit::AllUnits
                        } else {
                            Hit::NotSubmarines
                        };
                        rolls.add(Roll::new(strength, hit), count);
                    }
                }
                Unit::Submarine => {
                    let strength = if context.defending { 1 } else { 2 };
                    if (combat == CombatType::SurpriseStrike && !context.hostile_destroyer)
                        || (combat == CombatType::General && context.hostile_destroyer)
                    {
                        rolls.add(Roll::new(strength, Hit::NotAirUnits), count);
                    }
                }
                Unit::Destroyer => {
                    if combat == CombatType::General {
                        rolls.add(Roll::new(2, Hit::AllUnits), count);
                    }
                }
                Unit::Cruiser => {
                    if combat == CombatType::General {
                        rolls.add(Roll::new(3, Hit::AllUnits), count);
                    }
                }
                Unit::Carrier => {}
                Unit::Battleship | Unit::BattleshipDamaged => {
                    if combat == CombatType::General {
                        rolls.add(Roll::new(4, Hit::AllUnits), count);
                    }
                }
            }
        }
        rolls
    }
}
