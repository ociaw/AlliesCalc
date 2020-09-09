use crate::*;
use calc::{Force, ProbDist, QuantDistBuilder};
use std::rc::Rc;

pub struct SurvivorSelector {
    pub removal_order: Vec<Unit>,
    pub reserved: Option<Unit>,
}

impl SurvivorSelector {
    pub fn default_attacker_order() -> Vec<Unit> {
        vec![
            Unit::Battleship,
            Unit::Infantry,
            Unit::Artillery,
            Unit::Tank,
            Unit::Submarine,
            Unit::Destroyer,
            Unit::Fighter,
            Unit::Bomber,
            Unit::Cruiser,
            Unit::Carrier,
            Unit::BattleshipDamaged,
            Unit::AntiAir,
        ]
    }

    pub fn default_defender_order() -> Vec<Unit> {
        vec![
            Unit::Battleship,
            Unit::Infantry,
            Unit::Artillery,
            Unit::AntiAir,
            Unit::Tank,
            Unit::Submarine,
            Unit::Destroyer,
            Unit::Bomber,
            Unit::Fighter,
            Unit::Cruiser,
            Unit::Carrier,
            Unit::BattleshipDamaged,
        ]
    }

    fn select_survivors(
        &self,
        candidates: &QuantDist<Unit>,
        hits: &QuantDist<Hit>,
    ) -> QuantDist<Unit> {
        let mut survivors: QuantDistBuilder<Unit> = candidates.clone().into();
        for hit in &Hit::order() {
            let mut count = hits.count(hit);
            count -= self.remove_dead(&mut survivors, *hit, count, self.reserved);
            // If any are left, take the reseved unit as well
            if count > 0 && self.reserved.is_some() {
                self.remove_dead(&mut survivors, *hit, count, None);
            }
        }
        survivors.build()
    }

    fn remove_dead(
        &self,
        candidates: &mut QuantDistBuilder<Unit>,
        hit: Hit,
        count: u32,
        reserved: Option<Unit>,
    ) -> u32 {
        let mut count = count;
        let mut total_removed = 0;

        for unit in &self.removal_order {
            use calc::Hit;
            if !hit.hits(*unit) {
                continue;
            }

            // If the current unit is reserved, keep at least one of it
            let remove_count = match reserved {
                Some(reserved) if reserved == *unit => {
                    let candidate_count = candidates.count(&unit);
                    std::cmp::min(
                        count,
                        if candidate_count > 1 {
                            candidate_count - 1
                        } else {
                            0
                        },
                    )
                }
                _ => count,
            };

            let removed = candidates.remove(&unit, remove_count);
            total_removed += removed;
            count -= removed;

            if let Some(replacement) = unit.damaged() {
                candidates.add(replacement, removed);
            }

            if count == 0 {
                return total_removed;
            }
        }

        total_removed
    }

    fn without_nontargetable(force: &QuantDist<Unit>) -> QuantDist<Unit> {
        let mut force: QuantDistBuilder<Unit> = force.clone().into();
        for unit in &Unit::all() {
            if unit.is_targetable() {
                continue;
            }
            force.remove_all(&unit);
        }
        force.build()
    }
}

impl calc::SurvivorSelector<Unit, Hit> for SurvivorSelector {
    fn select(
        &self,
        starting_force: &QuantDist<Unit>,
        hit_dists: &ProbDist<QuantDist<Hit>>,
    ) -> ProbDist<Force<Unit>> {
        let mut result = ProbDistBuilder::<Force<Unit>>::new();
        let starting_force = &Self::without_nontargetable(starting_force);
        for hit_dist in hit_dists.outcomes() {
            let survivors = self.select_survivors(starting_force, &hit_dist.item);
            result.add(Rc::new(survivors), hit_dist.p);
        }
        result.build()
    }
}
