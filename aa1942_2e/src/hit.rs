use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Hit {
    AllUnits,
    NotSubmarines,
    NotAirUnits,
    OnlyAirUnits,
}

impl calc::Hit<crate::Unit> for Hit {
    fn hits(self, unit: Unit) -> bool {
        unit.is_targetable() && match self {
            Hit::AllUnits => true,
            Hit::NotSubmarines => !unit.is_submarine(),
            Hit::NotAirUnits => !unit.is_air(),
            Hit::OnlyAirUnits => unit.is_air(),
        }
    }
}

impl Hit {
    pub fn order() -> [Hit; 4] {
        [
            Hit::OnlyAirUnits,
            Hit::NotAirUnits,
            Hit::NotSubmarines,
            Hit::AllUnits,
        ]
    }
}
