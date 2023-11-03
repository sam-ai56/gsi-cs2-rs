use poem::{
    handler, listener::TcpListener, post,
    Route, Server, web::Json
};
use gsi_cs2::{
    Body, weapon::{
        WeaponState, WeaponType::*, WeaponName
    }
};

#[handler]
fn update(data: Json<Body>) {
    let map = data.map.as_ref();

    if let None = map {
        println!("You need to load map");
        return;
    }

    let player_data = data.player.as_ref();

    if let None = player_data {
        return;
    }

    let player = player_data.unwrap();
    let weapons = &player.weapons;

    print!("\x1B[2J\x1B[1;1H"); //clear

    // A loop through all weapons
    for (_k, weapon) in weapons.iter() {

        // Check if the weapon is currently active
        if let WeaponState::Active = weapon.state {
            // There is no ammunition for these types of weapons
            match weapon.r#type {
                Some(Knife)         => println!("Knife"),
                Some(Melee)         => println!("Melee"),
                Some(Fists)         => println!("Fists"),
                Some(C4)            => println!("C4"),
                Some(Grenade)       => println!("Grenade"),
                Some(Tablet)        => println!("Tablet"),
                Some(StackableItem) => println!("StackableItem"),
                _ => {
                    let deserialized_name = serde_json::to_string(&weapon.name).unwrap();
                    let weapon_type = weapon.r#type.as_ref().unwrap();

                    println!("Name: {}\nType: {:?}\nAmmo: ( {} / {} )",
                        deserialized_name, weapon_type, weapon.ammo_clip,
                        weapon.ammo_reserve
                    );

                    if weapon.ammo_clip < weapon.ammo_clip_max / 5 {
                        println!("Low on ammo. Reload.");
                    }
                }
            }
            // Check if the name of the weapon is "weapon_glock"
            if let WeaponName::Glock = weapon.name {
                println!("\nWow, is that a Glock?");
            }

            // The same, but for "weapon_usp_silencer"
            if let WeaponName::USPS = weapon.name {
                println!("\nWow, is that a USP-S?");
            }
            return;
        }

        // Checking if the weapon is reloading
        if let WeaponState::Reloading = weapon.state {
            println!("Reloading...");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let app = Route::new().at("/", post(update));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}