mod battle_phase;
mod hit;
mod roll_selector;
mod survivor_selector;
mod unit;

pub use crate::stats::*;
pub use battle_phase::BattlePhase;
pub use hit::Hit;
pub use roll_selector::RollSelector;
pub use survivor_selector::SurvivorSelector;
pub use unit::Unit;

use calc::*;
pub fn get_combat_manager() -> CombatManager<BattlePhase, Unit, Hit, RollSelector, SurvivorSelector>
{
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
        roll_selector,
    )
}

pub fn create_round_manager(
    attackers: Force<Unit>,
    defenders: Force<Unit>,
) -> RoundManager<BattlePhase, Unit, Hit, RollSelector, SurvivorSelector> {
    let sequence = BattlePhase::create_sequence(&attackers, &defenders);
    let combat_manager = get_combat_manager();

    RoundManager::new(combat_manager, sequence.clone(), attackers, defenders)
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::*;
    use fixed::types::U1F63;
    const EPSILON: U1F63 = U1F63::from_bits(1);

    fn assert_prob_eq(first: Probability, second: Probability, ulps: i64) -> bool {
        let diff: U1F63 = if first > second { first - second } else { second - first }.into();
        let tolerance = EPSILON * ulps as u64;
        diff <= tolerance
        //approx_eq!(f64, first.into(), second.into(), ulps = ulps)
    }

    #[test]
    fn bombardment() {
        let attackers = Force::new(vec![Quant::new(Unit::BombardingBattleship, 1)].into());
        let defenders = Force::new(vec![Quant::new(Unit::Infantry, 1)].into());

        let (mut summarizer, mut round_manager) = setup(attackers, defenders);
        run_to_completion(&mut round_manager, &mut summarizer);
        let summary = summarizer.summarize();

        assert!(!round_manager.last_round().stalemate);
        assert_eq!(summary.attacker.ipc_lost.mean, 0.0);
        assert_eq!(summary.defender.ipc_lost.mean, 2.0);
        assert_eq!(summary.completed_combats.len(), 2);

        assert_eq!(summary.attacker.win_p, Probability::zero());
        assert!(assert_prob_eq(
            summary.defender.win_p,
            Probability::from_ratio(1, 3),
            0
        ), "expected: {}, actual: {}, diff: {}, eps: {}", Probability::from_ratio(1, 3), summary.defender.win_p, summary.defender.win_p - Probability::from_ratio(1, 3), EPSILON);
        assert!(assert_prob_eq(
            summary.draw_p,
            Probability::from_ratio(2, 3),
            2
        ));

        assert_eq!(summary.total_p, Probability::one());
    }

    #[test]
    fn surprise_strike() {
        let attackers = Force::new(vec![Quant::new(Unit::Submarine, 1)].into());
        let defenders = Force::new(vec![Quant::new(Unit::Cruiser, 1)].into());

        let (mut summarizer, mut round_manager) = setup(attackers, defenders);
        run_to_completion(&mut round_manager, &mut summarizer);
        let summary = summarizer.summarize();

        assert!(approx_eq!(
            f64,
            summary.attacker.ipc_lost.mean,
            3.0,
            ulps = 6
        ));
        assert!(approx_eq!(
            f64,
            summary.defender.ipc_lost.mean,
            6.0,
            ulps = 6
        ));

        assert!(assert_prob_eq(
            summary.attacker.win_p,
            summary.defender.win_p,
            2
        ));
        assert!(assert_prob_eq(
            summary.attacker.win_p,
            Probability::from_ratio(1, 2),
            3
        ));
        assert!(assert_prob_eq(
            summary.defender.win_p,
            Probability::from_ratio(1, 2),
            3
        ));
        assert_eq!(summary.draw_p, Probability::zero());

        assert!(assert_prob_eq(summary.total_p, Probability::one(), 6));
    }

    #[test]
    fn surprise_strike_cancel() {
        let attackers = Force::new(vec![Quant::new(Unit::Submarine, 1)].into());
        let defenders = Force::new(vec![Quant::new(Unit::Destroyer, 1)].into());

        let (mut summarizer, mut round_manager) = setup(attackers, defenders);
        run_to_completion(&mut round_manager, &mut summarizer);
        let summary = summarizer.summarize();

        assert_eq!(summary.attacker.win_p, summary.defender.win_p);
        assert!(assert_prob_eq(summary.total_p, Probability::one(), 1));
    }

    #[test]
    fn artillery_boost() {
        let attackers = Force::new(
            vec![
                Quant::new(Unit::Infantry, 1),
                Quant::new(Unit::Artillery, 1),
            ]
            .into(),
        );
        let defenders = Force::new(
            vec![
                Quant::new(Unit::Infantry, 1),
                Quant::new(Unit::Artillery, 1),
            ]
            .into(),
        );

        let (mut summarizer, mut round_manager) = setup(attackers, defenders);
        run_to_completion(&mut round_manager, &mut summarizer);
        let summary = summarizer.summarize();

        assert!(assert_prob_eq(
            summary.attacker.win_p,
            summary.defender.win_p,
            1
        ));
        assert!(
            assert_prob_eq(summary.total_p, Probability::one(), 2),
            "actual: {}",
            summary.total_p
        );

        let attackers = Force::new(
            vec![
                Quant::new(Unit::Infantry, 2),
                Quant::new(Unit::Artillery, 1),
            ]
            .into(),
        );
        let defenders = Force::new(
            vec![
                Quant::new(Unit::Infantry, 2),
                Quant::new(Unit::Artillery, 1),
            ]
            .into(),
        );

        let (mut summarizer, mut round_manager) = setup(attackers, defenders);
        run_to_completion(&mut round_manager, &mut summarizer);
        let summary = summarizer.summarize();

        assert!(f64::from(summary.attacker.win_p) < f64::from(summary.defender.win_p));
        assert!(assert_prob_eq(summary.total_p, Probability::one(), 1));
    }

    #[test]
    fn sub_plane_stalemate() {
        let attackers = Force::new(vec![Quant::new(Unit::Submarine, 1)].into());
        let defenders = Force::new(vec![Quant::new(Unit::Fighter, 1)].into());

        let (mut summarizer, mut round_manager) = setup(attackers, defenders);
        run_to_completion(&mut round_manager, &mut summarizer);
        let summary = summarizer.summarize();

        assert_eq!(summary.attacker.win_p, Probability::zero());
        assert_eq!(summary.defender.win_p, Probability::zero());
        assert_eq!(summary.draw_p, Probability::zero());
        assert_eq!(
            round_manager.last_round().total_probability(),
            Probability::one()
        );
        assert!(round_manager.last_round().stalemate);
    }

    #[test]
    fn sub_plane_destroyer() {
        let attackers = Force::new(vec![Quant::new(Unit::Submarine, 2)].into());
        let defenders =
            Force::new(vec![Quant::new(Unit::Fighter, 1), Quant::new(Unit::Destroyer, 1)].into());

        let (mut summarizer, mut round_manager) = setup(attackers, defenders);
        let last_round = run_to_completion(&mut round_manager, &mut summarizer);
        let summary = summarizer.summarize();

        // See test_probabilities.txt for probabilty calculations
        assert_eq!(summary.attacker.win_p, Probability::zero());
        assert!(assert_prob_eq(
            summary.defender.win_p,
            Probability::from_ratio(834, 1679),
            2
        ),  "{} - {}", summary.defender.win_p, Probability::from_ratio(834, 1679));
        assert_eq!(summary.draw_p, Probability::zero());
        assert!(assert_prob_eq(
            last_round.total_probability(),
            Probability::from_ratio(845, 1679),
            2
        ), "{} - {}", last_round.total_probability(), Probability::from_ratio(845, 1679));
        assert!(round_manager.last_round().stalemate);
    }

    #[test]
    fn antiair() {
        let attackers = Force::new(vec![Quant::new(Unit::Fighter, 1)].into());
        let defenders = Force::new(vec![Quant::new(Unit::AntiAir, 1)].into());

        let (mut summarizer, mut round_manager) = setup(attackers, defenders.clone());
        let last_round = run_to_completion(&mut round_manager, &mut summarizer);
        let summary = summarizer.summarize();

        assert!(assert_prob_eq(
            summary.attacker.win_p,
            Probability::from_ratio(5, 6),
            1
        ));
        assert!(assert_prob_eq(
            summary.defender.win_p,
            Probability::from_ratio(1, 6),
            1
        ));
        assert_eq!(summary.draw_p, Probability::zero());
        assert!(assert_prob_eq(
            last_round.total_probability(),
            Probability::zero(),
            1
        ));
        assert!(!round_manager.last_round().stalemate);

        let attackers = Force::new(vec![Quant::new(Unit::Fighter, 2)].into());

        let (mut summarizer, mut round_manager) = setup(attackers, defenders.clone());
        let last_round = run_to_completion(&mut round_manager, &mut summarizer);
        let summary = summarizer.summarize();
        assert!(assert_prob_eq(
            summary.attacker.win_p,
            Probability::from_ratio(35, 36),
            8
        ));
        assert!(assert_prob_eq(
            summary.defender.win_p,
            Probability::from_ratio(1, 36),
            1
        ));
        assert_eq!(summary.draw_p, Probability::zero());
        assert_eq!(last_round.total_probability(), Probability::zero());
        assert!(!round_manager.last_round().stalemate);

        let attackers =
            Force::new(vec![Quant::new(Unit::Fighter, 2), Quant::new(Unit::Bomber, 1)].into());

        let (mut summarizer, mut round_manager) = setup(attackers, defenders.clone());
        let last_round = run_to_completion(&mut round_manager, &mut summarizer);
        let summary = summarizer.summarize();
        assert!(assert_prob_eq(
            summary.attacker.win_p,
            Probability::from_ratio(215, 216),
            7
        ));
        assert!(assert_prob_eq(
            summary.defender.win_p,
            Probability::from_ratio(1, 216),
            1
        ));
        assert_eq!(summary.draw_p, Probability::zero());
        assert_eq!(last_round.total_probability(), Probability::zero());
        assert!(!round_manager.last_round().stalemate);

        let attackers =
            Force::new(vec![Quant::new(Unit::Fighter, 2), Quant::new(Unit::Bomber, 2)].into());

        let (mut summarizer, mut round_manager) = setup(attackers, defenders);
        let last_round = run_to_completion(&mut round_manager, &mut summarizer);
        let summary = summarizer.summarize();
        assert!(assert_prob_eq(
            summary.attacker.win_p,
            Probability::one(),
            1
        ));
        assert_eq!(summary.defender.win_p, Probability::zero());
        assert_eq!(summary.draw_p, Probability::zero());
        assert_eq!(last_round.total_probability(), Probability::zero());
        assert!(!round_manager.last_round().stalemate);
    }

    #[test]
    fn battleship_undamaged() {
        let attackers = Force::new(vec![Quant::new(Unit::Bomber, 1)].into());
        let defenders = Force::new(vec![Quant::new(Unit::Battleship, 1)].into());

        let (mut summarizer, mut round_manager) = setup(attackers, defenders);
        let last_round = run_to_completion(&mut round_manager, &mut summarizer);
        let summary = summarizer.summarize();

        // See test_probabilities.txt for probabilty calculations
        assert_eq!(summary.attacker.win_p, Probability::from_ratio(1, 16));
        assert!(assert_prob_eq(
            summary.defender.win_p,
            Probability::from_ratio(13, 16),
            1
        ));
        assert!(assert_prob_eq(
            summary.draw_p,
            Probability::from_ratio(2, 16),
            1
        ));
        assert_eq!(last_round.total_probability(), Probability::zero());
        assert!(!round_manager.last_round().stalemate);
    }

    #[test]
    fn battleship_damaged() {
        let attackers = Force::new(vec![Quant::new(Unit::Bomber, 1)].into());
        let defenders = Force::new(vec![Quant::new(Unit::BattleshipDamaged, 1)].into());

        let (mut summarizer, mut round_manager) = setup(attackers, defenders);
        let last_round = run_to_completion(&mut round_manager, &mut summarizer);
        let summary = summarizer.summarize();

        assert!(assert_prob_eq(
            summary.attacker.win_p,
            Probability::from_ratio(1, 4),
            1
        ));
        assert!(assert_prob_eq(
            summary.defender.win_p,
            Probability::from_ratio(1, 4),
            1
        ));
        assert_eq!(summary.draw_p, Probability::from_ratio(2, 4));
        assert_eq!(last_round.total_probability(), Probability::zero());
        assert!(!round_manager.last_round().stalemate);
    }

    #[test]
    fn reserve_tank() {
        // One tank is reserved by default
        let attackers =
            Force::new(vec![Quant::new(Unit::Tank, 1), Quant::new(Unit::Bomber, 1)].into());
        let defenders =
            Force::new(vec![Quant::new(Unit::Tank, 1), Quant::new(Unit::Fighter, 1)].into());

        let (mut summarizer, mut round_manager) = setup(attackers, defenders);
        let last_round = run_to_completion(&mut round_manager, &mut summarizer);
        let summary = summarizer.summarize();

        // See test_probabilities.txt for probabilty calculations
        assert!(assert_prob_eq(
            summary.attacker.win_p,
            Probability::from_ratio(2351, 6545),
            2
        ));
        assert!(assert_prob_eq(
            summary.defender.win_p,
            Probability::from_ratio(2726, 6545),
            2
        ));
        assert!(assert_prob_eq(
            summary.draw_p,
            Probability::from_ratio(1468, 6545),
            2
        ));

        assert_eq!(last_round.total_probability(), Probability::zero());
        assert!(!round_manager.last_round().stalemate);
    }

    fn setup(
        attackers: Force<Unit>,
        defenders: Force<Unit>,
    ) -> (
        Summarizer<BattlePhase, Unit>,
        RoundManager<BattlePhase, Unit, Hit, RollSelector, SurvivorSelector>,
    ) {
        let sequence = BattlePhase::create_sequence(&attackers, &defenders);
        let combat_manager = get_combat_manager();

        let mut round_manager =
            RoundManager::new(combat_manager, sequence.clone(), attackers, defenders);
        round_manager.set_prune_threshold(Probability::zero());
        let summary = Summarizer::new(round_manager.last_round());
        (summary, round_manager)
    }

    fn run_to_completion<'a>(
        round_manager: &'a mut RoundManager<BattlePhase, Unit, Hit, RollSelector, SurvivorSelector>,
        summary: &mut Summarizer<BattlePhase, Unit>,
    ) -> &'a RoundResult<BattlePhase, Unit> {
        while !round_manager.is_complete() {
            summary.add_round(&round_manager.advance_round());
        }
        round_manager.advance_round()
    }
}
