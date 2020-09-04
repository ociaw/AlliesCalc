use crate::*;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Copy, Clone)]
pub struct Roll<THit: Hit> {
    pub strength: u8,
    pub hit: THit,
}

impl<THit: Hit> Roll<THit> {
    pub fn new(strength: u8, hit: THit) -> Roll<THit> {
        Roll { strength, hit }
    }
}

pub trait RollSelector<TCombatType, THit, TUnit>
where
    TCombatType: CombatType,
    THit: Hit,
    TUnit: Unit,
{
    fn get_rolls(&self, context: &CombatContext<TCombatType, TUnit>) -> QuantDist<Roll<THit>>;
}
