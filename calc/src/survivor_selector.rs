use crate::{Force, Hit, ProbDist, QuantDist, Unit};

pub trait SurvivorSelector<THit, TUnit>
where
    THit: Hit,
    TUnit: Unit,
{
    fn select(
        &self,
        starting_force: &QuantDist<TUnit>,
        outcomes: &ProbDist<QuantDist<THit>>,
    ) -> ProbDist<Force<TUnit>>;
}
