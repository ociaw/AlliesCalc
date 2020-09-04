use crate::{Prob, Probability};

#[derive(Debug, Clone, Copy)]
pub struct Pruner {
    threshold: Probability,
    count: usize,
    sum: Probability,
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
        probable.p < self.threshold
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
