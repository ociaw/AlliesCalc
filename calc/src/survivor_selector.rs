use crate::{Force, Hit, ProbDist, QuantDist, Unit};

pub trait SurvivorSelector<TUnit, THit>
where
    TUnit: Unit,
    THit: Hit<TUnit>,
{
    fn select(
        &self,
        starting_force: &QuantDist<TUnit>,
        outcomes: &ProbDist<QuantDist<THit>>,
    ) -> ProbDist<Force<TUnit>>;
}
