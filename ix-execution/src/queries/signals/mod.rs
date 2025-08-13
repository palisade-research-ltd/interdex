pub mod create_tables;
pub mod write_tables;

#[derive(Debug, Clone)]
pub struct Signal {
    pub ts: u64,
    pub symbol: String,
    pub side: String,
    pub exchange: String,
}
