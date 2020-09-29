use crate::Side;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Unit {
    Infantry,
    Artillery,
    Tank,
    AntiAir,
    BombardingCruiser,
    BombardingBattleship,
    Fighter,
    Bomber,
    Submarine,
    Destroyer,
    Cruiser,
    Carrier,
    Battleship,
    BattleshipDamaged,
}

impl Unit {
    pub fn is_air(self) -> bool {
        self == Unit::Fighter || self == Unit::Bomber
    }

    pub fn is_submarine(self) -> bool {
        self == Unit::Submarine
    }

    pub fn is_targetable(self) -> bool {
        !(self == Unit::BombardingCruiser || self == Unit::BombardingBattleship)
    }

    pub fn is_anti_sub(self) -> bool {
        self == Unit::Destroyer
    }

    pub fn is_unsurprisable(self) -> bool {
        self == Unit::Destroyer
    }

    pub fn is_booster(self) -> bool {
        self == Unit::Artillery
    }

    pub fn combat_type(self) -> crate::CombatType {
        use crate::CombatType;

        match self {
            Unit::BombardingBattleship | Unit::BombardingCruiser => CombatType::Bombardment,
            Unit::AntiAir => CombatType::AntiAir,
            Unit::Submarine => CombatType::SurpriseStrike,
            _ => CombatType::General,
        }
    }

    pub fn hit(self) -> crate::Hit {
        use crate::Hit;
        match self {
            Unit::AntiAir => Hit::OnlyAirUnits,
            Unit::Submarine => Hit::NotAirUnits,
            Unit::Destroyer
            | Unit::Cruiser
            | Unit::Carrier
            | Unit::Battleship
            | Unit::BattleshipDamaged => Hit::AllUnits,
            _ => Hit::NotSubmarines,
        }
    }

    pub fn boosted_strength(self) -> Option<u8> {
        if self == Unit::Infantry {
            Some(2)
        } else {
            None
        }
    }

    pub fn damaged(self) -> Option<Self> {
        match self {
            Unit::Battleship => Some(Unit::BattleshipDamaged),
            _ => None,
        }
    }

    pub fn all() -> [Unit; 14] {
        [
            Unit::Infantry,
            Unit::Artillery,
            Unit::Tank,
            Unit::AntiAir,
            Unit::BombardingCruiser,
            Unit::BombardingBattleship,
            Unit::Fighter,
            Unit::Bomber,
            Unit::Submarine,
            Unit::Destroyer,
            Unit::Cruiser,
            Unit::Carrier,
            Unit::Battleship,
            Unit::BattleshipDamaged,
        ]
    }
}

impl calc::Unit for Unit {
    fn ipc(self) -> u32 {
        match self {
            Unit::Infantry => 3,
            Unit::Artillery => 4,
            Unit::Tank => 6,
            Unit::AntiAir => 5,
            Unit::BombardingCruiser => 0,
            Unit::BombardingBattleship => 0,
            Unit::Fighter => 10,
            Unit::Bomber => 12,
            Unit::Submarine => 6,
            Unit::Destroyer => 8,
            Unit::Cruiser => 12,
            Unit::Carrier => 14,
            Unit::Battleship => 20,
            Unit::BattleshipDamaged => 20,
        }
    }

    fn strength(self, side: Side) -> u8 {
        match side {
            Side::Attacker => self.attack(),
            Side::Defender => self.defense(),
        }
    }

    fn attack(self) -> u8 {
        match self {
            Unit::Infantry => 1,
            Unit::Artillery => 2,
            Unit::Tank => 3,
            Unit::AntiAir => 0,
            Unit::BombardingCruiser => 3,
            Unit::BombardingBattleship => 4,
            Unit::Fighter => 3,
            Unit::Bomber => 4,
            Unit::Submarine => 2,
            Unit::Destroyer => 2,
            Unit::Cruiser => 3,
            Unit::Carrier => 1,
            Unit::Battleship => 4,
            Unit::BattleshipDamaged => 4,
        }
    }

    fn defense(self) -> u8 {
        match self {
            Unit::Infantry => 2,
            Unit::Artillery => 2,
            Unit::Tank => 3,
            Unit::AntiAir => 1,
            Unit::BombardingCruiser => 0,
            Unit::BombardingBattleship => 0,
            Unit::Fighter => 4,
            Unit::Bomber => 1,
            Unit::Submarine => 1,
            Unit::Destroyer => 2,
            Unit::Cruiser => 3,
            Unit::Carrier => 2,
            Unit::Battleship => 4,
            Unit::BattleshipDamaged => 4,
        }
    }
}

impl core::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Unit::Infantry => "Infantry",
                Unit::Artillery => "Artillery",
                Unit::Tank => "Tank",
                Unit::AntiAir => "Anti-Air",
                Unit::BombardingCruiser => "Bombarding Cruiser",
                Unit::BombardingBattleship => "Bombarding Battleship",
                Unit::Fighter => "Fighter",
                Unit::Bomber => "Bomber",
                Unit::Submarine => "Submarine",
                Unit::Destroyer => "Destroyer",
                Unit::Cruiser => "Cruiser",
                Unit::Carrier => "Carrier",
                Unit::Battleship => "Battleship",
                Unit::BattleshipDamaged => "Battleship (Damaged)",
            }
        )
    }
}
