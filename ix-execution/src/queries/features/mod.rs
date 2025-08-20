pub mod read_tables;
pub mod write_tables;
pub mod create_tables;

pub struct FeatureNew {
    feature_ts: u64,
    symbol: String,
    exchange: String,
    spread: String,
    midprice: String,
    w_midprice: String,
    vwap: String,
    imb: String,
    tav: String,
}


