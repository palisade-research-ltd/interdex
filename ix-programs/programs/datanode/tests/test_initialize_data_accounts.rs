#[cfg(test)]

// -- ----------------------------------------------------------------- TESTS UTILS -- //
// -- ----------------------------------------------------------------- ----------- -- //

mod test_utils; 

mod tests {

    use std::{sync::Arc, str::FromStr};
    use anchor_lang::system_program;
    use anchor_client::{
        Client, Cluster,
        solana_sdk::{
            signature::Signer,
            pubkey::Pubkey,
            signature::read_keypair_file,}
        };
    use solana_sdk::signer::keypair::Keypair;

    #[test]
    fn test_initialize_pf_accounts() -> Result<(), Box<dyn std::error::Error>> {

        println!("🧪 Testing Priority Fees accounts Initialization... ");

        use crate::test_utils::AnchorConfig;

        let test_config = AnchorConfig::new(
            Cluster::Devnet,
            "PROGRAM".to_string(),
            "WALLET".to_string(),
        );

        // Load helper struct
        let anchor_config: AnchorConfig = test_config.get_config();
        println!("{:?}", anchor_config.wallet.to_string());

        let keypair_file: Keypair = read_keypair_file(
            anchor_config
                .wallet
                .to_string())
            .unwrap();
        
        let payer = Arc::new(keypair_file);
        let payer_pubkey = payer.pubkey();
        let client = Client::new(anchor_config.cluster, payer.clone());
        let pubkey = Pubkey::from_str(&anchor_config.program).unwrap();
        let program = client.program(pubkey).unwrap();

        println!(" testing priority fees accounts ");
        
        // derive pdas
        let (pf_buffer_pda, _) = Pubkey::find_program_address(
            &[b"pf_buffer", payer_pubkey.as_ref()],
            &program.id()
        );
        
        // derive pdas
        let (pf_stats_pda, _) = Pubkey::find_program_address(
            &[b"pf_stats", payer_pubkey.as_ref()],
            &program.id()
        );
        
        // Test each initialization
        let accounts_to_test = vec![
            ("pf_buffer", pf_buffer_pda, "InitializePFBuffer"),
            ("pf_stats", pf_stats_pda, "InitializePFStats"),
        ];
        
        for (name, pda, instruction_name) in accounts_to_test {
            println!(" Testing {} initialization...", name);
            
            let account_exists = program.rpc()
                .get_account(&pda)
                .is_ok();
            
            if account_exists {
                println!(" {} account already exists", name);
                continue;
            }
            
            // Call appropriate initialization instruction
            let result = match instruction_name {

                "InitializePFBuffer" => {

                program
                    .request()
                    .accounts(datanode::accounts::InitializePFBuffer {
                        pf_buffer: pf_buffer_pda,
                        authority: payer_pubkey,
                        system_program: system_program::ID,
                    })
                    .args(datanode::InitializePFBuffer {})
                    .signer(&payer)
                    .send()
                },

                "InitializePFStats" => {

                program
                    .request()
                    .accounts(datanode::accounts::InitializePFStats {
                        pf_stats: pf_stats_pda,
                        authority: payer_pubkey,
                        system_program: system_program::ID,
                    })
                    .args(datanode::InitializePFStats {})
                    .signer(&payer)
                    .send()
                },

                _ => continue,

            };
            
            match result {
                Ok(signature) => {
                    println!(" {} initialization successful: {}", name, signature);
                },
                Err(e) => {
                    println!(" {} initialization failed: {}", name, e);
                    return Err(format!("Failed to initialize {}: {}", name, e).into());
                }
            }
        }
        
        Ok(())
    }
}
