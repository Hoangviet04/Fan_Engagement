#![cfg(test)]

use super::{FanEngagementContract, FanEngagementContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup_test() -> (Env, FanEngagementContractClient, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, FanEngagementContract);
    let client = FanEngagementContractClient::new(&env, &contract_id);
    let admin = Address::random(&env);
    client.initialize(&admin);
    (env, client, admin)
}

#[test]
fn test_initialize() {
    let (env, client, admin) = setup_test();
    // Re-initialization should fail.
    let res = client.try_initialize(&admin);
    assert!(res.is_err());
}

#[test]
fn test_mint_nft() {
    let (_env, client, _admin) = setup_test();
    let creator = Address::random(&_env);
    let owner = Address::random(&_env);
    let uri = String::from_str(&_env, "https://example.com/nft.json");
    client.mint_nft(&creator, &owner, &uri);

    let nft = client.get_nft(&1);
    assert!(nft.is_some());
    let nft_data = nft.unwrap();
    assert_eq!(nft_data.creator, creator);
    assert_eq!(nft_data.owner, owner);
    assert_eq!(nft_data.uri, uri);
}

#[test]
fn test_transfer_nft() {
    let (env, client, _admin) = setup_test();
    let creator = Address::random(&env);
    let owner1 = Address::random(&env);
    let owner2 = Address::random(&env);
    let uri = String::from_str(&env, "ipfs://some-hash");
    client.mint_nft(&creator, &owner1, &uri);

    client.transfer_nft(&owner1, &owner2, &1);
    let nft = client.get_nft(&1).unwrap();
    assert_eq!(nft.owner, owner2);
}

#[test]
#[should_panic(expected = "Not the owner")]
fn test_transfer_nft_not_owner() {
    let (env, client, _admin) = setup_test();
    let creator = Address::random(&env);
    let owner1 = Address::random(&env);
    let owner2 = Address::random(&env);
    let another_account = Address::random(&env);
    let uri = String::from_str(&env, "ipfs://some-hash");
    client.mint_nft(&creator, &owner1, &uri);
    client.transfer_nft(&another_account, &owner2, &1);
}

#[test]
fn test_royalties() {
    let (env, client, _admin) = setup_test();
    let creator = Address::random(&env);
    let owner = Address::random(&env);
    let buyer = Address::random(&env);
    let uri = String::from_str(&env, "ipfs://some-hash-2");
    let token_address = env.register_stellar_asset_contract(creator.clone());

    client.mint_nft(&creator, &owner, &uri);
    client.set_royalty(&creator, &1, &10); // 10% royalty

    let token_client = soroban_sdk::token::Client::new(&env, &token_address);
    token_client.mint(&buyer, &1000);

    client.pay_royalty(&buyer, &1, &1000, &token_address);

    assert_eq!(token_client.balance(&buyer), 900);
    assert_eq!(token_client.balance(&creator), 100);
}
