mod utils;
use wasm_bindgen::prelude::*;

use calc::stats::*;
use calc::{Force, QuantDistBuilder, Unit};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = "setPanicHook")]
pub fn set_panic_hook() {
    utils::set_panic_hook();
}

type Unit1942_2E = aa1942_2e::Unit;
type RoundManagerAA1942_2E = calc::RoundManager<
    aa1942_2e::CombatType,
    Unit1942_2E,
    aa1942_2e::Hit,
    aa1942_2e::RollSelector,
    aa1942_2e::SurvivorSelector,
>;
type CombatSequenceAA1942_2E = calc::CombatSequence<aa1942_2e::CombatType>;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Ruleset {
    AA1942_2E,
}

#[wasm_bindgen]
pub struct UnitProvider {
    ruleset: Ruleset,
}

#[wasm_bindgen]
impl UnitProvider {
    #[wasm_bindgen(constructor)]
    pub fn new(ruleset: Ruleset) -> Self {
        UnitProvider { ruleset }
    }

    #[wasm_bindgen(js_name = getUnitCount)]
    pub fn get_unit_count(&self) -> u32 {
        match self.ruleset {
            Ruleset::AA1942_2E => Unit1942_2E::all().len() as u32,
        }
    }

    #[wasm_bindgen(js_name = getUnitName)]
    pub fn get_unit_name(&self, index: u32) -> String {
        match self.ruleset {
            Ruleset::AA1942_2E => format!("{}", Unit1942_2E::all()[index as usize]),
        }
    }

    #[wasm_bindgen(js_name = getUnitIpc)]
    pub fn get_unit_ipc(&self, index: u32) -> u32 {
        match self.ruleset {
            Ruleset::AA1942_2E => Unit1942_2E::all()[index as usize].ipc(),
        }
    }

    #[wasm_bindgen(js_name = getUnitAttack)]
    pub fn get_unit_attack(&self, index: u32) -> u8 {
        match self.ruleset {
            Ruleset::AA1942_2E => Unit1942_2E::all()[index as usize].attack(),
        }
    }

    #[wasm_bindgen(js_name = getUnitDefense)]
    pub fn get_unit_defense(&self, index: u32) -> u8 {
        match self.ruleset {
            Ruleset::AA1942_2E => Unit1942_2E::all()[index as usize].defense(),
        }
    }
}

#[wasm_bindgen]
pub struct BattleBuilder {
    ruleset: Ruleset,
    attackers: QuantDistBuilder<Unit1942_2E>,
    defenders: QuantDistBuilder<Unit1942_2E>,
}

#[wasm_bindgen]
impl BattleBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(ruleset: Ruleset) -> Self {
        Self {
            ruleset,
            attackers: QuantDistBuilder::default(),
            defenders: QuantDistBuilder::default(),
        }
    }

    #[wasm_bindgen(js_name = addAttacker)]
    pub fn add_attacker(&mut self, unit_index: u32, count: u32) {
        match self.ruleset {
            Ruleset::AA1942_2E => {
                self.attackers
                    .add(Unit1942_2E::all()[unit_index as usize], count);
            }
        }
    }

    #[wasm_bindgen(js_name = addDefender)]
    pub fn add_defender(&mut self, unit_index: u32, count: u32) {
        match self.ruleset {
            Ruleset::AA1942_2E => {
                self.defenders
                    .add(Unit1942_2E::all()[unit_index as usize], count);
            }
        }
    }

    pub fn build(self) -> Battle {
        use std::rc::Rc;
        Battle::new(
            Rc::new(self.attackers.build()),
            Rc::new(self.defenders.build()),
        )
    }
}

#[wasm_bindgen]
pub struct Battle {
    round_manager: RoundManagerAA1942_2E,
    sequence: CombatSequenceAA1942_2E,
    total_pruned_p: f64,
    summarizer: Summarizer<aa1942_2e::CombatType, Unit1942_2E>,
}

#[wasm_bindgen]
impl Battle {
    fn new(attackers: Force<Unit1942_2E>, defenders: Force<Unit1942_2E>) -> Self {
        use core::convert::TryInto;
        let sequence = aa1942_2e::CombatType::create_sequence(&attackers, &defenders);
        let mut round_manager = aa1942_2e::create_round_manager(attackers, defenders);
        round_manager.set_prune_threshold(0.0000000001.try_into().unwrap());
        let summarizer = Summarizer::new(round_manager.last_round());
        Self {
            round_manager,
            sequence,
            total_pruned_p: 0.0,
            summarizer,
        }
    }

    #[wasm_bindgen(js_name = isComplete)]
    pub fn is_complete(&self) -> bool {
        self.round_manager.is_complete()
    }

    #[wasm_bindgen(js_name = roundIndex)]
    pub fn round_index(&self) -> u32 {
        self.round_manager.round_index() as u32
    }

