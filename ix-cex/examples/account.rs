use ix_cex::BybitPrivateClient;

#[tokio::main]
async fn main() {
    let client = BybitPrivateClient::new().unwrap();
    let p_account_type = "UNIFIED";
    let p_coin = Some("USD");
    let result = client.get_wallet_balance(p_account_type, p_coin).await;

    println!("result {:?}", result);
}
