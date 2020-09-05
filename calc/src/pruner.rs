use crate::{Prob, Probability};

#[derive(Debug, Clone, Copy)]
pub struct Pruner {
    pub threshold: Probability,
    pub count: usize,
    pub sum: Probability,
}

impl Pruner {
    pub fn new(threshold: Probability) -> Pruner {
        Pruner {
            threshold,
            count: 0,
            sum: 0.0,
        }
    }

    pub fn check<T>(&self, probable: &Prob<T>) -> bool {
        probable.p <= self.threshold
    }

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
        Self {
            threshold: 0.000000001,
            count: 0,
            sum: 0.0,
        }
    }
}
