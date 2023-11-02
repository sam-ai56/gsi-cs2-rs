use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Grenades {
    pub owner: String,
    pub position: Option<String>,
    pub velocity: Option<String>,
    pub lifetime: String,
    pub r#type: GrenadeType,
    #[serde(default)]
    pub flames: HashMap<String, String>,
    pub effecttime: Option<String>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GrenadeType {
    Smoke,
    Inferno,
    Firebomb,
    Frag,
    Flashbang,
    Decoy
}