// src/bin/collector.rs

use clap::{Arg, Command};

use ix_cex::{
    exchanges::{BinanceClient, CoinbaseClient, ExchangeClient, KrakenClient},
    models::{exchanges::Exchange, orderbook::TradingPair},
};

use ix_database::{
    queries::orderbooks::{
        create_tables::create_orderbooks_table_ddl, write_tables::q_insert_orderbook,
    },
    ClickHouseClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let ch_client = ClickHouseClient::builder()
        .url(matches.get_one::<String>("url").unwrap())
        .database(matches.get_one::<String>("database").unwrap())
        .build()
        .await?;

    // Create orderbooks table if it doesn't exist
    ch_client
        .create_table(&create_orderbooks_table_ddl())
        .await?;

    // Loop (first slowest: Kraken)

    let v_exchanges = vec![Exchange::Kraken, Exchange::Binance, Exchange::Coinbase];
    let pair = TradingPair::SolUsdc;
    let depth = 10;

    for i_exchange in v_exchanges {
        let exchange_client: Box<dyn ExchangeClient + Send + Sync> = match i_exchange {
            Exchange::Binance => Box::new(BinanceClient::new()?),
            Exchange::Coinbase => Box::new(CoinbaseClient::new()?),
            Exchange::Kraken => Box::new(KrakenClient::new()?),
        };

        let r_orderbook = exchange_client
            .get_orderbook(pair.clone(), Some(depth))
            .await?;

        let query_str: String = q_insert_orderbook(&r_orderbook).unwrap();
        println!("query_str for {:?} was: \n{:?}", i_exchange, query_str);

        ch_client.write_table(&query_str).await?;
    }

    Ok(())
}
