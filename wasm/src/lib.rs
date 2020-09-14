mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn set_panic_hook() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub struct RoundManager {
    round_index: u32
}

#[wasm_bindgen]
impl RoundManager {
    pub fn new() -> Self {
        RoundManager {
            round_index: 0
        }
    }

    pub fn is_complete(&self) -> bool {
        false
    }

    pub fn round_index(&self) -> u32 {
        self.round_index
    }

    pub fn stats(&self) -> Statistics {
        Statistics {
            defender_win_p: 0.25,
            attacker_win_p: 0.25,
            draw_p: 0.5
        }
    }

    pub fn advance_round(&mut self) {
        self.round_index += 1;
    }
}

#[wasm_bindgen]
pub struct Statistics {
    defender_win_p: f64,
    attacker_win_p: f64,
    draw_p: f64
}

#[wasm_bindgen]
impl Statistics {
    pub fn defender_win_p(&self) -> f64 {
        self.defender_win_p
    }

    pub fn attacker_win_p(&self) -> f64 {
        self.attacker_win_p
    }

    pub fn draw_p(&self) -> f64 {
        self.draw_p
    }
}
