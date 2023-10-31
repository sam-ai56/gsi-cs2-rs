use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Weapon {
    pub name: String,
    pub paintkit: String,
    pub r#type: Option<WeaponType>,
    pub state: WeaponState,
    pub ammo_clip: Option<u64>,
    pub ammo_clip_max: Option<u64>,
    pub ammo_reserve: Option<u64>
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum WeaponType {
    #[serde(rename = "Submachine Gun")]
    SMG,
    #[serde(rename = "Machine Gun")]
    MachineGun,
    Rifle,
    SniperRifle,
    Shotgun,
    Pistol,
    Knife,
    Fists,
    Melee,
    Grenade,
    #[serde(rename = "Breach Charge")]
    BreachCharge,
    C4,
    #[serde(rename = "Bump Mine")]
    BumpMine,
    StackableItem,
    Tablet
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum WeaponState {
    Active,
    Holstered,
    Reloading
}