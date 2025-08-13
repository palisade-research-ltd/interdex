#[cfg(test)]
mod tests {

    use chrono::Utc;
    use ix_cex::models::orderbook::{
        Orderbook, OrderbookInput, PriceLevel, PriceLevelInput,
    };

    #[test]
    fn test_price_level_creation() {
        let level = PriceLevel::new(151.52, 95.568);
        assert_eq!(level.price, 151.52);
        assert_eq!(level.quantity, 95.568);
        assert_eq!(level.price, 151.52);
        assert_eq!(level.quantity, 95.568);
    }

    #[test]
    fn test_orderbook_creation() {
        let bids = vec![
            PriceLevel::new(151.52, 95.568),
            PriceLevel::new(151.51, 85.869),
        ];
        let asks = vec![
            PriceLevel::new(151.53, 131.551),
            PriceLevel::new(151.54, 89.835),
        ];

        let orderbook = Orderbook::new(
            "SOLUSDC".to_string(),
            "Binance".to_string(),
            Utc::now(),
            bids,
            asks,
            Some(3939500473),
            None,
        );

        assert_eq!(orderbook.symbol, "SOLUSDC");
        assert_eq!(orderbook.exchange, "Binance");
        assert_eq!(orderbook.best_bid().unwrap().price, 151.52);
        assert_eq!(orderbook.best_ask().unwrap().price, 151.53);
    }

    #[test]
    fn test_orderbook_calculations() {
        let bids = vec![PriceLevel::new(100.0, 10.0), PriceLevel::new(99.0, 5.0)];
        let asks = vec![PriceLevel::new(101.0, 8.0), PriceLevel::new(102.0, 12.0)];

        let orderbook = Orderbook::new(
            "TEST".to_string(),
            "Exchange".to_string(),
            Utc::now(),
            bids,
            asks,
            Some(12345),
            None,
        );

        assert_eq!(orderbook.mid_price().unwrap(), 100.5);
        assert_eq!(orderbook.spread().unwrap(), 1.0);
        assert_eq!(orderbook.bid_volume(), 15.0);
        assert_eq!(orderbook.ask_volume(), 20.0);
    }

    #[test]
    fn test_orderbook_validation() {
        let bids = vec![
            PriceLevel::new(100.0, 10.0),
            PriceLevel::new(99.0, 5.0), // Correct descending order
        ];
        let asks = vec![
            PriceLevel::new(101.0, 8.0),
            PriceLevel::new(102.0, 12.0), // Correct ascending order
        ];

        let orderbook = Orderbook::new(
            "TEST".to_string(),
            "Exchange".to_string(),
            Utc::now(),
            bids,
            asks,
            Some(12345),
            None,
        );

        assert!(orderbook.validate().is_ok());
    }

    #[test]
    fn test_json_input_conversion() {
        let json_input = OrderbookInput {
            symbol: "SOLUSDC".to_string(),
            exchange: "Binance".to_string(),
            timestamp: "2025-07-08T20:05:02.722776Z".to_string(),
            bids: vec![PriceLevelInput {
                price: "151.52".to_string(),
                quantity: "95.568".to_string(),
            }],
            asks: vec![PriceLevelInput {
                price: "151.53".to_string(),
                quantity: "131.551".to_string(),
            }],
            last_update_id: 3939500473,
            sequence: None,
        };

        let orderbook: Orderbook = json_input.try_into().unwrap();

        assert_eq!(orderbook.symbol, "SOLUSDC");
        assert_eq!(orderbook.exchange, "Binance");
        assert_eq!(orderbook.best_bid().unwrap().price, 151.52);
        assert_eq!(orderbook.best_ask().unwrap().price, 151.53);
    }
}
