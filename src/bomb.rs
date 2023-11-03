use serde::{
    Serialize, Deserialize
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Bomb {
    /// Bomb state.d
    pub state: String,
    /// Bomb position.
    pub position: String,
    /// SteamID64 of the player.
    pub player: Option<String>
}