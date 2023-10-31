use poem::{handler, listener::TcpListener, post, Route, Server, web::Json};
use gsi_csgo::Body;

// idk just dumb example of authentication
#[handler]
async fn update(data: Json<Body>) {
    let token = "CCWJu64ZV3JHDT8hZc";

    if data.auth.get("token").unwrap() != token {
        println!("Authentication failed");
        return;
    }

    println!("Authentication Passed");
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let app = Route::new().at("/", post(update));
    
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}