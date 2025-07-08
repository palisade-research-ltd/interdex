#[cfg(test)]
mod test_client_utils {

    use ix_dex::solana::{SolanaRpc, SolanaRpcBuilder};

    pub const WALLET_1: &str = "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2";
    pub const DEVNET: &str = "https://api.devnet.solana.com";
    // pub const LOCALNET:&str = "https://localhost::8099";
    // pub const TESTNET: &str = "https://api.testnet.solana.com";
    // pub const MAINNET: &str = "https://api.mainnet-beta.solana.com";

    pub fn new_solana_client(url: &str) -> Result<SolanaRpc, Box<dyn std::error::Error>> {
        let s_client = SolanaRpcBuilder::new()
            .url(url.to_string())
            .build()
            .expect("Failed to build SolanaRpc client");

        Ok(s_client)
    }
}

mod tests {

    // use tokio;
    use crate::test_client_utils::*;
    use ix_core::data::SolanaResponse2;

    // --- ------------------------------------------------------------- GET BLOCK --- //
    // --- ------------------------------------------------------------- --------- --- //

    #[tokio::test]
    async fn test_get_block() {
        println!("\nTest: Solana RPC call to getBlock\n");

        let s_client = new_solana_client(DEVNET).unwrap();
        let test_block = 337288619;
        let block_response = s_client.get_block(test_block).await;

        let block_data: SolanaResponse2 = block_response.unwrap();
        let previous_test_block = block_data.result.unwrap().parent_slot.unwrap();

        println!("assert_eq!(test_block - 1, previous_test_block);");
        assert_eq!(test_block - 1, previous_test_block);
    }

    // --- ----------------------------------------------- GET PRIORITY FEE RECENT --- //
    // --- ----------------------------------------------- ----------------------- --- //

    #[tokio::test]
    async fn test_get_priority_fee_recent() {
        println!("\nTest: Solana RPC call to getRecentPrioritizationFees\n");

        let s_client = new_solana_client(DEVNET).unwrap();
        let i_acc = WALLET_1.to_string();
        let v_accounts = vec![i_acc];
        let pfr_response = s_client.get_priority_fee_recent(v_accounts).await;

        let dimensions: (u8, u8) = match pfr_response {
            Ok(response) => {
                let len_slots = response.slots.unwrap().len() as u8;
                let len_fees = response.fees.unwrap().len() as u8;
                (len_slots, len_fees)
            }
            _ => (0, 0),
        };

        println!("According to the Docs:");
        println!("The getRecentPrioritizationFees stores up to 150 blocks\n");
        println!("assert_eq!(response.slots.len(), 150)");
        println!("assert_eq!(response.fees.len(), 150)");

        let results = (
            if dimensions.0 == 150 { true } else { false },
            if dimensions.1 == 150 { true } else { false },
        );

        assert_eq!(results, (true, true));
    }
}
