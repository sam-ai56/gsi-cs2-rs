use serde::{
    Serialize, Deserialize
};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Provider {
    /// Name of the provider. (Counter-Strike: Global Offensive)
    pub name: String,
    /// Game steam id of the provider. (730)
    #[serde(rename = "appid")]
    pub app_id: u16,
    /// Provider version.
    pub version: u64,
    /// SteamID64 of the player.
    #[serde(rename = "steamid")]
    pub steam_id: String,
    /// Timestamp of last gsi update.
    pub timestamp: u64

}