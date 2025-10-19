#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, Address, Env, Map, String, Symbol,
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Creator,
    NFTCounter,
    NFTs,
    Royalties,
}

#[derive(Clone)]
#[contracttype]
pub struct NFT {
    pub owner: Address,
    pub creator: Address,
    pub uri: String,
}

#[contract]
pub struct FanEngagementContract;

#[contractimpl]
impl FanEngagementContract {
    /// Initialize the contract with an administrator.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::NFTCounter, &0u64);
        env.storage()
            .instance()
            .set(&DataKey::NFTs, &Map::<u64, NFT>::new(&env));
        env.storage()
            .instance()
            .set(&DataKey::Royalties, &Map::<u64, u64>::new(&env));
    }

    /// Mint a new NFT.
    pub fn mint_nft(env: Env, creator: Address, owner: Address, uri: String) {
        creator.require_auth();
        let mut nft_counter: u64 = env.storage().instance().get(&DataKey::NFTCounter).unwrap();
        nft_counter += 1;

        let nft = NFT {
            owner: owner.clone(),
            creator: creator.clone(),
            uri,
        };

        let mut nfts: Map<u64, NFT> = env.storage().instance().get(&DataKey::NFTs).unwrap();
        nfts.set(nft_counter, nft);

        env.storage().instance().set(&DataKey::NFTs, &nfts);
        env.storage()
            .instance()
            .set(&DataKey::NFTCounter, &nft_counter);

        let topics = (symbol_short!("mint"), owner, creator);
        env.events().publish(topics, nft_counter);
    }

    /// Transfer an NFT to a new owner.
    pub fn transfer_nft(env: Env, from: Address, to: Address, token_id: u64) {
        from.require_auth();

        let mut nfts: Map<u64, NFT> = env.storage().instance().get(&DataKey::NFTs).unwrap();
        let mut nft = nfts.get(token_id.clone()).unwrap();

        if nft.owner != from {
            panic!("Not the owner");
        }

        nft.owner = to.clone();
        nfts.set(token_id.clone(), nft);
        env.storage().instance().set(&DataKey::NFTs, &nfts);

        let topics = (symbol_short!("transfer"), from, to);
        env.events().publish(topics, token_id);
    }

    /// Set the royalty percentage for an NFT.
    pub fn set_royalty(env: Env, creator: Address, token_id: u64, amount: u64) {
        creator.require_auth();
        let mut royalties: Map<u64, u64> =
            env.storage().instance().get(&DataKey::Royalties).unwrap();
        royalties.set(token_id, amount);
        env.storage()
            .instance()
            .set(&DataKey::Royalties, &royalties);
    }

    /// Pay royalty for a secondary sale.
    pub fn pay_royalty(
        env: Env,
        buyer: Address,
        token_id: u64,
        amount: i128,
        token_address: Address,
    ) {
        buyer.require_auth();

        let nfts: Map<u64, NFT> = env.storage().instance().get(&DataKey::NFTs).unwrap();
        let nft = nfts.get(token_id.clone()).unwrap();

        let royalties: Map<u64, u64> = env.storage().instance().get(&DataKey::Royalties).unwrap();
        let royalty_percentage = royalties.get(token_id).unwrap_or(0);

        if royalty_percentage > 0 {
            let royalty_amount = (amount * royalty_percentage as i128) / 100;
            let token = token::Client::new(&env, &token_address);
            token.transfer(&buyer, &nft.creator, &royalty_amount);
        }
    }

    /// Get NFT details.
    pub fn get_nft(env: Env, token_id: u64) -> Option<NFT> {
        let nfts: Map<u64, NFT> = env.storage().instance().get(&DataKey::NFTs).unwrap();
        nfts.get(token_id)
    }
}
