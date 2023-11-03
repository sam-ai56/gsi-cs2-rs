use serde::{
    Serialize, Deserialize
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum TeamClass {
    CT,
    T,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TeamInfo {
    /// Team name.
    pub name: Option<String>,
    /// Team flag code.
    pub flag: Option<String>,
    /// Team score.
    pub score: u8,
    /// The number of consecutive rounds lost.
    pub consecutive_round_losses: u8,
    /// The number of timeouts remaining for this team.
    pub timeouts_remaining: u8,
    /// Number of matches won in the series.
    pub matches_won_this_series: u8
}