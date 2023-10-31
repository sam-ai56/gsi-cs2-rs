use poem::{handler, listener::TcpListener, post, Route, Server, web::Json};
use gsi_csgo::Body;

async fn name(data: Json<Body>){
    let player = data.player.as_ref().unwrap();
    println!("Name: {}", player.name);
}

async fn health(data: Json<Body>){
    let player = data.player.as_ref().unwrap();
    let state = player.state.as_ref().unwrap();
    println!("Health: {}", state.health)
}

fn armor(data: Json<Body>){
    let player = data.player.as_ref().unwrap();
    let state = player.state.as_ref().unwrap();
    println!("Armor: {}", state.armor)
}

#[handler]
async fn update(data: Json<Body>) {
    let map = data.map.as_ref();

    if let None = map {
        println!("You need to load map");
        return;
    }

    name(data.clone()).await;
    health(data.clone()).await;
    armor(data.clone());
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    tracing_subscriber::fmt::init();

    let app = Route::new().at("/", post(update));
    
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}