    #[wasm_bindgen(js_name = roundCombatType)]
    pub fn round_combat_type(&self) -> String {
        format!(
            "{}",
            self.sequence.combat_at(self.round_manager.round_index())
        )
    }

    #[wasm_bindgen(js_name = roundStats)]
    pub fn round_stats(&self) -> RoundStats {
        let round_manager = &self.round_manager;
        let round = round_manager.last_round();
        let round_count = round_manager.round_index() as u32;

        RoundStats {
            round_count,
            combat_type: self.round_combat_type(),
            p: round.total_probability.into(),
            pending_count: round.pending.len() as u32,
            completed_count: round.completed.len() as u32,
            pruned_count: round.pruned_count as u32,
            pruned_p: round.pruned_p.into(),
        }
    }

    #[wasm_bindgen(js_name = cumulativeStats)]
    pub fn cumulative_stats(&self) -> CumulativeStats {
        let summary = self.summarizer.clone().summarize();
        CumulativeStats {
            attacker_win_p: summary.attacker.win_p.into(),
            defender_win_p: summary.defender.win_p.into(),
            draw_p: summary.draw_p.into(),
            attacker_ipc_lost: summary.attacker.ipc_lost.mean,
            defender_ipc_lost: summary.defender.ipc_lost.mean,
            attacker_ipc_stddev: summary.attacker.ipc.std_dev(),
            defender_ipc_stddev: summary.defender.ipc.std_dev(),
            pruned_p: self.total_pruned_p,
        }
    }

    pub fn advance_round(&mut self) {
        let round_manager = &mut self.round_manager;
        self.total_pruned_p = round_manager.pruned_p().into();
        let round = round_manager.advance_round();
        self.summarizer.add_round(&round);
    }

    pub fn default() -> Self {
        Default::default()
    }
}

impl Default for Battle {
    fn default() -> Self {
        use calc::Quant;
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
        Self::new(attackers, defenders)
    }
}

#[wasm_bindgen]
pub struct RoundStats {
    round_count: u32,
    combat_type: String,
    p: f64,
    pending_count: u32,
    completed_count: u32,
    pruned_count: u32,
    pruned_p: f64,
}

#[wasm_bindgen]
impl RoundStats {
    #[wasm_bindgen(getter = roundCount)]
    pub fn round_count(&self) -> u32 {
        self.round_count
    }

    #[wasm_bindgen(getter = combatType)]
    pub fn combat_type(&self) -> String {
        self.combat_type.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn probability(&self) -> f64 {
        self.p
    }

    #[wasm_bindgen(getter = pendingCount)]
    pub fn pending_count(&self) -> u32 {
        self.pending_count
    }

    #[wasm_bindgen(getter = completedCount)]
    pub fn completed_count(&self) -> u32 {
        self.completed_count
    }

    #[wasm_bindgen(getter = prunedCount)]
    pub fn pruned_count(&self) -> u32 {
        self.pruned_count
    }

    #[wasm_bindgen(getter = prunedP)]
    pub fn pruned_p(&self) -> f64 {
        self.pruned_p
    }
}

#[wasm_bindgen]
pub struct CumulativeStats {
    defender_win_p: f64,
    attacker_win_p: f64,
    draw_p: f64,
    pruned_p: f64,
    attacker_ipc_lost: f64,
    defender_ipc_lost: f64,
    attacker_ipc_stddev: f64,
    defender_ipc_stddev: f64,
}

#[wasm_bindgen]
impl CumulativeStats {
    #[wasm_bindgen(getter = attackerWinP)]
    pub fn attacker_win_p(&self) -> f64 {
        self.attacker_win_p
    }

    #[wasm_bindgen(getter = defenderWinP)]
    pub fn defender_win_p(&self) -> f64 {
        self.defender_win_p
    }

    #[wasm_bindgen(getter = drawP)]
    pub fn draw_p(&self) -> f64 {
        self.draw_p
    }

    #[wasm_bindgen(getter = prunedP)]
    pub fn pruned_p(&self) -> f64 {
        self.pruned_p
    }

    #[wasm_bindgen(getter = attackerIpcLost)]
    pub fn attacker_ipc_lost(&self) -> f64 {
        self.attacker_ipc_lost
    }

    #[wasm_bindgen(getter = defenderIpcLost)]
    pub fn defender_ipc_lost(&self) -> f64 {
        self.defender_ipc_lost
    }

    #[wasm_bindgen(getter = attackerIpcStdDev)]
    pub fn attacker_ipc_stddev(&self) -> f64 {
        self.attacker_ipc_stddev.sqrt()
    }

    #[wasm_bindgen(getter = defenderIpcStdDev)]
    pub fn defender_ipc_stddev(&self) -> f64 {
        self.defender_ipc_stddev.sqrt()
    }
}
