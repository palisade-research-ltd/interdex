#[cfg(test)]
// -- ----------------------------------------------------------------- TESTS UTILS -- //
// -- ----------------------------------------------------------------- ----------- -- //
mod test_utils;

mod tests {

    use anchor_client::{
        solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
        Client, Cluster,
    };
    use std::{str::FromStr, sync::Arc};

    #[test]
    fn test_program_exists() -> Result<(), Box<dyn std::error::Error>> {
        use crate::test_utils::AnchorConfig;
        let test_config = AnchorConfig::new(
            Cluster::Devnet,
            "PROGRAM".to_string(),
            "WALLET".to_string(),
        );

        // Load helper struct
        let anchor_config: AnchorConfig = test_config.get_config();
        
        println!(
            "\nProgram {:?} \nNetwork {:?}",
            anchor_config.program, anchor_config.cluster
        );

        let payer = Arc::new(read_keypair_file(anchor_config.wallet).unwrap());

        let client = Client::new(anchor_config.cluster, payer.clone());
        let pubkey = Pubkey::from_str(&anchor_config.program).unwrap();
        let program = client.program(pubkey).unwrap();

        // Try to get program account info
        let account_info = program.rpc().get_account(&program.id());

        match account_info {
            Ok(account) => {
                println!("\nProgram account found on {:?}", test_config.cluster);
                println!("  - Owner: {}", account.owner);
                println!("  - Executable: {}", account.executable);
                println!("  - Lamports: {}", account.lamports);
                assert!(account.executable, "Program account should be executable");
            }
            Err(e) => {
                return Err(format!("Error fetching program account: {}", e).into());
            }
        }
        Ok(())
    }
}
