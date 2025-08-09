/// Instrument related endpoints
use crate::client::http_client::RequestType;
use crate::exchanges::bybit::responses::InstrumentResponse;
use crate::BybitPrivateClient;
use ix_results::errors::Result;
use tracing::info;

impl BybitPrivateClient {

    /// Get the instrument info
    pub async fn get_instrument_info(
        &self,
        p_category: &str,
        p_symbol: Option<&str>,
    ) -> Result<InstrumentResponse> {
        info!("Fetching Instrumento Info");
        let mut params = vec![("category", p_category)];

        if let Some(p_symbol) = p_symbol {
            params.push(("symbol", p_symbol));
        }

        let request_type = RequestType::Get;

        self.request_private("/v5/market/instruments-info", &params, request_type)
            .await
    }

}

