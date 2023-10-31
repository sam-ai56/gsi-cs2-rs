use poem::{handler, listener::TcpListener, post, Route, Server, web::Json};
use gsi_csgo::{Body, payload::weapon::{WeaponState, WeaponType::*}};

#[handler]
async fn update(data: Json<Body>) {
    let map = data.map.as_ref();

    if let None = map {
        println!("You need to load map");
        return;
    }

    let player = data.player.as_ref().unwrap();
    let weapons = &player.weapons;

    print!("\x1B[2J\x1B[1;1H"); //clear
    // Cheking player weapons for active state
    for (_k, weapon) in weapons.iter() {

        // The weapon that is currently active 
        if let WeaponState::Active = weapon.state{

            // Check if the weapon type has ammo clip
            // The taser has no type, but has an ammo clip.
            match weapon.r#type {
                Some(Knife)         => println!("Knife"),
                Some(Melee)         => println!("Melee"),
                Some(Fists)         => println!("Fists"),
                Some(C4)            => println!("C4"),
                Some(Grenade)       => println!("Grenade"),
                Some(Tablet)        => println!("Tablet"),
                Some(StackableItem) => println!("StackableItem"),
                _ => {
                    if let None = weapon.ammo_clip{
                        // The shield has no type and no clip.
                        if weapon.name == "weapon_shield" {
                            println!("Shield")
                        }
                        return;
                    }
                    // Weapon have ammo clip
                    println!("{} ({}/{})", weapon.name, weapon.ammo_clip.unwrap(), weapon.ammo_reserve.unwrap())
                }
            }
        }

        if let WeaponState::Reloading = weapon.state{
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