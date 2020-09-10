use crate::Probability;
use core::{hash::Hash, ops::Mul};
use fnv::FnvBuildHasher;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Prob<T> {
    pub item: T,
    pub p: Probability,
}

impl<T> Prob<T> {
    pub fn new(item: T, p: Probability) -> Prob<T> {
        Prob { item, p }
    }
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
    outcomes: Vec<Prob<T>>,
}

impl<T> ProbDist<T> {
    pub fn outcomes(&self) -> &[Prob<T>] {
        &self.outcomes
    }

    pub fn len(&self) -> usize {
        self.outcomes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.outcomes.is_empty()
    }
}

impl<T: Eq + Hash> From<Vec<Prob<T>>> for ProbDist<T> {
    fn from(outcomes: Vec<Prob<T>>) -> Self {
        let mut builder = ProbDistBuilder::with_capacity(outcomes.len());
        for outcome in outcomes.into_iter() {
            builder.add_prob(outcome);
        }
        builder.build()
    }
}

impl<T> Default for ProbDist<T> {
    fn default() -> Self {
        Self {
            outcomes: Vec::<Prob<T>>::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProbDistBuilder<T: Eq + Hash> {
    outcomes: HashMap<T, Probability, FnvBuildHasher>,
}

impl<T: Eq + Hash> ProbDistBuilder<T> {
    pub fn new() -> Self {
        Self {
            outcomes: HashMap::default(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            outcomes: HashMap::with_capacity_and_hasher(capacity, Default::default()),
        }
    }

    pub fn build(self) -> ProbDist<T> {
        ProbDist {
            outcomes: self
                .outcomes
                .into_iter()
                .map(|t| Prob::new(t.0, t.1))
                .collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.outcomes.len()
    }
}

impl<T: Eq + Hash> Default for ProbDistBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Eq + Hash> ProbDistBuilder<T> {
    pub fn add(&mut self, item: T, p: Probability) {
        self.add_prob(Prob::new(item, p));
    }

    pub fn add_prob(&mut self, prob: Prob<T>) {
        if prob.p == Probability::zero() {
            return;
        }
        match self.outcomes.entry(prob.item) {
            std::collections::hash_map::Entry::Occupied(mut occupied) => {
                *occupied.get_mut() += prob.p;
            }
            std::collections::hash_map::Entry::Vacant(vacant) => {
                vacant.insert(prob.p);
            }
        }
    }
}

impl<T: Eq + Hash> From<ProbDist<T>> for ProbDistBuilder<T> {
    fn from(dist: ProbDist<T>) -> Self {
        let mut outcomes = HashMap::with_capacity_and_hasher(dist.len(), Default::default());
        for outcome in dist.outcomes.into_iter() {
            outcomes.insert(outcome.item, outcome.p);
        }
        ProbDistBuilder { outcomes }
    }
}
