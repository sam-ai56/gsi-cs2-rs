use std::collections::HashMap;

use serde::{Serialize, Deserialize, de::IgnoredAny};

pub mod map;
use map::Map;

pub mod player;
use player::Player;

pub mod round;
use round::Round;

pub mod weapon;
use weapon::Weapon;

pub mod grenades;
use grenades::Grenades;

pub mod team;
use team::{TeamInfo, TeamClass};

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Body {
    pub map: Option<Map>,
    pub player: Option<Player>,
    pub provider: Option<Provider>,
    pub round: Option<Round>,
    #[serde(default)]
    pub grenades: HashMap<String, Grenades>,
    #[serde(default)]
    pub allplayers: HashMap<String, Player>,
    pub bomb: Option<Bomb>,
    pub phase_countdowns: Option<PhaseCountdowns>,
    #[serde(skip_serializing, default)]
    previously: IgnoredAny,
    #[serde(skip_serializing, default)]
    added: IgnoredAny,
    pub auth: HashMap<String, String>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Provider {
    pub name: String,
    #[serde(rename = "appid")]
    pub app_id: u16,
    pub version: u64,
    #[serde(rename = "steamid")]
    pub steam_id: String,
    pub timestamp: u64

}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Bomb {
    pub state: String,
    pub position: String,
    pub player: Option<u64>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PhaseCountdowns {
    pub phase: String,
    pub phase_countdown: Option<String>,
    pub phase_ends_in: Option<String>
}