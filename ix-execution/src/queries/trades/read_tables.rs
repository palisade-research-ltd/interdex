pub async fn q_read_trades(
    p_exchange: String,
    p_symbol: String,
    p_limit: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = format!(
        r#"SELECT 
            timestamp, 
            symbol, 
            side, 
            amount, 
            price,
            exchange
        FROM operations.publictrades 
        WHERE exchange = '{}' AND symbol = '{}' 
        ORDER BY timestamp DESC
        LIMIT {}"#,
        p_exchange, p_symbol, p_limit
    );
    Ok(query)
}
