use std::collections::HashMap;

use serde::{
    Serialize, Deserialize
};

use super::team::TeamClass;
use super::weapon::Weapon;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Player {
    /// SteamID64 of the player.
    #[serde(rename = "steamid")]
    pub steam_id: Option<String>,

    /// Clan name
    pub clan: Option<String>,

    /// Username of the player.
    pub name: Option<String>,
    /// Player's assigned observer slot number.
    pub observer_slot: Option<u8>,
    /// Player's team. Refer to [TeamClass].
    pub team: Option<TeamClass>,
    /// In game activity. Refer to [Activity].
    pub activity: Option<Activity>,
    /// Stats of the player. Refer to [MatchStats].
    pub match_stats: Option<MatchStats>,
    /// Information about the player's state. Refer to [State].
    pub state: Option<State>,
    /// Player carried weapons. Refer to [Weapon].
    #[serde(default)]
    pub weapons: HashMap<String, Weapon>,
    /// SteamID64 of the player.
    pub spectarget: Option<String>,
    /// Player's position in the world.
    pub position: Option<String>,
    /// Player's forward movement.
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
    /// Total amount of player kills.
    pub kills: u16,
    /// Total amount of player's assists.
    pub assists: u16,
    /// Total amount of player deaths.
    pub deaths: u16,
    /// Total amount of player MVP.
    pub mvps: u8,
    /// Player score.
    pub score: u16
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct State {
    /// Player health.
    pub health: u8,
    /// Player armor.
    pub armor: u8,
    /// Is the player has a helmet.
    pub helmet: bool,
    /// How much the player is flashed.
    pub flashed: u8,
    /// How much the player is smoked.
    pub smoked: u8,
    /// How much the player is burning.
    pub burning: u8,
    /// Where do you get the money from?
    pub money: u16,
    /// The number of kills a player has made in the current round.
    pub round_kills: u16,
    /// The number of kills a player has made with headshots in the current round.
    pub round_killhs: u64,
    /// The total amount of damage dealt by the player in the current round.
    #[serde(default)]
    pub round_totaldmg: u32,
    /// Overall cost of the player's equipment.
    pub equip_value: u16,
    /// Is the player has a defuse kit.
    #[serde(rename = "defusekit", default)]
    pub defuse_kit: bool
}