pub async fn read_trades_table(
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
        FROM trades 
        WHERE exchange = '{}' AND symbol = '{}' 
        AND isValidUTF8(symbol) = 1 
        AND isValidUTF8(exchange) = 1
        LIMIT 10"#,
        p_exchange, p_symbol
    );
    Ok(query)
}
