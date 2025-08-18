// Create the orderbooks table DDL
pub fn read_orderbooks_table() -> String {
    r#"
    SELECT *
    FROM orderbooks
    WHERE 
        exchange = {}
    AND
        symbol = {}
"#
    .trim()
    .to_string()
}

