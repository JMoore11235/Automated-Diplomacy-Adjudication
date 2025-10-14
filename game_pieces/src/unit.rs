use crate::player::PlayerID;

#[derive(Clone, Copy, PartialEq)]
pub enum UnitType {
    // Base Game types
    Army,
    Fleet,
}

pub struct Unit {
    owner: PlayerID,

    unit_type: UnitType,
}

impl Unit {
    pub fn new(owner: PlayerID, unit_type: UnitType) -> Self {
        Self { owner, unit_type }
    }

    pub fn owner(&self) -> PlayerID {
        self.owner
    }

    pub fn get_type(&self) -> UnitType {
        self.unit_type
    }

    fn can_convoy(&self) -> bool {
        match self.unit_type {
            UnitType::Army => false,
            UnitType::Fleet => true,
        }
    }

    fn can_be_convoyed(&self) -> bool {
        match self.unit_type {
            UnitType::Army => true,
            UnitType::Fleet => false,
        }
    }
}
