// Read from the orderbooks table DDL
pub fn read_orderbooks_table() -> String {
    r#"
        SELECT 
            * FROM orderbooks =
        WHERE
            exchange = 'binance',
        AND
            symbol = 'btcusdc'
        LIMIT 10 
    "#
    .trim()
    .to_string()
}
