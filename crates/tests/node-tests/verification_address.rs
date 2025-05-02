use anyhow::Result;
use node::node_client::NodeClient;
use node_tests::init_node_client;
use starknet_liquidity_source::withdraw::MeltPaymentRequest;
use starknet_types::{Asset, is_valid_starknet_address};
use starknet_types_core::felt::Felt;
use node::MeltRequest;
use tonic::Status;

#[tokio::test]
async fn test_melt_with_valid_address() -> Result<()> {
    // Initialize node client
    let mut node_client = init_node_client().await?;
    
    // Create a valid Starknet address
    let valid_address = Felt::from_hex_unchecked(
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
    );
    
    // Confirm it's valid using the function
    assert!(is_valid_starknet_address(&valid_address));
    
    // Create a payment request with the valid address
    let payment_request = MeltPaymentRequest {
        payee: valid_address,
        asset: Asset::Strk, // Using STRK as the asset
    };
    
    // Serialize the payment request
    let serialized_request = serde_json::to_string(&payment_request)?;
    
    // Create a melt request (with empty inputs since we're just testing validation)
    let melt_request = MeltRequest {
        method: "starknet".to_string(),
        unit: "ETH".to_string(),
        request: serialized_request,
        inputs: vec![],  // Empty inputs for testing
    };
    
    // Send the request - it should pass address validation but fail due to empty inputs
    // We're only concerned with whether it passes address validation
    let result = node_client.melt(melt_request).await;
    
    // Validate: It should fail, but NOT with an invalid starknet address error
    // (It would fail with something like "no inputs" or "insufficient funds")
    match result {
        Err(status) => {
            // We expect an error, but ensure it's not an "Invalid starknet address" error
            assert!(!status.message().contains("Invalid starknet address"),
                "Address validation should pass, but failed with: {}", status.message());
            // The error should be about inputs or amounts, not address validation
            println!("Test passed: Request failed as expected with error: {}", status.message());
        },
        Ok(_) => {
            // This is unlikely to happen with empty inputs
            println!("Test passed but unexpectedly got successful response");
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_melt_with_invalid_addresses() -> Result<()> {
    // Initialize node client
    let mut node_client = init_node_client().await?;
    
    // Create test cases for invalid addresses
    let invalid_addresses = [
        // Address 0 (reserved)
        (Felt::from(0u64), "Address 0 is reserved for system use"),
        // Address 1 (reserved)
        (Felt::from(1u64), "Address 1 is reserved for block mapping storage"),
        // Address at 2^251 (upper bound)
        (
            Felt::from_hex_unchecked("0x800000000000000000000000000000000000000000000000000000000000000"),
            "Address at 2^251 is out of range"
        ),
        // Address above 2^251
        (
            Felt::from_hex_unchecked("0x800000000000000000000000000000000000000000000000000000000000001"),
            "Address above 2^251 is out of range"
        )
    ];
    
    for (invalid_address, description) in invalid_addresses {
        // Verify that our test address is actually invalid
        assert!(!is_valid_starknet_address(&invalid_address), 
                "{}: Address should be invalid", description);
                
        // Create a payment request with the invalid address
        let payment_request = MeltPaymentRequest {
            payee: invalid_address,
            asset: Asset::Strk,
        };
        
        // Serialize the payment request
        let serialized_request = serde_json::to_string(&payment_request)?;
        
        // Create a melt request (with empty inputs since we're just testing validation)
        let melt_request = MeltRequest {
            method: "starknet".to_string(),
            unit: "ETH".to_string(),
            request: serialized_request,
            inputs: vec![],
        };
        
        // Send the request - it should fail with an invalid address error
        let result = node_client.melt(melt_request).await;
        
        // Validate: It should fail with an "Invalid starknet address" error
        match result {
            Err(status) => {
                // Check if the error message indicates an invalid address
                assert!(status.message().contains("Invalid starknet address") || 
                       status.message().contains("invalid starknet address"),
                    "Expected an invalid address error, but got: {}", status.message());
                    
                println!("Test passed: Invalid address {} correctly rejected with error: {}", 
                         invalid_address, status.message());
            },
            Ok(_) => {
                panic!("Test failed: Request with invalid address {} was accepted", invalid_address);
            }
        }
    }
    
    Ok(())
}