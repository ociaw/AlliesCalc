#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Hit {
    AllUnits,
    NotSubmarines,
    NotAirUnits,
    OnlyAirUnits,
}

impl calc::Hit for Hit {}

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
