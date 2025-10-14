use crate::unit::Unit;

pub type PlayerID = u8;
pub type ProvinceID = u8;

pub enum ProvinceType {
    // These are the only 3 province types in standard Diplomacy.
    // Note that "Coast" specifically means locations like Spain North Coast, and Spain south Coast (and not Brest).
    Land = 0,
    Coast = 1,
    Water = 2,

    // Variant Diplomacy province types go here:
    DeepSea = 3,
}

impl ProvinceType {
    // Whether or not a unit can convoy through this space (i.e. can a unit legally order "Convoy X -> Y" here).
    pub fn can_convoy_through(&self) -> bool {
        match self {
            Self::Land => false,
            Self::Coast => false,
            Self::Water => true,
            Self::DeepSea => true,
        }
    }

    // Whether or not a unit can be convoyed into this space. (i.e. can a "Convoy X -> Y" order be legal, if this is Y).
    pub fn can_convoy_into(&self) -> bool {
        match self {
            Self::Land => true,
            Self::Coast => true,
            Self::Water => false,
            Self::DeepSea => false,
        }
    }

    // Whether or not a unit can be convoyed out of this space. (i.e. can a "Convoy X -> Y" order be legal, if this is X).
    pub fn can_convoy_out_of(&self) -> bool {
        match self {
            Self::Land => true,
            Self::Coast => true,
            Self::Water => false,
            Self::DeepSea => false,
        }
    }
}

/// Struct representing an individual province on the game board
pub struct Province {
    // Internal ID of the province
    province_id: ProvinceID,

    // Visible name of the province (i.e. "NTH" or "Par")
    province_name: String,

    // Type of the province (i.e. land/water/coast)
    province_type: ProvinceType,

    // Owner of the province. Note that neutral/unowned is a value of `0`.
    owned_by: PlayerID,

    // None if not a Supply Center, Some(1) if an SC, and Some(0) if it is the coast of an SC.
    // The distinction between None and Some(0) is that it may be possible to build in a Some(0),
    // but it still doesn't count towards the player's SC count.
    sc_value: Option<u8>,

    // List of players that can build in this province. Is at most one in standard diplomacy, up to all countries in "build anywhere" variants.
    core_of: Vec<PlayerID>,

    // List of all provinces that are coasts of this province. Should always have a 1:1 match with another Province's "is_coast_of"
    has_coasts: Vec<ProvinceID>,

    // Some(Province) if this is the coast of another province; None otherwise. This should never be recursive or cyclical.
    is_coast_of: Option<ProvinceID>,

    // Some(Unit) if there is a unit in this province; None otherwise.
    occupied_by: Option<Unit>,

    // Some(Unit) if there is a unit that was dislodged from this province, before it retreats. Should only be Some() during a retreat phase.
    disloged_unit: Option<Unit>,

    // Whether or not this province can be retreated to. Only matters during retreat phase.
    available_for_retreat: bool,
}

impl Province {
    pub fn new() {}
}
