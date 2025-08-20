
pub async fn read_orderbooks_table(
    p_exchange: String,
    p_symbol: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = format!(
        r#"SELECT 
            CAST(timestamp AS String) as timestamp,
            symbol,
            exchange,
            bids,
            asks
        FROM orderbooks 
        WHERE exchange = '{}' AND symbol = '{}'
        LIMIT 10
        "#,
        p_exchange, p_symbol
    );
    Ok(query)
}
