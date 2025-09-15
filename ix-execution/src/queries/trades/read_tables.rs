pub async fn q_read_trades(
    p_exchange: String,
    p_symbol: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = format!(
        r#"SELECT 
            timestamp, 
            symbol, 
            exchange, 
            side, 
            amount, 
            price,
        FROM publictrades 
        WHERE exchange = '{}' AND symbol = '{}' "#,
        p_exchange, p_symbol
    );
    Ok(query)
}
