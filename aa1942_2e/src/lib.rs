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

        assert_eq!(stats.attacker_win_p(), Probability::zero());
        assert!(approx_eq!(f64, stats.defender_win_p().into(), 1.0 / 3.0, ulps = 1));
        assert!(approx_eq!(f64, stats.draw_p().into(), 2.0 / 3.0, ulps = 1));

        assert_eq!(stats.total_p(), Probability::one());
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

        assert!(approx_eq!(f64, stats.attacker_ipc_lost(), 3.0, ulps = 6));
        assert!(approx_eq!(f64, stats.defender_ipc_lost(), 6.0, ulps = 6));

        assert!(approx_eq!(f64, stats.attacker_win_p().into(), stats.defender_win_p().into(), ulps = 2));
        assert!(approx_eq!(f64, stats.attacker_win_p().into(), 0.5, ulps = 3));
        assert!(approx_eq!(f64, stats.defender_win_p().into(), 0.5, ulps = 3));
        assert_eq!(stats.draw_p(), Probability::zero());

        assert!(approx_eq!(f64, stats.total_p().into(), 1.0, ulps = 6));
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
        assert!(approx_eq!(f64, stats.total_p().into(), 1.0, ulps = 1));
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
        assert!(approx_eq!(f64, stats.total_p().into(), 1.0, ulps = 1));

        let attackers = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Infantry, 2), Quant::new(Unit::Artillery, 1)],
        });
        let defenders = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Infantry, 2), Quant::new(Unit::Artillery, 1)],
        });

        let (mut stats, mut round_manager) = setup(attackers, defenders);
        run_to_completion(&mut round_manager, &mut stats);

        assert!(f64::from(stats.attacker_win_p()) < f64::from(stats.defender_win_p()));
        assert!(approx_eq!(f64, stats.total_p().into(), 1.0, ulps = 1));
    }

    #[test]
    fn sub_plane_stalemate() {
        let attackers = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Submarine, 1)],
        });
        let defenders = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Fighter, 1)],
        });

        let (mut stats, mut round_manager) = setup(attackers, defenders);
        run_to_completion(&mut round_manager, &mut stats);

        assert_eq!(stats.attacker_win_p(), Probability::zero());
        assert_eq!(stats.defender_win_p(), Probability::zero());
        assert_eq!(stats.draw_p(), Probability::zero());
        assert_eq!(round_manager.last_round().total_probability(), Probability::one());
        assert!(round_manager.last_round().stalemate);
    }

    #[test]
    fn sub_plane_destroyer() {
        let attackers = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Submarine, 2)],
        });
        let defenders = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Fighter, 1), Quant::new(Unit::Destroyer, 1)],
        });

        let (mut stats, mut round_manager) = setup(attackers, defenders);
        let last_round = run_to_completion(&mut round_manager, &mut stats);

        // See test_probabilities.txt for probabilty calculations
        assert_eq!(stats.attacker_win_p(), Probability::zero());
        assert!(approx_eq!(f64, stats.defender_win_p().into(), 834.0 / 1679.0, ulps = 2));
        assert_eq!(stats.draw_p(), Probability::zero());
        assert_eq!(last_round.total_probability(), Probability::from_ratio(845, 1679));
        assert!(round_manager.last_round().stalemate);
    }

    #[test]
    fn antiair() {
        let attackers = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Fighter, 1)],
        });
        let defenders = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::AntiAir, 1)],
        });

        let (mut stats, mut round_manager) = setup(attackers, defenders.clone());
        let last_round = run_to_completion(&mut round_manager, &mut stats);

        assert!(approx_eq!(f64, stats.attacker_win_p().into(), 5.0 / 6.0, ulps = 1));
        assert!(approx_eq!(f64, stats.defender_win_p().into(), 1.0 / 6.0, ulps = 1));
        assert_eq!(stats.draw_p(), Probability::zero());
        assert_eq!(last_round.total_probability(), Probability::zero());
        assert!(!round_manager.last_round().stalemate);

        let attackers = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Fighter, 2)],
        });

        let (mut stats, mut round_manager) = setup(attackers, defenders.clone());
        let last_round = run_to_completion(&mut round_manager, &mut stats);
        assert!(approx_eq!(f64, stats.attacker_win_p().into(), 35.0 / 36.0, ulps = 8));
        assert!(approx_eq!(f64, stats.defender_win_p().into(), 1.0 / 36.0, ulps = 1));
        assert_eq!(stats.draw_p(), Probability::zero());
        assert_eq!(last_round.total_probability(), Probability::zero());
        assert!(!round_manager.last_round().stalemate);

        let attackers = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Fighter, 2), Quant::new(Unit::Bomber, 1)],
        });

        let (mut stats, mut round_manager) = setup(attackers, defenders.clone());
        let last_round = run_to_completion(&mut round_manager, &mut stats);
        assert!(approx_eq!(f64, stats.attacker_win_p().into(), 215.0 / 216.0, ulps = 7));
        assert!(approx_eq!(f64, stats.defender_win_p().into(), 1.0 / 216.0, ulps = 1));
        assert_eq!(stats.draw_p(), Probability::zero());
        assert_eq!(last_round.total_probability(), Probability::zero());
        assert!(!round_manager.last_round().stalemate);

        let attackers = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Fighter, 2), Quant::new(Unit::Bomber, 2)],
        });

        let (mut stats, mut round_manager) = setup(attackers, defenders.clone());
        let last_round = run_to_completion(&mut round_manager, &mut stats);
        assert!(approx_eq!(f64, stats.attacker_win_p().into(), 1.0, ulps = 1));
        assert_eq!(stats.defender_win_p(), Probability::zero());
        assert_eq!(stats.draw_p(), Probability::zero());
        assert_eq!(last_round.total_probability(), Probability::zero());
        assert!(!round_manager.last_round().stalemate);
    }

    #[test]
    fn battleship_undamaged() {
        let attackers = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Bomber, 1)],
        });
        let defenders = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Battleship, 1)],
        });

        let (mut stats, mut round_manager) = setup(attackers, defenders);
        let last_round = run_to_completion(&mut round_manager, &mut stats);

        // See test_probabilities.txt for probabilty calculations
        assert_eq!(stats.attacker_win_p(), Probability::from_ratio(1, 16));
        assert!(approx_eq!(f64, stats.defender_win_p().into(), 13.0 / 16.0, ulps = 1));
        assert!(approx_eq!(f64, stats.draw_p().into(), 2.0 / 16.0, ulps = 1));
        assert_eq!(last_round.total_probability(), Probability::zero());
        assert!(!round_manager.last_round().stalemate);
    }

    #[test]
    fn battleship_damaged() {
        let attackers = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Bomber, 1)],
        });
        let defenders = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::BattleshipDamaged, 1)],
        });

        let (mut stats, mut round_manager) = setup(attackers, defenders);
        let last_round = run_to_completion(&mut round_manager, &mut stats);

        assert!(approx_eq!(f64, stats.attacker_win_p().into(), 1.0 / 4.0, ulps = 1));
        assert!(approx_eq!(f64, stats.defender_win_p().into(), 1.0 / 4.0, ulps = 1));
        assert_eq!(stats.draw_p(), Probability::from_ratio(2, 4));
        assert_eq!(last_round.total_probability(), Probability::zero());
        assert!(!round_manager.last_round().stalemate);
    }

    #[test]
    fn reserve_tank() {
        // One tank is reserved by default
        let attackers = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Tank, 1), Quant::new(Unit::Bomber, 1)],
        });
        let defenders = Force::new(QuantDist {
            outcomes: vec![Quant::new(Unit::Tank, 1), Quant::new(Unit::Fighter, 1)],
        });

        let (mut stats, mut round_manager) = setup(attackers, defenders);
        let last_round = run_to_completion(&mut round_manager, &mut stats);

        // See test_probabilities.txt for probabilty calculations
        assert!(approx_eq!(f64, stats.attacker_win_p().into(), 2351.0 / 6545.0, ulps = 1));
        assert!(approx_eq!(f64, stats.defender_win_p().into(), 2726.0 / 6545.0, ulps = 1));
        assert!(approx_eq!(f64, stats.draw_p().into(), 1468.0 / 6545.0, ulps = 1));

        assert_eq!(last_round.total_probability(), Probability::zero());
        assert!(!round_manager.last_round().stalemate);
    }

    fn setup(attackers: Force<Unit>, defenders: Force<Unit>) -> (Statistics, RoundManager<CombatType, Unit, Hit, RollSelector, SurvivorSelector>) {
        let sequence = CombatType::create_sequence(&attackers, &defenders);
        let combat_manager = get_combat_manager();

        let stats = Statistics::new(&attackers, &defenders);
        let mut round_manager =
            RoundManager::new(combat_manager, sequence.clone(), attackers, defenders);
        round_manager.set_prune_threshold(Probability::zero());
        (stats, round_manager)
    }

    fn run_to_completion<'a>(round_manager: &'a mut RoundManager<CombatType, Unit, Hit, RollSelector, SurvivorSelector>, stats: &mut Statistics) -> &'a RoundResult<CombatType, Unit> {
        while !round_manager.is_complete() {
            stats.add_dist(&round_manager.advance_round().completed);
        }
        &round_manager.advance_round()
    }
}
