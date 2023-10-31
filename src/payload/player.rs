use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Player {
    #[serde(rename = "steamid")]
    pub steam_id: Option<String>,
    /// Clan name
    pub clan: Option<String>,
    pub name: String,
    pub observer_slot: Option<u8>,
    pub team: Option<super::TeamClass>,
    /// In game activity
    pub activity: Option<Activity>,
    pub match_stats: Option<MatchStats>,
    pub state: Option<State>,
    #[serde(default)]
    pub weapons: HashMap<String, super::Weapon>,
    /// Id of the player the spectator is watching
    pub spectarget: Option<String>,
    /// Player's position in the world
    pub position: Option<String>,
    /// Player's camera rotation vector
    pub forward: Option<String>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Activity {
    Menu,
    Playing,
    TextInput
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MatchStats {
    pub kills: i64,
    pub assists: u64,
    pub deaths: u64,
    pub mvps: u64,
    pub score: u64
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub health: u64,
    pub armor: u64,
    pub helmet: bool,
    pub flashed: u16,
    pub smoked: Option<u64>,
    pub burning: u16,
    pub money: u16,
    pub round_kills: i64,
    /// Round headshots kills
    pub round_killhs: u64,
    pub round_totaldmg: Option<u64>,
    pub equip_value: u64,
    #[serde(rename = "defusekit")]
    pub defuse_kit: Option<bool>
}