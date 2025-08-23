// src/bin/trader.rs

use ix_cex::BybitPrivateClient;
// use atelier_data::orderbooks::Orderbook;

#[tokio::main]
async fn main() {

    // --- Get Balance
    let client = BybitPrivateClient::new().unwrap();
    let p_account_type = "UNIFIED";
    let p_coin = Some("USD");
    let wallet_balance = client.get_wallet_balance(p_account_type, p_coin).await;
    let wb = wallet_balance.unwrap().result.list[0].total_equity.clone();

    println!("\n ---- Wallet Balance: {:?} ", wb);

    // --- 

    // --- Open a trade
    let client = BybitPrivateClient::new().unwrap();
    let p_category = "spot";
    let p_symbol = "SOLUSDT";
    let p_side = "Sell";

    let p_order_type = "Market";
    let p_qty = "5.0";
    let open_trade = client
        .new_order(p_category, p_symbol, p_side, p_order_type, p_qty)
        .await;

    let ot = open_trade.unwrap();

    println!("\n ---- Opened Trade: {:?} ", ot);
}

