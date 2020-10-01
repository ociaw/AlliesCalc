pub mod stats;

mod battle_phase;
mod combat;
mod combat_manager;
mod hit;
mod prob;
mod probability;
mod pruner;
mod quant;
mod roll;
mod roller;
mod round_manager;
mod round_result;
mod survivor_selector;
mod unit;

pub use battle_phase::*;
pub use combat::*;
pub use combat_manager::CombatManager;
pub use hit::Hit;
pub use prob::*;
pub use probability::Probability;
pub use pruner::Pruner;
pub use quant::*;
pub use roll::*;
pub use roller::Roller;
pub use round_manager::*;
pub use round_result::*;
pub use survivor_selector::SurvivorSelector;
pub use unit::*;

pub use roller::roll_hits;

/// The side of combat - attacker or defender.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Side {
    Attacker,
    Defender,
}
