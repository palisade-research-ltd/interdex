// src/bin/collector.rs
use atelier_data::{exchanges::bybit::WssExchange, stream_liquidations};
use ix_execution::liquidations::LiquidationNew;

use clap::{Arg, Command};
use ix_cex::{
    exchanges::{BinanceClient, CoinbaseClient, ExchangeClient, KrakenClient},
    models::{exchanges::Exchange, orderbook::TradingPair},
};
use ix_execution::{queries, ClickHouseClient};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() {
    // --- Orderbook Data Collector --- //
    let matches = Command::new("collector")
        .about("Collects orderbook data and stores in ClickHouse")
        .arg(
            Arg::new("url")
                .long("url")
                .value_name("URL")
                .help("ClickHouse URL")
                .default_value("http://localhost:8123"),
        )
        .arg(
            Arg::new("database")
                .long("database")
                .value_name("DB")
                .help("Database name")
                .default_value("default"),
        )
        .get_matches();

    let ch_admin_client = ClickHouseClient::builder()
        .url(matches.get_one::<String>("url").unwrap())
        .database(matches.get_one::<String>("database").unwrap())
        .build()
        .await
        .unwrap();

    // Create tables if they don't exist
    let _ = ch_admin_client
        .create_table(&queries::orderbooks::create_tables::create_orderbooks_table_ddl())
        .await;

    let _ = ch_admin_client
        .create_table(
            &queries::liquidations::create_tables::create_liquidations_table_ddl(),
        )
        .await;

    let _ = ch_admin_client
        .create_table(&queries::signals::create_tables::create_signals_table_ddl())
        .await;

    // Clone client for liquidation task
    let ch_lq_client = ClickHouseClient::builder()
        .url(matches.get_one::<String>("url").unwrap())
        .database(matches.get_one::<String>("database").unwrap())
        .build()
        .await
        .unwrap();

    // --- Spawn Liquidations Task --- //
    let liquidation_task = tokio::spawn(async move {
        let symbols = vec![
            "BTCUSDT".to_string(),
            "ETHUSDT".to_string(),
            "SOLUSDT".to_string(),
        ];
        let source = WssExchange::Bybit;

        let mut liquidation_rx = match stream_liquidations(symbols, source).await {
            Ok(receiver) => receiver,
            Err(e) => {
                eprintln!("Failed to create liquidation stream: {}", e);
                return;
            }
        };

        while let Some(liquidation) = liquidation_rx.recv().await {
            // let timestamp =
            //     chrono::DateTime::from_timestamp_millis(liquidation.ts as i64)
            //         .unwrap_or_default();
            println!(
                "[{}] {} qty={} price={} at {}",
                liquidation.symbol,
                liquidation.side,
                liquidation.amount,
                liquidation.price,
                liquidation.ts,
            );

            let i_liq = LiquidationNew {
                ts: liquidation.ts,
                symbol: liquidation.symbol,
                side: liquidation.side,
                amount: liquidation.amount,
                price: liquidation.price,
                exchange: "bybit".to_string(),
            };

            let liquidation_query =
                queries::liquidations::write_tables::q_insert_liquidations(&i_liq)
                    .unwrap();
            let _ = ch_lq_client.write_table(&liquidation_query).await;
        }
    });

    // Clone client for orderbook collection task
    let ch_ob_client = ClickHouseClient::builder()
        .url(matches.get_one::<String>("url").unwrap())
        .database(matches.get_one::<String>("database").unwrap())
        .build()
        .await
        .unwrap();

    // --- Main Orderbook Collection Loop --- //
    let orderbook_task = tokio::spawn(async move {
        let interval = Duration::from_secs(1);
        let mut next_time = Instant::now() + interval;

        loop {
            let v_exchanges =
                vec![Exchange::Kraken, Exchange::Binance, Exchange::Coinbase];
            let pair = TradingPair::SolUsdc;
            let depth = 25;

            for i_exchange in v_exchanges {
                let exchange_client: Box<dyn ExchangeClient + Send + Sync> =
                    match i_exchange {
                        Exchange::Binance => Box::new(BinanceClient::new().unwrap()),
                        Exchange::Coinbase => Box::new(CoinbaseClient::new().unwrap()),
                        Exchange::Kraken => Box::new(KrakenClient::new().unwrap()),
                    };

                let r_orderbook = exchange_client
                    .get_orderbook(pair.clone(), Some(depth))
                    .await
                    .unwrap();

                let query_str: String =
                    queries::orderbooks::write_tables::q_insert_orderbook(&r_orderbook)
                        .unwrap();
                let _ = ch_ob_client.write_table(&query_str).await;

                sleep(next_time - Instant::now());
                next_time += interval;
            }
        }
    });

    // Wait for both tasks
    let _ = tokio::join!(orderbook_task, liquidation_task);
}
