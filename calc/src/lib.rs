pub mod stats;

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

pub use combat::{BattlePhase, Combat, CombatContext, CombatResult, CombatSequence, Side};
pub use combat_manager::CombatManager;
pub use hit::Hit;
pub use prob::{Prob, ProbDist, ProbDistBuilder};
pub use probability::Probability;
pub use pruner::Pruner;
pub use quant::{Quant, QuantDist, QuantDistBuilder};
pub use roll::{Roll, RollSelector};
pub use roller::Roller;
pub use round_manager::RoundManager;
pub use round_result::{RoundResult, RoundResultBuilder};
pub use survivor_selector::SurvivorSelector;
pub use unit::{Force, Unit};

pub use roller::roll_hits;
