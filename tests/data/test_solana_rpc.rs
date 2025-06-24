#[cfg(test)]

// ---
// --- 

mod test_client_utils {

    use ci_data::clients::{SolanaRpc, SolanaRpcBuilder};

    pub fn new_solana_client() -> Result<SolanaRpc, Box<dyn std::error::Error>> {

        let url_sol = "https://api.devnet.solana.com";
        let s_client = SolanaRpcBuilder::new()
            .url(url_sol.to_string())
            .build()
            .expect("Failed to build SolanaRpc client");

        Ok(s_client)

    }

}

mod tests {

    use tokio;
    use crate::test_client_utils::*;

    // --- 
    // ---

    #[tokio::test]
    async fn test_solana_rpc() {
           
            println!("\nTest:\nSolana RPC call to getRecentPrioritizationFees");

            let s_client = new_solana_client().unwrap();
            let i_acc = "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2".to_string(); 
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
            
            println!("\nAccording to the Docs:");
            println!("The getRecentPrioritizationFees stores up to 150 blocks");
            
            let results = (
                if dimensions.0 == 150 { true } else { false },
                if dimensions.1 == 150 { true } else { false }
            );
            
            assert_eq!(results, (true, true));
            
        }

}

