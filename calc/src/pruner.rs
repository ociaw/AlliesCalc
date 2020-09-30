use crate::{Prob, Probability};

/// Prunes outcomes with a probability at or below the threshold.
#[derive(Debug, Clone, Copy)]
pub struct Pruner {
    /// The threshold at which outcomes are pruned.
    pub threshold: Probability,
    /// The number of outcomes that were pruned.
    pub count: usize,
    /// The cumulative probability of outcomes pruned.
    pub sum: Probability,
}

impl Pruner {
    /// Constructs a new `Pruner` with the given pruning threshold.
    pub fn new(threshold: Probability) -> Pruner {
        Pruner {
            threshold,
            count: 0,
            sum: Default::default(),
        }
    }

    /// Returns whether or not the given `Prob` will be pruned.
    ///
    /// `probable` will be considered prunable if `probable.p <= self.threshold`.
    pub fn check<T>(&self, probable: &Prob<T>) -> bool {
        probable.p <= self.threshold
    }

    /// Returns whether or not the given `Prob` will be pruned and accumulates the probability and count.
    ///
    /// `probable` will be considered pruned if `probable.p <= self.threshold`.
    pub fn prune<T>(&mut self, probable: &Prob<T>) -> bool {
        if !self.check(probable) {
            return false;
        }

        self.count += 1;
        self.sum += probable.p;
        true
    }
}

impl Default for Pruner {
    fn default() -> Self {
        use std::convert::TryInto;
        Self {
            threshold: 0.000000001.try_into().unwrap(),
            count: 0,
            sum: Default::default(),
        }
    }
}
