use crate::Probability;
use std::ops::Mul;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Prob<T> {
    pub item: T,
    pub p: Probability,
}

impl<T> Mul<Probability> for Prob<T> {
    type Output = Prob<T>;

    fn mul(self, rhs: Probability) -> Self::Output {
        Prob {
            item: self.item,
            p: self.p * rhs,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProbDist<T> {
    pub outcomes: Vec<Prob<T>>,
}

impl<T> ProbDist<T> {
    pub fn new() -> ProbDist<T> {
        ProbDist {
            outcomes: Vec::<Prob<T>>::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> ProbDist<T> {
        ProbDist {
            outcomes: Vec::<Prob<T>>::with_capacity(capacity),
        }
    }
}

impl<T> Default for ProbDist<T> {
    fn default() -> Self {
        Self {
            outcomes: Vec::<Prob<T>>::new(),
        }
    }
}

impl<T: Eq> ProbDist<T> {
    pub fn add(&mut self, outcome: Prob<T>) {
        match self.outcomes.iter().position(|o| o.item == outcome.item) {
            Some(index) => {
                self.outcomes[index] = Prob {
                    item: outcome.item,
                    p: self.outcomes[index].p + outcome.p,
                };
            }
            None => {
                self.outcomes.push(outcome);
            }
        }
    }
}
