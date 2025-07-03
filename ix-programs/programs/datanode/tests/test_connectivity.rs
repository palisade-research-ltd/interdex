#[cfg(test)]

// -- ---------------------------------------------------------------- DEVNET TESTS -- //
// -- ---------------------------------------------------------------- ------------ -- //

mod tests {
    
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::commitment_config::CommitmentConfig;

    #[test]
    fn verify_connectivity() {
        
        println!(" Verifying connectivity...");
        
        let client = RpcClient::new_with_commitment(
            std::env::var("NETWORK").unwrap().to_string(),
            CommitmentConfig::confirmed()
        );
        
        // Test basic connectivity
        let version = client.get_version();
        assert!(
            version.is_ok(),
            "Failed to client.get_version {:?}", version.err())
       
    }
}

