#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::token::StellarAssetClient;

#[test]
fn test_happy_path_transfer() {
    let env = Env::default();
    env.mock_all_auths();

    // Setup Token
    let admin = Address::generate(&env);
    let token_contract_id = env.register_stellar_asset_contract(admin.clone());
    let token_admin_client = StellarAssetClient::new(&env, &token_contract_id);
    let token_client = TokenClient::new(&env, &token_contract_id);

    // Setup PadalaChain
    let contract_id = env.register_contract(None, PadalaChain);
    let client = PadalaChainClient::new(&env, &contract_id);

    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    let recipient_phone = String::from_str(&env, "+639171234567");
    let amount: i128 = 100_0000000;

    // Mint tokens to sender
    token_admin_client.mint(&sender, &amount);

    client.register_agent(&agent);

    // Pass token_contract_id into the create_transfer call
    let transfer_id = client.create_transfer(&sender, &agent, &token_contract_id, &amount, &recipient_phone);
    assert_eq!(transfer_id, 1);
    
    // Verify tokens were actually moved
    assert_eq!(token_client.balance(&agent), amount);
    assert_eq!(token_client.balance(&sender), 0);

    client.complete_transfer(&agent, &transfer_id);

    let transfer = client.get_transfer(&transfer_id);
    assert_eq!(transfer.status, TransferStatus::Completed);
}