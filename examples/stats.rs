use poem::{
    handler, listener::TcpListener, post,
    Route, Server, web::Json
};
use gsi_cs2::Body;

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
    let state = player.state.as_ref().unwrap();

    print!("\x1B[2J\x1B[1;1H"); //clear

    println!("HP: {}\nArmor: {}\nHelmet: {}", state.health, state.armor, state.helmet);
    println!("Defuse Kit: {}\nMoney: {}\nKills: {}", state.defuse_kit, state.money, state.round_kills);
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let app = Route::new().at("/", post(update));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}