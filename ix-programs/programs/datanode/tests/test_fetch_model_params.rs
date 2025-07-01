

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

    use datanode::state::model_params::ModelParameters;

    #[test]
    fn test_fetch_model_params() {

        println!("🧪 Testing Fetch Model Params ... ");
        
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

        println!(" testing account data fetching...");
        
        // derive pdas
        let (model_params_pda, _) = Pubkey::find_program_address(
            &[b"model_params", payer_pubkey.as_ref()],
            &program.id()
        );
        
        // fetch and verify model parameters
        match program.account::<ModelParameters>(model_params_pda) {

            Ok(model_params) => {

                println!(" successfully fetched model parameters");
                println!("  - weights: {:?}", model_params.weights);
                println!("  - bias: {}", model_params.bias);
                println!("  - authority: {}", model_params.authority);
                
                // verify data integrity
                assert_eq!(model_params.authority, payer_pubkey);
                assert!(model_params.weights.len() == 5);
            },

            Err(e) => {
                println!("failed to fetch model parameters: {}", e);
            }
        }
    }
}

