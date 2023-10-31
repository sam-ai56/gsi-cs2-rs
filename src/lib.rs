//! A library containing ready-made structures for deserializing or reverse serializing data provided by [CSGO GSI][valvedocs] using [serde][serdecrates].
//! 
//! ## Example
//! 
//! First you need to install [`gamestate_integration_fast.cfg`](https://github.com/sam-ai56/blob/main/gsi-csgo/gsi_cfg/gamestate_integration_fast.cfg) in `csgo/cfg` folder.
//! 
//! [`examples/payload.rs`](https://github.com/sam-ai56/blob/main/gsi-csgo/examples/payload.rs)
//! ```rust
//! use poem::{handler, listener::TcpListener, post, Route, Server, web::Json};
//! use gsi_csgo::Body;
//! 
//! #[handler]
//! async fn update(data: Json<Body>) {
//!     println!("{:#?}", data);
//! }
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), std::io::Error> {
//!     tracing_subscriber::fmt::init();
//! 
//!     let app = Route::new().at("/", post(update));
//!     
//!     Server::new(TcpListener::bind("127.0.0.1:3000"))
//!         .run(app)
//!         .await
//! }
//! ```
//! 
//! Also you need add this to your `Cargo.toml`
//! ```toml
//! [dependencies]
//! gsi-csgo = "0.1.0"
//! poem = "1.3.48"
//! tokio = { version = "1.21.2", features = ["rt-multi-thread", "macros"] }
//! tracing-subscriber = "0.3.16"
//! ```
//! or just use `cargo run --example payload`
//! 
//! [`Another examples`](https://github.com/sam-ai56/gsi-csgo/tree/main/examples) can be found in the github repository.
//! 
//! [valvedocs]: https://developer.valvesoftware.com/wiki/Counter-Strike:_Global_Offensive_Game_State_Integration
//! [serdecrates]: https://crates.io/crates/serde

pub mod payload;
pub use payload::Body;