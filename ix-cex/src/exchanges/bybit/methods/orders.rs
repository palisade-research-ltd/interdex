// private
use crate::exchanges::bybit::{
    clients::BybitPrivateClient,
    methods::{OrderResponse, TradeResponse},
};

use crate::{
    client::http_client::RequestType, exchanges::bybit::responses::CancelResponse,
};

use ix_results::errors::{ExchangeError, Result};
use tracing::info;

/// Get Open & Closed Orders
pub async fn get_orders(
    client: BybitPrivateClient,
    p_category: &str,
) -> Result<OrderResponse> {
    info!("Fetching Bybit orders");
    let params = vec![("category", p_category)];
    let request_type = RequestType::Get;

    client
        .request_private("/v5/order/realtime", &params, request_type)
        .await
}

/// Cancell All Open Orders
pub async fn cancel_orders(
    client: BybitPrivateClient,
    p_category: &str,
    p_symbol: Option<&str>,
    p_base_coin: Option<&str>,
    p_settle_coin: Option<&str>,
    p_order_filter: Option<&str>,
) -> Result<CancelResponse> {
    info!("Pushing a Bybit cancel all orders");

    let mut params = vec![("category", p_category)];
    let request_type = RequestType::Post;

    if let Some(symbol) = p_symbol {
        params.push(("p_symbol", symbol));
    }

    if let Some(base_coin) = p_base_coin {
        params.push(("p_base_coin", base_coin));
    }

    if let Some(settle_coin) = p_settle_coin {
        params.push(("p_settle_coin", settle_coin));
    }

    if let Some(order_filter) = p_order_filter {
        params.push(("p_order_filter", order_filter));
    }

    client
        .request_private("/v5/order/cancel-all", &params, request_type)
        .await
}

/// Open a New Order
pub async fn new_order(
    client: BybitPrivateClient,
    p_category: &str,
    p_symbol: &str,
    p_side: &str,
    p_order_type: &str,
    p_qty: &str,
) -> Result<TradeResponse> {
    info!("Post Bybit New Order");

    let p_endpoint = "/v5/order/create".to_string();

    let p_params = vec![
        ("category", p_category),
        ("symbol", p_symbol),
        ("side", p_side),
        ("orderType", p_order_type),
        ("qty", p_qty),
    ];

    let p_request_type = RequestType::Post;
    let response: &TradeResponse = client
        .request_private(&p_endpoint, &p_params, p_request_type)
        .await?;

    if response.ret_code != 0 {
        return Err(ExchangeError::ApiError {
            exchange: "Bybit".to_string(),
            message: format!(
                "Bybit API Error\n Code: {:?} Message: {:?}",
                response.ret_code, response.ret_msg,
            ),
        });
    }

    Ok(response.clone())
}
