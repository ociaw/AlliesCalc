use crate::*;
use calc::{CombatSequence, Force};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum CombatType {
    PreBattle,
    Bombardment,
    AntiAir,
    SurpriseStrike,
    General,
}

impl CombatType {
    pub fn create_sequence(
        attackers: &Force<Unit>,
        defenders: &Force<Unit>,
    ) -> CombatSequence<CombatType> {
        let mut start = Vec::new();

        let units = attackers
            .outcomes
            .iter()
            .chain(defenders.outcomes.iter())
            .filter(|q| q.count > 0)
            .map(|q| q.item)
            .collect::<Vec<_>>();
        if units
            .iter()
            .any(|u| u.combat_type() == CombatType::Bombardment)
        {
            start.push(CombatType::Bombardment);
        }
        if units.iter().any(|u| u.combat_type() == CombatType::AntiAir) {
            start.push(CombatType::AntiAir);
        }

        let mut cycle = Vec::new();
        if units
            .iter()
            .any(|u| u.combat_type() == CombatType::SurpriseStrike)
        {
            cycle.push(CombatType::SurpriseStrike);
        }
        cycle.push(CombatType::General);

        CombatSequence::new(start, cycle)
    }
}

impl calc::CombatType for CombatType {
    fn prebattle() -> Self {
        CombatType::PreBattle
    }
}

impl std::fmt::Display for CombatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CombatType::PreBattle => "Pre-Battle",
            CombatType::Bombardment => "Bombardment",
            CombatType::AntiAir => "Anti-Air",
            CombatType::SurpriseStrike => "Surprise Strike",
            CombatType::General => "General Combat",
        };

        write!(f, "{}", name)
    }
}
