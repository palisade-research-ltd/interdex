#[cfg(test)]

// -- ----------------------------------------------------------------- TESTS UTILS -- //
// -- ----------------------------------------------------------------- ----------- -- //

mod test_utils; 

mod tests {

    use std::{sync::Arc, str::FromStr};
    use anchor_client::{
        Client, Cluster,
        solana_sdk::{
            signature::Signer,
            pubkey::Pubkey,
            signature::read_keypair_file,}
        };

    use ix_program::state::ind_data::DataPrices;

    #[test]
    fn test_fetch_ind_data() {

        println!("🧪 Testing Fetch Data Prices ... ");
        
        use crate::test_utils::AnchorConfig;
        let test_config = AnchorConfig::new(
            Cluster::Localnet,
            "PROGRAM".to_string(),
            "WALLET".to_string(),
        );

        // Load helper struct
        let anchor_config: AnchorConfig = test_config.get_config();
        let payer = Arc::new(read_keypair_file(anchor_config.wallet).unwrap());
        let payer_pubkey = payer.pubkey();
        let client = Client::new(anchor_config.cluster, payer.clone());
        let pubkey = Pubkey::from_str(&anchor_config.program).unwrap();
        let program = client.program(pubkey).unwrap();

        println!(" testing account data prices fetching...");
        
        // derive pdas
        let (ind_data_pda, _) = Pubkey::find_program_address(
            &[b"ind_data", payer_pubkey.as_ref()],
            &program.id()
        );
        
        // fetch and verify 
        match program.account::<DataPrices>(ind_data_pda) {

            Ok(ind_data) => {

                println!(" successfully fetched data prices");
                println!("  - last_updated: {:?}", ind_data.last_updated);
                println!("  - current_index: {:?}", ind_data.current_index);
                println!("  - prices: {:?}", ind_data.prices);
                println!("  - timestamps: {:?}", ind_data.timestamps);
                println!("  - is_full: {:?}", ind_data.is_full);
                println!("  - authority: {:?}", ind_data.authority);
                
                // verify data integrity
                assert_eq!(ind_data.authority, payer_pubkey);
                assert!(ind_data.prices.len() == 10);
            },

            Err(e) => {
                println!("failed to fetch parameters: {}", e);
            }
        }
    }
}

