//! Ready-to-use structures for serializing data from Counter Strike 2 GSI
use std::collections::HashMap;
use serde::{
    Serialize, Deserialize, de::IgnoredAny
};

pub mod map;
pub mod player;
pub mod provider;
pub mod round;
pub mod weapon;
pub mod grenades;
pub mod bomb;
pub mod phase_countdowns;
pub mod team;

use map::Map;
use player::Player;
use provider::Provider;
use round::Round;
use grenades::Grenades;
use bomb::Bomb;
use phase_countdowns::PhaseCountdowns;

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Body {
    /// Current map - the game, information about what is currently happening on the map, for example, objects, information about round... Refer to [Map].
    pub map: Option<Map>,
    /// Information about the player, his status, weapons, in-game activity, stats... Refer to [Player].
    pub player: Option<Player>,
    /// GSI provider information, version, timestamp of the last gsi update. Refer to [Provider].
    pub provider: Option<Provider>,
    /// Information about round, round state, bomb state info, win team. Refer to [Round].
    pub round: Option<Round>,
    /// Information about grenades projectiles. Refer to [Grenades].
    #[serde(default)]
    pub grenades: HashMap<String, Grenades>,
    /// Information about all players in the game. Refer to [Player].
    #[serde(default)]
    pub allplayers: HashMap<String, Player>,
    /// Information about bomb, state, positon... Refer to [Bomb].
    pub bomb: Option<Bomb>,
    /// Phase countdowns, when round is ending, and what phase. Refer to [PhaseCountdowns].
    pub phase_countdowns: Option<PhaseCountdowns>,
    /// To much data to serialize, disabled. Mostly not needed information.
    #[serde(skip_serializing, default)]
    previously: IgnoredAny,
    /// To much data to serialize, disabled. Mostly not needed information.
    #[serde(skip_serializing, default)]
    added: IgnoredAny,
    /// Authentication field here can be found token that is written inside gamestate config.
    pub auth: HashMap<String, String>
}