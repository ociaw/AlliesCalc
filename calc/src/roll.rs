use crate::*;
use std::marker::PhantomData;

/// Represents the roll of a single die.
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Copy, Clone)]
pub struct Roll<TUnit: Unit, THit: Hit<TUnit>> {
    /// The likeliness of this roll to succeed.
    pub strength: u8,
    /// The hit that results if this roll succeeds.
    pub hit: THit,
    phantom_unit: PhantomData<TUnit>,
}

impl<TUnit: Unit, THit: Hit<TUnit>> Roll<TUnit, THit> {
    /// Constructs a new `Roll` with the given strength and hit.
    pub fn new(strength: u8, hit: THit) -> Self {
        Roll {
            strength,
            hit,
            phantom_unit: PhantomData,
        }
    }
}

/// A type that selects rolls according to the combat context.
pub trait RollSelector<TBattlePhase, TUnit, THit>
where
    TBattlePhase: BattlePhase,
    TUnit: Unit,
    THit: Hit<TUnit>,
{
    /// Selects rolls based to the combat context.
    fn get_rolls(
        &self,
        context: &CombatContext<TBattlePhase, TUnit>,
    ) -> QuantDist<Roll<TUnit, THit>>;
}
