use crate::{Force, Hit, ProbDist, QuantDist, Unit};

/// Selects surviors given a starting force and a distribution of hits.
pub trait SurvivorSelector<TUnit, THit>
where
    TUnit: Unit,
    THit: Hit<TUnit>,
{
    /// Returns all possible surviving forces and their probabilities.
    fn select(
        &self,
        starting_force: &QuantDist<TUnit>,
        outcomes: &ProbDist<QuantDist<THit>>,
    ) -> ProbDist<Force<TUnit>>;
}
