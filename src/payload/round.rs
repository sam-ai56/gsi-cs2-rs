use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Round {
    pub phase: RoundPhase,
    pub bomb: Option<BombState>,
    pub win_team: Option<super::TeamClass>
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