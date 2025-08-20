use serde::{Deserialize, Serialize};
use clickhouse::Row;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

pub mod create_tables;
pub mod read_tables;
pub mod write_tables;

// Import the atelier-base types
use atelier_base::{
    orderbooks::Orderbook,
    levels::Level,
    orders::{Order, OrderSide, OrderType},
};

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct OrderbookCH {
    pub timestamp: String,
    pub symbol: String,
    pub exchange: String,
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
}

impl OrderbookCH {
    /// Converts OrderbookCH to atelier-rs Orderbook
    pub fn to_orderbook(&self) -> Result<Orderbook, Box<dyn std::error::Error>> {
        // Parse timestamp - handle different possible formats
        let orderbook_ts = self.parse_timestamp()?;
        
        // Generate a hash-based orderbook_id from symbol and exchange
        let orderbook_id = self.generate_orderbook_id();
        
        // Convert bids (String, String) tuples to Vec<Level>
        let bids = self.convert_price_levels(&self.bids, OrderSide::Bids)?;
        
        // Convert asks (String, String) tuples to Vec<Level>  
        let asks = self.convert_price_levels(&self.asks, OrderSide::Asks)?;
        
        Ok(Orderbook::new(
            orderbook_id,
            orderbook_ts,
            self.symbol.clone(),
            bids,
            asks,
        ))
    }
    
    /// Parse timestamp string to u64 microseconds
    fn parse_timestamp(&self) -> Result<u64, Box<dyn std::error::Error>> {
        // Format 1: ISO 8601 (e.g., "2023-12-01T10:30:45.123456Z")
        if let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(&self.timestamp) {
            return Ok(parsed.timestamp_micros() as u64);
        }
        
        // Format 2: Unix timestamp as string (seconds)
        if let Ok(secs) = self.timestamp.parse::<f64>() {
            return Ok((secs * 1_000_000.0) as u64);
        }
        
        // Format 3: Unix timestamp as string (milliseconds/microseconds)
        if let Ok(millis) = self.timestamp.parse::<u64>() {
            if millis > 1_000_000_000_000 && millis < 10_000_000_000_000 {
                return Ok(millis * 1_000); // Convert milliseconds to microseconds
            }
            return Ok(millis); // Assume microseconds
        }
        
        // Fallback: use current timestamp
        Ok(SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_micros() as u64)
    }
    
    /// Generate orderbook ID from symbol and exchange using hash
    fn generate_orderbook_id(&self) -> u32 {
        let mut hasher = DefaultHasher::new();
        format!("{}:{}", self.symbol, self.exchange).hash(&mut hasher);
        (hasher.finish() & 0xFFFFFFFF) as u32
    }
    
    /// Convert price level tuples to Level structs
    fn convert_price_levels(
        &self, 
        price_levels: &[(String, String)], 
        side: OrderSide
    ) -> Result<Vec<Level>, Box<dyn std::error::Error>> {
        let mut levels = Vec::new();
        
        for (level_id, (price_str, volume_str)) in price_levels.iter().enumerate() {
            let price = price_str.parse::<f64>()?;
            let volume = volume_str.parse::<f64>()?;
            
            // Create synthetic order for this level
            let order = self.create_synthetic_order(side, price, volume)?;
            
            let level = Level::new(
                level_id as u32,
                side,
                price,
                volume,
                vec![order],
            );
            
            levels.push(level);
        }
        
        Ok(levels)
    }
    
    /// Create a synthetic order for a price level
    fn create_synthetic_order(
        &self,
        side: OrderSide, 
        price: f64, 
        amount: f64
    ) -> Result<Order, Box<dyn std::error::Error>> {
        let order_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_micros() as u64;
        
        let order = Order::builder()
            .order_ts(order_ts)
            .order_type(OrderType::Limit)
            .side(side)
            .price(price)
            .amount(amount)
            .build()
            .map_err(|e| format!("Failed to build order: {}", e))?;
            
        Ok(order)
    }
}

