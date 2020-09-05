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
        match self {
            Unit::Fighter | Unit::Bomber => true,
            _ => false,
        }
    }

    pub fn is_bombarding(self) -> bool {
        match self {
            Unit::BombardingCruiser | Unit::BombardingBattleship => true,
            _ => false,
        }
    }

    pub fn is_submarine(self) -> bool {
        self == Unit::Submarine
    }

    pub fn is_targetable(self) -> bool {
        match self {
            Unit::BombardingCruiser | Unit::BombardingBattleship => false,
            _ => true,
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
    fn ipc(&self) -> u32 {
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
}
