
#[derive(Debug, Clone)]
pub enum Clients {
    Rest,
    Wss,
}

pub mod http_client;
pub use http_client::*;
