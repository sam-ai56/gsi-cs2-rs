use std::collections::HashMap;

use serde::{Serialize, Deserialize};

/// Map information
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Map {
    #[serde(default)]
    pub round_wins: HashMap<u8, String>,
    pub mode: Mode,
    pub name: String,
    pub phase: GeneralPhase,
    /// Current round
    pub round: u8,
    /// Counter Terrorists team info
    pub team_ct: super::TeamInfo,
    /// Terrorists team info
    pub team_t: super::TeamInfo,
    pub num_matches_to_win_series: u8,
    pub current_spectators: u8,
    pub souvenirs_total: u8
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    #[serde(rename = "gungameprogressive")]
    ArmsRace,
    Competitive,
    Casual,
    Custom,
    Deathmatch,
    #[serde(rename = "gungametrbomb")]
    Demolition,
    Survival,
    Training,
    #[serde(rename = "scrimcomp2v2")]
    Wingman
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GeneralPhase {
    Warmup,
    Live,
    Intermission,
    GameOver
}