#[cfg(test)]
mod helius_rpc_utils {

    pub const DEVNET: &str = "https://api-devnet.helius-rpc.com/";
    pub const TX_URL: &str = "v0/transactions/";
    pub const API_KEY: &str = "?api-key=";

}

mod tests {

    // use tokio;
    use crate::helius_rpc_utils::*;
    use ix_data::data::SolanaResponse2;
    use ix_sources::helius::HeliusRpc;

    // --- ------------------------------------------------------------ GET VERSION --- //
    // --- ------------------------------------------------------------ ---------- --- //

    #[tokio::test]
    async fn test_rpc() {

        let rpc_url: String = DEVNET.to_owned() + TX_URL + API_KEY;
        let helius_client = HeliusRpc::get_client();

    }

    // --- ------------------------------------------------------------- GET BLOCK --- //
    // --- ------------------------------------------------------------- --------- --- //

    #[tokio::test]
    async fn test_get_block() {

        println!("\nTest: Solana RPC call to getBlock\n");

        let s_client = new_solana_client(DEVNET).unwrap();
        let test_block = 337288619;
        let block_response  = s_client.get_block(test_block).await;

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
           
            let dimensions:(u8, u8) = match pfr_response {
                Ok(response) => {
                let len_slots = response.slots.unwrap().len() as u8;
                let len_fees = response.fees.unwrap().len() as u8;
                    (len_slots, len_fees)
                },
                _ => {
                    (0, 0)
                }

            };
            
            println!("According to the Docs:");
            println!("The getRecentPrioritizationFees stores up to 150 blocks\n");
            println!("assert_eq!(response.slots.len(), 150)");
            println!("assert_eq!(response.fees.len(), 150)");
            
            let results = (
                if dimensions.0 == 150 { true } else { false },
                if dimensions.1 == 150 { true } else { false }
            );
            
            assert_eq!(results, (true, true));
            
    }

}

