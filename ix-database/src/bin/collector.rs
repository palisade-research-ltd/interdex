// src/bin/collector.rs
//

use ix_cex::{
    exchanges::{BinanceClient, CoinbaseClient, ExchangeClient, KrakenClient},
    models::TradingPair,
    ExchangeError,
};
use tokio::time::{sleep, timeout, Duration};

use clap::{Arg, Command};
use ix_database::ClickHouseClient;

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

    let client = ClickHouseClient::builder()
        .url(matches.get_one::<String>("url").unwrap())
        .database(matches.get_one::<String>("database").unwrap())
        .build()
        .await?;

    // Create orderbooks table if it doesn't exist
    client
        .create_table(&ix_database::create_orderbooks_table_ddl())
        .await?;

    // Read JSON from ix-cex source
    //

    #[derive(Clone, Debug)]
    enum Exchange {
        Binance,
        Coinbase,
        Kraken,
    }

    #[derive(Clone, Debug)]
    enum TradingPairArg {
        BtcUsdc,
        SolUsdc,
    }

    async fn query_exchange(
        exchange: Exchange,
        pair: TradingPair,
        depth: u32,
        timeout_secs: u64,
        timewait_millis: u64,
    ) -> Result<ix_cex::models::OrderBook, ExchangeError> {
        let client: Box<dyn ExchangeClient + Send + Sync> = match exchange {
            Exchange::Binance => Box::new(BinanceClient::new()?),
            Exchange::Coinbase => Box::new(CoinbaseClient::new()?),
            Exchange::Kraken => Box::new(KrakenClient::new()?),
        };

        sleep(Duration::from_millis(timewait_millis)).await;

        let request_future = client.get_orderbook(pair, Some(depth));

        timeout(Duration::from_secs(timeout_secs), request_future)
            .await
            .map_err(|_| {
                ExchangeError::Timeout(format!(
                    "Request timed out after {timeout_secs} seconds"
                ))
            })?
    }

    let result = query_exchange(Exchange::Binance, TradingPair::SolUsdc, 10, 2, 20).await;

    println!("Collector finished : result {:?}", result);

    Ok(())
}
