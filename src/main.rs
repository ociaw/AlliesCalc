use aa1942_2e::CombatType as CombatType1942_2E;
use aa1942_2e::RollSelector as RollSelector1942_2E;
use aa1942_2e::SurvivorSelector as SurvivorSelector1942_2E;
use aa1942_2e::Unit as Unit1942_2E;
use calc::*;

fn main() {
    let attackers = Force::new(QuantDist {
        outcomes: vec![Quant::new(Unit1942_2E::Submarine, 1)],
    });
    let defenders = Force::new(QuantDist {
        outcomes: vec![Quant::new(Unit1942_2E::Cruiser, 1)],
    });

    let sequence = CombatType1942_2E::create_sequence(&attackers, &defenders);
    let attacker_survivor_selector = aa1942_2e::SurvivorSelector {
        removal_order: SurvivorSelector1942_2E::default_attacker_order(),
        reserved: Some(Unit1942_2E::Tank),
    };
    let defender_survivor_selector = aa1942_2e::SurvivorSelector {
        removal_order: SurvivorSelector1942_2E::default_defender_order(),
        reserved: None,
    };

    let roll_selector = RollSelector1942_2E {};
    let combat_manager = CombatManager::new(
        attacker_survivor_selector,
        defender_survivor_selector,
        roll_selector,
    );

    let mut stats = Statistics::new(&attackers, &defenders);
    let mut round_manager =
        RoundManager::new(combat_manager, sequence.clone(), attackers, defenders);

    while !round_manager.is_complete() {
        let round_index = round_manager.round_index() + 1;
        println!(
            "Round {} - {}",
            round_index,
            sequence.combat_at(round_index)
        );
        round_manager.advance_round();
        let last_round = round_manager.last_round();
        stats.add_dist(&last_round.completed);

        println!(
            "Pending: {}, Completed: {}, ∑P: {:>9.6}",
            last_round.pending.outcomes.len(),
            last_round.completed.outcomes.len(),
            last_round.total_probability()
        );
    }

    println!(
        "{} rounds and {} outcomes analyzed",
        round_manager.round_index(),
        stats.total_count()
    );
    println!("Winner      Prob.");
    println!("Attack:    {:>5.2}%", stats.attacker_win_p() * 100.0);
    println!("Defend:    {:>5.2}%", stats.defender_win_p() * 100.0);
    println!("Draw:      {:>5.2}%", stats.draw_p() * 100.0);
    if round_manager.last_round().stalemate {
        println!(
            "Stalemate: {:>5.2}%",
            round_manager.last_round().total_probability()
        );
    }

    println!(
        "Attacker Loss - μ: {:>6.2} IPC, σ: {:>5.2} IPC",
        stats.attacker_ipc_lost(),
        stats.attacker_ipc_variance().sqrt()
    );
    println!(
        "Defender Loss - μ: {:>6.2} IPC, σ: {:>5.2} IPC",
        stats.defender_ipc_lost(),
        stats.defender_ipc_variance().sqrt()
    );
}
