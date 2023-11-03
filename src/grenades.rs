use std::collections::HashMap;

use serde::{
    Serialize, Deserialize
};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Grenades {
    /// SteamID64 of the player.
    pub owner: String,
    /// Grenade position in the world.
    pub position: Option<String>,
    /// Grenade projectile velocity.
    pub velocity: Option<String>,
    /// Grenade projectile lifetime.
    pub lifetime: String,
    /// Type of the grenade. Refer to [GrenadeType].
    pub r#type: GrenadeType,
    /// Grenade flames particle position.
    #[serde(default)]
    pub flames: HashMap<String, String>,
    /// Grenade projectile effecttime.
    pub effecttime: Option<String>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GrenadeType {
    Smoke,
    Inferno,
    Firebomb,
    Frag,
    Flashbang,
    Decoy
}