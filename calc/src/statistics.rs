use crate::*;

pub struct Statistics {
    attacker_ipc_initial: f64,
    defender_ipc_initial: f64,
    attacker_count: u64,
    defender_count: u64,
    draw_count: u64,
    attacker_p: Probability,
    defender_p: Probability,
    draw_p: Probability,
    total_p: Probability,
    attacker_ipc_mean: f64,
    defender_ipc_mean: f64,
    attacker_ipc_s: f64,
    defender_ipc_s: f64,
}

impl Statistics {
    pub fn new<TUnit: Unit>(attackers: &Force<TUnit>, defenders: &Force<TUnit>) -> Self {
        let attacker_ipc_initial = sum_ipc(attackers) as f64;
        let defender_ipc_initial = sum_ipc(defenders) as f64;
        Statistics {
            attacker_ipc_initial,
            defender_ipc_initial,
            attacker_count: 0,
            defender_count: 0,
            draw_count: 0,
            attacker_p: Default::default(),
            defender_p: Default::default(),
            draw_p: Default::default(),
            total_p: Default::default(),
            attacker_ipc_mean: 0.0,
            defender_ipc_mean: 0.0,
            attacker_ipc_s: 0.0,
            defender_ipc_s: 0.0,
        }
    }

    pub fn attacker_win_p(&self) -> Probability {
        self.attacker_p
    }

    pub fn defender_win_p(&self) -> Probability {
        self.defender_p
    }

    pub fn draw_p(&self) -> Probability {
        self.draw_p
    }

    pub fn total_p(&self) -> Probability {
        self.total_p
    }

    pub fn total_count(&self) -> u64 {
        self.attacker_count + self.defender_count + self.draw_count
    }

    pub fn attacker_ipc_lost(&self) -> f64 {
        (self.attacker_ipc_initial - self.attacker_ipc_mean) * f64::from(self.total_p)
    }

    pub fn defender_ipc_lost(&self) -> f64 {
        (self.defender_ipc_initial - self.defender_ipc_mean) * f64::from(self.total_p)
    }

    pub fn attacker_ipc_variance(&self) -> f64 {
        self.attacker_ipc_s / f64::from(self.total_p)
    }

    pub fn defender_ipc_variance(&self) -> f64 {
        self.defender_ipc_s / f64::from(self.total_p)
    }

    pub fn add_dist<TCombatType: CombatType, TUnit: Unit>(
        &mut self,
        combat: &ProbDist<Combat<TCombatType, TUnit>>,
    ) {
        for combat in combat.outcomes() {
            self.add(combat);
        }
    }

    pub fn add<TCombatType: CombatType, TUnit: Unit>(
        &mut self,
        combat: &Prob<Combat<TCombatType, TUnit>>,
    ) {
        let p = combat.p;
        let combat = &combat.item;
        match combat.winner() {
            Some(side) => match side {
                Side::Attacker => {
                    self.attacker_count += 1;
                    self.attacker_p += p;
                }
                Side::Defender => {
                    self.defender_count += 1;
                    self.defender_p += p;
                }
            },
            None => {
                self.draw_count += 1;
                self.draw_p += p;
            }
        }

        self.total_p += p;

        let attacker_ipc = sum_ipc(&combat.attackers) as f64;
        update_means(
            attacker_ipc,
            p,
            self.total_p,
            &mut self.attacker_ipc_mean,
            &mut self.attacker_ipc_s,
        );
        let defender_ipc = sum_ipc(&combat.defenders) as f64;
        update_means(
            defender_ipc,
            p,
            self.total_p,
            &mut self.defender_ipc_mean,
            &mut self.defender_ipc_s,
        );
    }
}

fn update_means(value: f64, p: Probability, total_p: Probability, mean: &mut f64, s: &mut f64) {
    let old_mean = *mean;
    let p = f64::from(p);
    *mean += (p / f64::from(total_p)) * (value - *mean);
    *s += p * (value - old_mean) * (value - *mean);
}

fn sum_ipc<TUnit: Unit>(force: &Force<TUnit>) -> u32 {
    let mut sum = 0;
    for quant in force.outcomes() {
        sum += quant.count * quant.item.ipc();
    }
    sum
}
