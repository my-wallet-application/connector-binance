mod binance_client_callback;
mod binance_ws_client;
mod binance_ws_settings;
mod error;
mod event_handler;
mod models;
pub use binance_ws_client::*;
pub use error::*;
pub use event_handler::*;
pub use models::*;

use my_web_socket_client::hyper_tungstenite::tungstenite::*;
use protocol::frame::Payload;

fn create_ping_message() -> Message {
    Message::Ping(Payload::Vec(vec![]))
}
