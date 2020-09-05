mod combattype;
mod hit;
mod roll_selector;
mod survivor_selector;
mod unit;

pub use combattype::CombatType;
pub use hit::Hit;
pub use roll_selector::RollSelector;
pub use survivor_selector::SurvivorSelector;
pub use unit::Unit;

use calc::*;
pub fn get_combat_manager() -> CombatManager<CombatType, Unit, Hit, RollSelector, SurvivorSelector> {
    let attacker_survivor_selector = SurvivorSelector {
        removal_order: SurvivorSelector::default_attacker_order(),
        reserved: Some(Unit::Tank),
    };
    let defender_survivor_selector = SurvivorSelector {
        removal_order: SurvivorSelector::default_defender_order(),
        reserved: None,
    };

    let roll_selector = RollSelector {};
    CombatManager::new(
        attacker_survivor_selector,
        defender_survivor_selector,
        roll_selector
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::*;

    #[test]
    fn bombardment() {
        let attackers = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::BombardingBattleship, 1)],
        });
        let defenders = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Infantry, 1)],
        });

        let (mut stats, mut round_manager) = setup(attackers, defenders);
        run_to_completion(&mut round_manager, &mut stats);

        assert!(!round_manager.last_round().stalemate);
        assert_eq!(stats.attacker_ipc_lost(), 0.0);
        assert_eq!(stats.defender_ipc_lost(), 2.0);
        assert_eq!(stats.total_count(), 2);

        assert_eq!(stats.attacker_win_p(), 0.0);
        assert!(approx_eq!(f64, stats.defender_win_p(), 1.0 / 3.0, ulps = 1));
        assert!(approx_eq!(f64, stats.draw_p(), 2.0 / 3.0, ulps = 1));

        assert!(approx_eq!(f64, stats.total_p(), 1.0, epsilon = 0.00000000002));
    }

    #[test]
    fn surprise_strike() {
        let attackers = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Submarine, 1)],
        });
        let defenders = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Cruiser, 1)],
        });

        let (mut stats, mut round_manager) = setup(attackers, defenders);
        run_to_completion(&mut round_manager, &mut stats);

        assert!(approx_eq!(f64, stats.attacker_ipc_lost(), 3.0, epsilon = 0.00000000004));
        assert!(approx_eq!(f64, stats.defender_ipc_lost(), 6.0, epsilon = 0.00000000007));

        assert!(approx_eq!(f64, stats.attacker_win_p(), stats.defender_win_p(), ulps = 2));
        assert!(approx_eq!(f64, stats.attacker_win_p(), 0.5, epsilon = 0.000000000006));
        assert!(approx_eq!(f64, stats.defender_win_p(), 0.5, epsilon = 0.000000000006));
        assert_eq!(stats.draw_p(), 0.0);

        assert!(approx_eq!(f64, stats.total_p(), 1.0, epsilon = 0.00000000002));
    }

    #[test]
    fn surprise_strike_cancel() {
        let attackers = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Submarine, 1)],
        });
        let defenders = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Destroyer, 1)],
        });

        let (mut stats, mut round_manager) = setup(attackers, defenders);
        run_to_completion(&mut round_manager, &mut stats);

        assert_eq!(stats.attacker_win_p(), stats.defender_win_p());
    }

    #[test]
    fn artillery_boost() {
        let attackers = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Infantry, 1), Quant::new(Unit::Artillery, 1)],
        });
        let defenders = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Infantry, 1), Quant::new(Unit::Artillery, 1)],
        });

        let (mut stats, mut round_manager) = setup(attackers, defenders);
        run_to_completion(&mut round_manager, &mut stats);

        assert_eq!(stats.attacker_win_p(), stats.defender_win_p());
    }

    fn setup(attackers: Force<Unit>, defenders: Force<Unit>) -> (Statistics, RoundManager<CombatType, Unit, Hit, RollSelector, SurvivorSelector>) {
        let sequence = CombatType::create_sequence(&attackers, &defenders);
        let combat_manager = get_combat_manager();

        let stats = Statistics::new(&attackers, &defenders);
        let round_manager =
            RoundManager::new(combat_manager, sequence.clone(), attackers, defenders);
        (stats, round_manager)
    }

    fn run_to_completion(round_manager: &mut RoundManager<CombatType, Unit, Hit, RollSelector, SurvivorSelector>, stats: &mut Statistics) {
        while !round_manager.is_complete() {
            stats.add_dist(&round_manager.advance_round().completed);
        }
    }
}