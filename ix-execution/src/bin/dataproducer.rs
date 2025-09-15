use ix_execution::{queries, ClickHouseClient};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ch_url =
        env::var("CLICKHOUSE_URL").unwrap_or_else(|_| "http://database:8123".to_string());
    let ch_db = env::var("CLICKHOUSE_DB").unwrap_or_else(|_| "operations".to_string());

    println!("VPIN Computation");
    println!("================");

    // --- LIQUIDATIONS Datacollector --- //
    let ch_pt_client = ClickHouseClient::builder()
        .url(ch_url.clone())
        .database(ch_db.clone())
        .build()
        .await
        .unwrap();

    let p_exchange = "Bybit".to_string();
    let p_symbol = "SOLUSDT".to_string();

    let pt_read_query: String =
        queries::trades::read_tables::q_read_trades(p_exchange, p_symbol)
            .await
            .unwrap();
    println!("\n ---- pt_read_query {:?} ---- \n", pt_read_query);

    // let pt_read_result = ch_pt_client.read_table(&pt_read_query).await;
    // println!("\n ---- pt_read_result {:?} ---- \n", pt_read_result);

    Ok(())
}
