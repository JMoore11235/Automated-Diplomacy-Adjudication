use crate::unit::Unit;

pub type PlayerID = u8;
pub type ProvinceID = u8;

pub enum ProvinceType {
    // These are the only 3 province types in standard Diplomacy.
    Land = 0,
    Coast = 1,
    Water = 2,

    // Variant Diplomacy province types go here:
    DeepSea = 3,
}

/// Struct representing an individual province on the game board
pub struct Province {
    // Internal
    province_id: ProvinceID,

    // Human readable name of the province
    province_name: String,

    // Type of the province (i.e. land/water/coast)
    province_type: ProvinceType,

    // Owner of the province. Note that neutral/unowned is a value of `0`.
    owned_by: PlayerID,

    // 0 if not a Supply Center, 1 if an SC. u8 to allow for potential for high-value SCs (worth 2+ builds).
    // Note that this should always be 0 if it is a coast to prevent double counting, since the "main" province will provide the SC
    sc_value: u8,

    // List of players that can build in this province. Is at most one in standard diplomacy, up to all countries in "build anywhere" variants.
    core_of: Vec<PlayerID>,

    // Some(Province) if this is the coast of another province; None otherwise. This should never be recursive or cyclical.
    is_coast_of: Option<ProvinceID>,

    // Whether or not a unit can be commanded to convoy while in this province.
    can_convoy_through: bool,

    // Whether or not an unit can be convoyed into *or out of* this province.
    can_convoy_into: bool,

    // Some(Unit) if there is a unit in this province; None otherwise.
    occupied_by: Option<Unit>,

    // Some(Unit) if there is a unit that was dislodged from this province, before it retreats. Should only be Some() during a retreat phase.
    disloged_unit: Option<Unit>,

    // Whether or not this province can be retreated to. Only matters during retreat phase.
    available_for_retreat: bool,
}
