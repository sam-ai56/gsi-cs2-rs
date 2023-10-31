use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum TeamClass {
    CT,
    T,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TeamInfo {
    pub name: Option<String>,
    /// Flag code
    pub flag: Option<String>,
    pub score: u8,
    pub consecutive_round_losses: u8,
    pub timeouts_remaining: u8,
    pub matches_won_this_series: u8
}