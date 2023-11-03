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