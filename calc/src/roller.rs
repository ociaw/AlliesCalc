use crate::*;
use statrs::distribution::{Binomial, Discrete};
use std::collections::hash_map::Iter;
use std::collections::HashMap;

pub struct Roller<TUnit: Unit, THit: Hit<TUnit>> {
    cache: HashMap<QuantDist<Roll<TUnit, THit>>, ProbDist<QuantDist<THit>>>,
}

impl<TUnit: Unit, THit: Hit<TUnit>> Roller<TUnit, THit> {
    pub fn roll_hits(
        &mut self,
        strike: QuantDist<Roll<TUnit, THit>>,
    ) -> &ProbDist<QuantDist<THit>> {
        self.cache.entry(strike).or_insert_with_key(roll_hits)
    }
}

impl<TUnit: Unit, THit: Hit<TUnit>> Default for Roller<TUnit, THit> {
    fn default() -> Self {
        Self {
            cache: HashMap::<QuantDist<Roll<TUnit, THit>>, ProbDist<QuantDist<THit>>>::new(),
        }
    }
}

pub fn roll_hits<TUnit: Unit, THit: Hit<TUnit>>(
    strike: &QuantDist<Roll<TUnit, THit>>,
) -> ProbDist<QuantDist<THit>> {
    use std::convert::TryInto;

    let mut hit_dists = HashMap::with_capacity(strike.outcomes().len());
    for quant in strike.outcomes() {
        let roll = quant.item;
        let roll_count = quant.count;
        let hit = roll.hit;
        let p = roll.strength as f64 / 6.0;
        let binomial = Binomial::new(p, roll_count as u64).unwrap();

        let mut dist = ProbDistBuilder::with_capacity(roll_count as usize);
        for hit_count in 0..=roll_count {
            let mass = binomial.pmf(hit_count as u64);
            dist.add(hit_count, mass.try_into().unwrap());
        }
        let dist = dist.build();

        let entry = hit_dists.entry(hit);
        match entry {
            std::collections::hash_map::Entry::Occupied(mut existing) => {
                existing.insert(combine_dists(existing.get(), &dist));
            }
            std::collections::hash_map::Entry::Vacant(vacant) => {
                vacant.insert(dist);
            }
        }
    }

    let mut results = ProbDistBuilder::new();
    combine_hit_dists(
        &mut hit_dists.iter(),
        &mut Vec::new(),
        Probability::one(),
        &mut results,
    );
    results.build()
}

fn combine_dists(destination: &ProbDist<u32>, source: &ProbDist<u32>) -> ProbDist<u32> {
    let mut result = ProbDistBuilder::with_capacity(destination.len());
    for first in destination.outcomes() {
        for second in source.outcomes() {
            let hit_count = first.item + second.item;
            let p = first.p * second.p;
            result.add(hit_count, p);
        }
    }
    result.build()
}

fn combine_hit_dists<TUnit: Unit, THit: Hit<TUnit>>(
    hit_dists: &mut Iter<THit, ProbDist<u32>>,
    hit_stack: &mut Vec<Quant<THit>>,
    current_p: Probability,
    results: &mut ProbDistBuilder<QuantDist<THit>>,
) {
    match hit_dists.next() {
        None => {
            let mut builder = QuantDistBuilder::with_capacity(hit_stack.len());
            for hit in hit_stack.iter() {
                builder.add(hit.item, hit.count);
            }
            results.add(builder.build(), current_p);
            if hit_stack.is_empty() {
                return;
            }
        }
        Some((hit, dist)) => {
            for prob in dist.outcomes() {
                hit_stack.push(Quant {
                    item: *hit,
                    count: prob.item,
                });
                let next_p = current_p * prob.p;
                combine_hit_dists(hit_dists, hit_stack, next_p, results);
                hit_stack.pop();
            }
        }
    }
}
