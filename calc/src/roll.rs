use std::marker::PhantomData;
use crate::*;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Copy, Clone)]
pub struct Roll<TUnit: Unit, THit: Hit<TUnit>> {
    pub strength: u8,
    pub hit: THit,
    phantom_unit: PhantomData<TUnit>
}

impl<TUnit: Unit, THit: Hit<TUnit>> Roll<TUnit, THit> {
    pub fn new(strength: u8, hit: THit) -> Self {
        Roll { strength, hit, phantom_unit: PhantomData }
    }
}

pub trait RollSelector<TCombatType, TUnit, THit>
where
    TCombatType: CombatType,
    TUnit: Unit,
    THit: Hit<TUnit>,
{
    fn get_rolls(&self, context: &CombatContext<TCombatType, TUnit>) -> QuantDist<Roll<TUnit, THit>>;
}
