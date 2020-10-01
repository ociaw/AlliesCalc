use crate::*;
use calc::{PhaseSequence, Force};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum BattlePhase {
    PreBattle,
    Bombardment,
    AntiAir,
    SurpriseStrike,
    General,
}

impl BattlePhase {
    pub fn create_sequence(
        attackers: &Force<Unit>,
        defenders: &Force<Unit>,
    ) -> PhaseSequence<BattlePhase> {
        let mut start = Vec::new();

        let units = attackers
            .outcomes()
            .iter()
            .chain(defenders.outcomes().iter())
            .filter(|q| q.count > 0)
            .map(|q| q.item)
            .collect::<Vec<_>>();
        if units
            .iter()
            .any(|u| u.battle_phase() == BattlePhase::Bombardment)
        {
            start.push(BattlePhase::Bombardment);
        }
        if units
            .iter()
            .any(|u| u.battle_phase() == BattlePhase::AntiAir)
        {
            start.push(BattlePhase::AntiAir);
        }

        let mut cycle = Vec::new();
        if units
            .iter()
            .any(|u| u.battle_phase() == BattlePhase::SurpriseStrike)
        {
            cycle.push(BattlePhase::SurpriseStrike);
        }
        cycle.push(BattlePhase::General);

        PhaseSequence::new(start, cycle)
    }
}

impl calc::BattlePhase for BattlePhase {
    fn prebattle() -> Self {
        BattlePhase::PreBattle
    }
}

impl std::fmt::Display for BattlePhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            BattlePhase::PreBattle => "Pre-Battle",
            BattlePhase::Bombardment => "Bombardment",
            BattlePhase::AntiAir => "Anti-Air",
            BattlePhase::SurpriseStrike => "Surprise Strike",
            BattlePhase::General => "General Combat",
        };

        write!(f, "{}", name)
    }
}
