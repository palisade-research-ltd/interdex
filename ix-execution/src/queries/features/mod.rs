use serde::{Deserialize, Serialize};
use clickhouse::Row;

pub mod write_tables;
pub mod create_tables;

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct FeatureData {
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

impl FeatureData {
    
    pub fn builder() -> FeatureDataBuilder {
        FeatureDataBuilder::new() 
    }

}

#[derive(Debug, Clone)]
pub struct FeatureDataBuilder {
    feature_ts: Option<u64>,
    symbol: Option<String>,
    exchange: Option<String>,
    spread: Option<String>,
    midprice: Option<String>,
    w_midprice: Option<String>,
    vwap: Option<String>,
    imb: Option<String>,
    tav: Option<String>,
}

impl Default for FeatureDataBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureDataBuilder {

    pub fn new() -> FeatureDataBuilder { 

        FeatureDataBuilder {
            feature_ts: None,
            symbol: None,
            exchange: None,
            spread: None,
            midprice: None,
            w_midprice: None,
            vwap: None,
            imb: None,
            tav: None,
        }
     
    }

    pub fn feature_ts(mut self, feature_ts: u64) -> Self {
        self.feature_ts = Some(feature_ts);
        self
    }

    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn exchange(mut self, exchange: String) -> Self {
        self.exchange = Some(exchange);
        self
    }

    pub fn spread(mut self, spread: String) -> Self {
        self.spread = Some(spread);
        self
    }

    pub fn midprice(mut self, midprice: String) -> Self {
        self.midprice = Some(midprice);
        self
    }

    pub fn w_midprice(mut self, w_midprice: String) -> Self {
        self.w_midprice = Some(w_midprice);
        self
    }

    pub fn vwap(mut self, vwap: String) -> Self {
        self.vwap = Some(vwap);
        self
    }


    pub fn imb(mut self, imb: String) -> Self {
        self.imb =Some(imb);
        self
    }

    pub fn tav(mut self, tav: String) -> Self {
        self.tav = Some(tav);
        self
    }

    pub fn build(self) -> Result<FeatureData, String> {

        let feature_ts = self.feature_ts.ok_or("Missing feature_ts")?;
        let symbol = self.symbol.ok_or("Missing symbol")?;
        let exchange = self.exchange.ok_or("Missing exchange")?;
        let spread = self.spread.ok_or("Missing spread")?;
        let midprice = self.midprice.ok_or("Mising midprice")?;
        let w_midprice = self.w_midprice.ok_or("Missing w_midprice")?;
        let vwap = self.vwap.ok_or("Missing vwap")?;
        let imb = self.imb.ok_or("Missing imb")?;
        let tav = self.tav.ok_or("Missing tav")?;

        Ok(FeatureData {
            feature_ts,
            symbol,
            exchange,
            spread, 
            midprice,
            w_midprice,
            vwap,
            imb,
            tav
        })

    }

}

