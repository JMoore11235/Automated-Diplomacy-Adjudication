pub type PlayerID = u8;

/// This struct represents a player of the game Note that this is the 'internal' player, such as the "France", "Austria", etc. (Not the person playing the country)
pub struct Player {
    // Internal ID number of the player
    player_id: PlayerID,

    // Human readable player name (i.e. "France")
    player_name: String,
}
