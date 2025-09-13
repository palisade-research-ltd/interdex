// src/bin/datacollector.rs

use atelier_data::{
    exchanges::bybit::{ws_decoder::BybitWssEvent, WssExchange},
    stream_data,
};
use std::{
    env,
    thread::sleep,
    time::{Duration, Instant},
};

use ix_execution::{
    liquidations::LiquidationNew, queries, trades::TradeNew, ClickHouseClient,
};

use ix_cex::{
    exchanges::{BinanceClient, BybitClient, CoinbaseClient, ExchangeClient, KrakenClient},
    models::{exchanges::Exchange, orderbook::TradingPair},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // -- take from env
    // Environment variable from docker-compose.yaml
    let ch_url = env::var("CLICKHOUSE_URL").unwrap_or_else(|_| "http://database:8123".to_string());
    let ch_db = env::var("CLICKHOUSE_DB").unwrap_or_else(|_| "operations".to_string());

    // --- LIQUIDATIONS Datacollector --- //
    let ch_lq_client = ClickHouseClient::builder()
        .url(ch_url.clone())
        .database(ch_db.clone())
        .build()
        .await
        .unwrap();

    let streams_task = tokio::spawn(async move {
        println!("started streams_task");

        let symbols = vec![
            "SOLUSDT".to_string(),
            "LINKUSDT".to_string(),
        ];

        let streams = vec!["allLiquidation".to_string(), "publicTrade".to_string()];
        let source = WssExchange::Bybit;
        let mut rx = stream_data(symbols, streams, source)
            .await
            .expect("failed to open Bybit WS");

        while let Some(recv_event) = rx.recv().await {
            println!("\nrecv_event");
            match recv_event {
                BybitWssEvent::LiquidationData(event_data) => {
                    let i_liq = LiquidationNew {
                        ts: event_data.liquidation_ts,
                        symbol: event_data.symbol.clone(),
                        side: event_data.side,
                        amount: event_data.amount,
                        price: event_data.price,
                        exchange: "Bybit".to_string(),
                    };

                    println!(
                        "\n ---- allLiquidation event received... {:?} ----\n ",
                        event_data.symbol
                    );

                    let liquidation_query =
                        queries::liquidations::write_tables::q_insert_liquidations(
                            &i_liq,
                        )
                        .unwrap();
                    println!("\n ---- liquidation_query {:?} ---- \n", liquidation_query);

                    let ch_lq_result = ch_lq_client.write_table(&liquidation_query).await;
                    println!("\n ---- ch_lq_result {:?} ---- \n", ch_lq_result);
                }

                BybitWssEvent::TradeData(event_data) => {
                    let i_trade = TradeNew {
                        trade_ts: event_data.trade_ts,
                        symbol: event_data.symbol.clone(),
                        side: event_data.side,
                        amount: event_data.amount,
                        price: event_data.price,
                        exchange: "Bybit".to_string(),
                    };

                    println!("\n ---- publicTrade event received... {:?} ---- ", event_data.symbol);

                    let trade_query =
                        queries::trades::write_tables::q_insert_trades(&i_trade).unwrap();
                    println!("\n ---- trade_query {:?} --- \n", trade_query);

                    let ch_pt_result = ch_lq_client.write_table(&trade_query).await;
                    println!("\n ---- ch_pt_result {:?} ---- \n", ch_pt_result);

                }
            }
        }
    });

    // --- ORDERBOOKS Dataollector --- //
    let ch_ob_client = ClickHouseClient::builder()
        .url(ch_url.clone())
        .database(ch_db.clone())
        .build()
        .await
        .unwrap();

    let orderbook_task = tokio::spawn(async move {
        let interval = Duration::from_millis(800);
        let mut next_time = Instant::now() + interval;

        loop {

            let v_exchanges = vec![
                Exchange::Kraken,
                Exchange::Binance,
                Exchange::Bybit,
                Exchange::Coinbase
            ];

            let v_pairs = vec![
                TradingPair::SolUsdt,
                TradingPair::LinkUsdt,
            ];

            let depth = 25;

            for i_exchange in v_exchanges {

                println!("exchange {:?}", i_exchange);

                let exchange_client: Box<dyn ExchangeClient + Send + Sync> =
                    match i_exchange {
                        Exchange::Binance => Box::new(BinanceClient::new().unwrap()),
                        Exchange::Coinbase => Box::new(CoinbaseClient::new().unwrap()),
                        Exchange::Kraken => Box::new(KrakenClient::new().unwrap()),
                        Exchange::Bybit => Box::new(BybitClient::new().unwrap()),
                    };

                for i_pair in v_pairs.clone() {
                    println!("\nOrderbook query for pair: {:?}", i_pair);

                    let r_orderbook = exchange_client
                        .get_orderbook(i_pair.clone(), Some(depth))
                        .await
                        .unwrap();

                    let ob_query: String =
                        queries::orderbooks::write_tables::q_insert_orderbook(&r_orderbook)
                            .unwrap();
                    println!("\n ---- ob_query {:?} ---- \n", ob_query);

                    let ch_ob_result = ch_ob_client.write_table(&ob_query).await;
                    println!("\n ---- ch_ob_result {:?} ---- \n", ch_ob_result);

                    sleep(next_time - Instant::now());
                    next_time += interval;

                }
            }
        }
    });

    // Wait for both tasks
    tokio::try_join!(streams_task, orderbook_task)?;
    Ok(())

}
