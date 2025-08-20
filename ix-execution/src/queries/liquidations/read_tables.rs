pub async fn read_liquidations_table(
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
        FROM liquidations 
        WHERE exchange = '{}' AND symbol = '{}' 
        LIMIT 10
        "#,
        p_exchange, p_symbol
    );
    Ok(query)
}

