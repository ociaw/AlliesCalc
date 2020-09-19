use aa1942_2e::CombatType as CombatType1942_2E;
use aa1942_2e::Unit as Unit1942_2E;
use calc::*;
use std::convert::TryInto;

fn main() {
    let attackers = Force::new(
        vec![
            Quant::new(Unit1942_2E::Infantry, 40),
            Quant::new(Unit1942_2E::Artillery, 40),
            Quant::new(Unit1942_2E::Tank, 40),
            Quant::new(Unit1942_2E::Fighter, 40),
            Quant::new(Unit1942_2E::Bomber, 40),
            Quant::new(Unit1942_2E::BombardingCruiser, 40),
            Quant::new(Unit1942_2E::BombardingBattleship, 40),
        ]
        .into(),
    );
    let defenders = Force::new(
        vec![
            Quant::new(Unit1942_2E::Infantry, 55),
            Quant::new(Unit1942_2E::Artillery, 40),
            Quant::new(Unit1942_2E::Tank, 40),
            Quant::new(Unit1942_2E::Fighter, 40),
            Quant::new(Unit1942_2E::Bomber, 40),
            Quant::new(Unit1942_2E::AntiAir, 40),
        ]
        .into(),
    );

    let sequence = CombatType1942_2E::create_sequence(&attackers, &defenders);
    let mut stats = Statistics::new(&attackers, &defenders);
    let mut round_manager = aa1942_2e::create_round_manager(attackers, defenders);
    round_manager.set_prune_threshold(0.0000000001.try_into().unwrap());

    let start = std::time::SystemTime::now();
    while !round_manager.is_complete() {
        let round_index = round_manager.round_index() + 1;
        println!(
            "Round {} - {}",
            round_index,
            sequence.combat_at(round_index)
        );
        let last_round = round_manager.advance_round();
        stats.add_dist(&last_round.completed);

        println!(
            "Pending: {}, Completed: {}, ∑P: {:>9.6}",
            last_round.pending.len(),
            last_round.completed.len(),
            last_round.total_probability()
        );
    }

    match start.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("Took {} seconds", elapsed.as_millis() as f64 / 2000.0);
        }
        Err(e) => {
            // an error occurred!
            println!("Timing error: {:?}", e);
        }
    }

    println!(
        "{} rounds and {} outcomes analyzed, {} ({:.2}%) outcomes discarded",
        round_manager.round_index(),
        stats.total_count(),
        round_manager.pruned_count(),
        round_manager.pruned_p() * 100.0
    );
    println!("Winner      Prob.");
    println!("Attack:    {:>5.2}%", stats.attacker_win_p() * 100.0);
    println!("Defend:    {:>5.2}%", stats.defender_win_p() * 100.0);
    println!("Draw:      {:>5.2}%", stats.draw_p() * 100.0);
    if round_manager.last_round().stalemate {
        println!(
            "Stalemate: {:>5.2}%",
            round_manager.last_round().total_probability() * 100.0
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
