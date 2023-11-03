use serde::{
    Serialize, Deserialize
};
use super::team::TeamClass;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Round {
    /// Round phase. Refer to [RoundPhase].
    pub phase: RoundPhase,
    /// Bomb state. Refer to [BombState].
    pub bomb: Option<BombState>,
    /// Team info that is won round. Refer to [TeamClass].
    pub win_team: Option<TeamClass>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RoundPhase {
    Live,
    FreezeTime,
    Over
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BombState {
    Planted,
    Exploded,
    Defused
}