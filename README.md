# gsi-cs2-rs
Ready-to-use structures for serializing data from Counter Strike 2 GSI

# Info
Documentation can be found [here](https://docs.rs/gsi-cs2 "here").

Examples can be found [here](https://github.com/sam-ai56/gsi-cs2-rs/tree/main/examples "here").

[GSI wiki](https://developer.valvesoftware.com/wiki/Counter-Strike:_Global_Offensive_Game_State_Integration "GSI wiki") and [guide](https://www.reddit.com/r/GlobalOffensive/comments/cjhcpy/game_state_integration_a_very_large_and_indepth/ "guide"), it's a bit outdated, but "this is fine".

# Examples
All examples in the repository are using the poem framework server.
But you can use any other alternatives, for example, axum :)

The first thing you will probably want to do is enable GSI in Counter Strike 2. There are two files in the [gsi_cfg](https://github.com/sam-ai56/gsi-cs2-rs/tree/main/gsi_cfg "gsi_cfg") folder, the first of which is prefixed with "[fast](https://github.com/sam-ai56/gsi-cs2-rs/blob/main/gsi_cfg/gamestate_integration_fast.cfg "fast")", use it if you want to receive data very quickly on your local machine, the other file "[normal](https://github.com/sam-ai56/gsi-cs2-rs/blob/main/gsi_cfg/gamestate_integration_normal.cfg "normal")" sends data with a delay, it can be used for transfer over the network. Copy one of these files to `(cs2)/game/csgo/cfg`. And that's it, restart the game.

In order to run examples, you need to add this to Cargo.toml

```toml
[dependencies]
gsi-cs2 = "0.1.0"
poem = { version = "1.3.48", features = ["server"] }
tokio = { version = "1.21.2", features = ["rt-multi-thread", "macros"] }
tracing-subscriber = { version = "0.3.16" }
```

---

In this example we can see serialized data from cs2

>[example/payload.rs](https://github.com/sam-ai56/gsi-cs2-rs/blob/main/examples/payload.rs "example/payload.rs") | *cargo run --example payload*

```rust
use poem::{
    handler, listener::TcpListener, post,
    Route, Server, web::Json
};
use gsi_cs2::Body;

#[handler]
fn update(data: Json<Body>) {
    println!("{:#?}", data);
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let app = Route::new().at("/", post(update));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
```

---

This example shows how to get the state of your active weapon.

>[example/weapons.rs](https://github.com/sam-ai56/gsi-cs2-rs/blob/main/examples/weapons.rs "example/weapons.rs") | *cargo run --example weapons*

```rust
use poem::{
    handler, listener::TcpListener, post,
    Route, Server, web::Json
};
use gsi_cs2::{
    Body, payload::weapon::{
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
```

---

*Sometimes, cs2 may not send data to the server for about 30 seconds.*
