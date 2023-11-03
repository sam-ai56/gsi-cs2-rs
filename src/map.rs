use std::collections::HashMap;

use serde::{
    Serialize, Deserialize
};

use super::team::TeamInfo;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Map {
    /// Round [number, condition].
    #[serde(default)]
    pub round_wins: HashMap<u8, String>,
    /// Game Mode. Refer to [Mode].
    pub mode: Mode,
    /// Map's title.
    pub name: String,
    /// Present stage of the game. Refer to [RoundPhase].
    pub phase: RoundPhase,
    /// Current round count.
    pub round: u8,
    /// Details regarding the Counter-Terrorists team. Refer to [TeamInfo].
    pub team_ct: TeamInfo,
    /// Details regarding the Terrorists team. Refer to [TeamInfo].
    pub team_t: TeamInfo,
    /// Count of victories required to secure the series.
    pub num_matches_to_win_series: u8,
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
pub enum RoundPhase {
    Warmup,
    Live,
    Intermission,
    GameOver
}