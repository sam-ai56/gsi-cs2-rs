use serde::{
    Serialize, Deserialize
};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Weapon {
    /// Weapon name. Refer to [WeaponName].
    pub name: WeaponName,
    /// Skin name.
    pub paintkit: String,
    /// Weapon type. Refer to [WeaponType].
    pub r#type: Option<WeaponType>,
    /// Weapon state. Refer to [WeaponState].
    pub state: WeaponState,
    /// Ammo clip.
    #[serde(default)]
    pub ammo_clip: u16,
    /// Maximum amount of ammo in the clip.
    #[serde(default)]
    pub ammo_clip_max: u16,
    /// Amount of reserve ammo.
    #[serde(default)]
    pub ammo_reserve: u16
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum WeaponName {
    /// AK-47
    #[serde(rename = "weapon_ak47")]
    AK47,
    /// AUG
    #[serde(rename = "weapon_aug")]
    AUG,
    /// AWP
    #[serde(rename = "weapon_awp")]
    AWP,
    /// AXE
    #[serde(rename = "weapon_axe")]
    AXE,
    /// PP-Bizon
    #[serde(rename = "weapon_bizon")]
    Bizon,
    /// Bump Mine
    #[serde(rename = "weapon_bumpmine")]
    BumpMine,
    /// Breach Charge
    #[serde(rename = "weapon_breachcharge")]
    BreachCharge,
    /// C4 Explosive
    #[serde(rename = "weapon_c4")]
    C4,
    /// CZ-75 Auto
    #[serde(rename = "weapon_cz75a")]
    CZ75A,
    /// Desert Eagle
    #[serde(rename = "weapon_deagle")]
    DesertEagle,
    /// Decoy Grenade
    #[serde(rename = "weapon_decoy")]
    DecoyGrenade,
    /// Diversion Device
    #[serde(rename = "weapon_diversion")]
    DiversionDevice,
    /// Dual Berettas
    #[serde(rename = "weapon_elite")]
    DualBerettas,
    /// FAMAS
    #[serde(rename = "weapon_famas")]
    FAMAS,
    /// Fire Grenade
    #[serde(rename = "weapon_fire_grenade")]
    FireGrenade,
    /// Fire Bomb
    #[serde(rename = "weapon_firebomb")]
    Firebomb,
    /// Fists
    #[serde(rename = "weapon_fists")]
    Fists,
    /// Five-SeveN
    #[serde(rename = "weapon_fiveseven")]
    FiveSeven,
    /// Flashbang Grenade
    #[serde(rename = "weapon_flashbang")]
    FlashbangGrenade,
    /// Frag Grenade
    #[serde(rename = "weapon_frag_grenade")]
    FragGrenade,
    /// G3SG1
    #[serde(rename = "weapon_g3sg1")]
    G3SG1,
    /// Galil AR
    #[serde(rename = "weapon_galilar")]
    Galilar,
    /// Glock
    #[serde(rename = "weapon_glock")]
    Glock,
    /// Medi-Shot (Healthshot)
    #[serde(rename = "weapon_healthshot")]
    MediShot,
    /// Hammer
    #[serde(rename = "weapon_hammer")]
    Hammer,
    /// HE Grenade
    #[serde(rename = "weapon_hegrenade")]
    HEGrenade,
    /// P2000
    #[serde(rename = "weapon_hkp2000")]
    P2000,
    /// Incendiary Grenade
    #[serde(rename = "weapon_incgrenade")]
    IncendiaryGrenade,
    /// Counter-Terrorist Knife
    #[serde(rename = "weapon_knife")]
    KnifeCT,
    /// Terrorist Knife
    #[serde(rename = "weapon_knife_t")]
    KnifeT,
    /// Bayonet Knife
    #[serde(rename = "weapon_bayonet")]
    KnifeBayonet,
    /// Bowie Knife
    #[serde(rename = "weapon_knife_survival_bowie")]
    KnifeBowie,
    /// Butterfly Knife
    #[serde(rename = "weapon_knife_butterfly")]
    KnifeButterfly,
    /// Classic Knife
    #[serde(rename = "weapon_knife_css")]
    KnifeClassic,
    /// Falchion Knife
    #[serde(rename = "weapon_knife_falchion")]
    KnifeFalchion,
    /// Flip Knife
    #[serde(rename = "weapon_knife_flip")]
    KnifeFlip,
    /// Gut Knife
    #[serde(rename = "weapon_knife_gut")]
    KnifeGut,
    /// Huntsman Knife
    #[serde(rename = "weapon_knife_tactical")]
    KnifeHuntsman,
    /// Karambit Knife
    #[serde(rename = "weapon_knife_karambit")]
    KnifeKarambit,
    /// Kukri Knife
    #[serde(rename = "weapon_knife_kukri")]
    KnifeKukri,
    /// M9_Bayonet Knife
    #[serde(rename = "weapon_knife_m9_bayonet")]
    KnifeM9Bayonet,
    /// Navaja Knife
    #[serde(rename = "weapon_knife_gypsy_jackknife")]
    KnifeNavaja,
    /// Nomad Knife
    #[serde(rename = "weapon_knife_outdoor")]
    KnifeNomad,
    /// Paracord Knife
    #[serde(rename = "weapon_knife_cord")]
    KnifeParacord,
    /// Shadow Daggers
    #[serde(rename = "weapon_knife_push")]
    KnifeShadowDaggers,
    /// Skeleton Knife
    #[serde(rename = "weapon_knife_skeleton")]
    KnifeSkeleton,
    /// Stiletto Knife
    #[serde(rename = "weapon_knife_stiletto")]
    KnifeStiletto,
    /// Survival Knife
    #[serde(rename = "weapon_knife_canis")]
    KnifeSurvival,
    /// Talon Knife
    #[serde(rename = "weapon_knife_talon")]
    KnifeTalon,
    /// Ursus Knife
    #[serde(rename = "weapon_knife_ursus")]
    KnifeUrsus,
    /// M249
    #[serde(rename = "weapon_m249")]
    M249,
    /// M4A4
    #[serde(rename = "weapon_m4a1")]
    M4A4,
    /// M4A1-S
    #[serde(rename = "weapon_m4a1_silencer")]
    M4A1S,
    /// MAC-10
    #[serde(rename = "weapon_mac10")]
    MAC10,
    /// MAG-7
    #[serde(rename = "weapon_mag7")]
    MAG7,
    /// Molotov (grenade)
    #[serde(rename = "weapon_molotov")]
    Molotov,
    /// MP5-SD
    #[serde(rename = "weapon_mp5sd")]
    MP5SD,
    /// MP7
    #[serde(rename = "weapon_mp7")]
    MP7,
    /// MP9
    #[serde(rename = "weapon_mp9")]
    MP9,
    /// Negev
    #[serde(rename = "weapon_negev")]
    Negev,
    /// Nova
    #[serde(rename = "weapon_nova")]
    Nova,
    /// P250
    #[serde(rename = "weapon_p250")]
    P250,
    /// P90
    #[serde(rename = "weapon_p90")]
    P90,
    /// Revolver
    #[serde(rename = "weapon_revolver")]
    Revolver,
    /// Sawedoff
    #[serde(rename = "weapon_sawedoff")]
    SawedOff,
    /// SCAR-20
    #[serde(rename = "weapon_scar20")]
    SCAR20,
    /// SG556
    #[serde(rename = "weapon_sg556")]
    SG556,
    /// Spanner
    #[serde(rename = "weapon_spanner")]
    Spanner,
    /// Riot Shield
    #[serde(rename = "weapon_shield")]
    Shield,
    /// Smoke Grenade
    #[serde(rename = "weapon_smokegrenade")]
    SmokeGrenade,
    /// Snowball
    #[serde(rename = "weapon_snowball")]
    Snowball,
    /// SSG 08
    #[serde(rename = "weapon_ssg08")]
    SSG08,
    /// Tablet
    #[serde(rename = "weapon_tablet")]
    Tablet,
    /// Tactical Awareness Grenade
    #[serde(rename = "weapon_tagrenade")]
    TAGrenade,
    /// Zeus x27 (taser)
    #[serde(rename = "weapon_taser")]
    Zeus27,
    /// ????
    #[serde(rename = "weapon_tripwirefire")]
    Tripwirefire,
    /// TEC-9
    #[serde(rename = "weapon_tec9")]
    TEC9,
    /// UMP-45
    #[serde(rename = "weapon_ump45")]
    UMP45,
    /// USP-S
    #[serde(rename = "weapon_usp_silencer")]
    USPS,
    /// XM1014
    #[serde(rename = "weapon_xm1014")]
    XM1014,
    /// Repulsor Device
    #[serde(rename = "weapon_zone_repulsor")]
    RepulsorDevice
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
