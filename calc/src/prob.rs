use crate::Probability;
use core::{hash::Hash, ops::Mul};
use fnv::FnvBuildHasher;
use std::collections::HashMap;

/// An item that has an associated `Probabilty` of occurrance.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Prob<T> {
    /// The inner item.
    pub item: T,
    /// The probability of occurrance.
    pub p: Probability,
}

impl<T> Prob<T> {
    /// Constructs a new `Prob` with item `item` and a probabilty of
    /// `Probability`.
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

/// A discrete probability distribution of `T`.
#[derive(Debug, Clone, PartialEq)]
pub struct ProbDist<T> {
    outcomes: Vec<Prob<T>>,
}

impl<T> ProbDist<T> {
    /// Returns a slice of `Prob<T>` representing this probability distribution.
    pub fn outcomes(&self) -> &[Prob<T>] {
        &self.outcomes
    }

    /// The number of discrete items in this distribution.
    pub fn len(&self) -> usize {
        self.outcomes.len()
    }

    /// Whether or not there are any items in this distribution.
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

/// A builder to facilitate piecemeal construction of a `ProbDist`.
#[derive(Debug, Clone, PartialEq)]
pub struct ProbDistBuilder<T: Eq + Hash> {
    outcomes: HashMap<T, Probability, FnvBuildHasher>,
}

impl<T: Eq + Hash> ProbDistBuilder<T> {
    /// Constructs a new `ProbDistBuilder`.
    pub fn new() -> Self {
        Self {
            outcomes: HashMap::default(),
        }
    }

    /// Constructs a new `ProbDistBuilder` with the given initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            outcomes: HashMap::with_capacity_and_hasher(capacity, Default::default()),
        }
    }

    /// Consumes this builder and returns a `ProbDist`.
    pub fn build(self) -> ProbDist<T> {
        ProbDist {
            outcomes: self
                .outcomes
                .into_iter()
                .map(|t| Prob::new(t.0, t.1))
                .collect(),
        }
    }

    /// The number of discrete items in this builder.
    pub fn len(&self) -> usize {
        self.outcomes.len()
    }

    /// Whether or not there are any items in this builder.
    pub fn is_empty(&self) -> bool {
        self.outcomes.is_empty()
    }

    /// Adds `item` to this distrbution with a probability of `p`.
    pub fn add(&mut self, item: T, p: Probability) {
        self.add_prob(Prob::new(item, p));
    }

    /// Adds the item in `prob` to this distrbution with the associated `Probability`.
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

impl<T: Clone + Eq + Hash> ProbDistBuilder<T> {
    /// Clones each item in this builder and returns a `ProbDist` with the
    /// clones.
    pub fn build_cloned(&self) -> ProbDist<T> {
        ProbDist {
            outcomes: self
                .outcomes
                .iter()
                .map(|t| Prob::new(t.0.clone(), *t.1))
                .collect(),
        }
    }
}

impl<T: Eq + Hash> Default for ProbDistBuilder<T> {
    fn default() -> Self {
        Self::new()
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
