use aa1942_2e::Unit as Unit1942_2E;
use calc::stats::*;
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

    let sequence = aa1942_2e::BattlePhase::create_sequence(&attackers, &defenders);
    let mut round_manager = aa1942_2e::create_round_manager(attackers, defenders);
    round_manager.set_prune_threshold(0.0000000001.try_into().unwrap());
    let mut summarizer = Summarizer::new(round_manager.last_round());

    println!("Round {} - {}", 0, sequence.combat_at(0));
    println!("Attacker Stats:");
    print_round_side_summary(&summarizer.prebattle().attacker);
    println!("Defender Stats:");
    print_round_side_summary(&summarizer.prebattle().defender);

    let start = std::time::SystemTime::now();
    while !round_manager.is_complete() {
        let round_index = round_manager.round_index() + 1;
        println!(
            "Round {} - {}",
            round_index,
            sequence.combat_at(round_index)
        );

        const PROCESS_LIMIT: usize = 5000;
        let mut processor = round_manager.round_processor();

        while !processor.process(PROCESS_LIMIT) {
            println!(
                " {:5.2}% complete",
                processor.processed_outcomes() as f64 / processor.total_outcomes() as f64 * 100.0
            );
        }

        let last_round = processor.finish();
        let summary = summarizer.add_round(last_round);

        println!("Attacker Stats:");
        print_round_side_summary(&summary.attacker);
        println!("Defender Stats:");
        print_round_side_summary(&summary.defender);

        println!(
            " Pending: {}, Completed: {}, ∑P: {:>9.6}",
            last_round.pending.len(),
            last_round.completed.len(),
            last_round.total_probability()
        );
    }

    match start.elapsed() {
        Ok(elapsed) => {
            println!("Took {} seconds", elapsed.as_millis() as f64 / 1000.0);
        }
        Err(e) => {
            // an error occurred!
            println!("Timing error: {:?}", e);
        }
    }

    let summary = summarizer.summarize();

    println!(
        "{} rounds and {} unique outcomes analyzed, {:.2}% of outcomes discarded",
        summary.round_count(),
        summary.completed_combats.len(),
        summary.pruned_p * 100.0
    );
    println!("Winner      Prob.");
    println!("Attack:    {:>5.2}%", summary.attacker.win_p * 100.0);
    println!("Defend:    {:>5.2}%", summary.defender.win_p * 100.0);
    println!("Draw:      {:>5.2}%", summary.draw_p * 100.0);
    if round_manager.last_round().stalemate {
        println!(
            "Stalemate: {:>5.2}%",
            round_manager.last_round().total_probability() * 100.0
        );
    }
    println!("Total:     {:>8.5}%", summary.total_p * 100.0);

    println!(
        "Attacker Loss - μ: {:>6.2} IPC, σ: {:>5.2} IPC",
        summary.prebattle.attacker.ipc.mean - summary.attacker.ipc.mean,
        summary.attacker.ipc.std_dev(),
    );
    println!(
        "Defender Loss - μ: {:>6.2} IPC, σ: {:>5.2} IPC",
        summary.prebattle.defender.ipc.mean - summary.defender.ipc.mean,
        summary.defender.ipc.std_dev(),
    );
}

fn print_round_side_summary(summary: &RoundSideSummary) {
    println!("  IPC:      {}", summary.ipc);
    println!("  Strength: {}", summary.strength);
    println!("  Units:    {}", summary.unit_count);
}
