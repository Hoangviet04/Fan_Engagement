#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Map, String};

#[derive(Clone)]
#[soroban_sdk::contracttype]
pub enum DataKey {
    Admin,
    NFTs,
    NFTCounter,
    RoyaltyInfo,
}

#[derive(Clone)]
#[soroban_sdk::contracttype]
pub struct NFT {
    pub owner: Address,
    pub creator: Address,
    pub metadata_url: String,
}

#[derive(Clone)]
#[soroban_sdk::contracttype]
pub struct RoyaltyInfo {
    pub creator: Address,
    pub rate: u32, // e.g., 500 for 5%
}

#[contract]
pub struct FanEngagementContract;

#[contractimpl]
impl FanEngagementContract {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::NFTCounter, &0u64);
        env.storage()
            .instance()
            .set(&DataKey::NFTs, &Map::<u64, NFT>::new(&env));
    }

    pub fn mint_nft(env: Env, owner: Address, creator: Address, metadata_url: String) -> u64 {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut counter: u64 = env.storage().instance().get(&DataKey::NFTCounter).unwrap();
        counter += 1;

        let nft = NFT {
            owner: owner.clone(),
            creator: creator.clone(),
            metadata_url,
        };

        let mut nfts: Map<u64, NFT> = env.storage().instance().get(&DataKey::NFTs).unwrap();
        nfts.set(counter, nft);

        env.storage().instance().set(&DataKey::NFTs, &nfts);
        env.storage().instance().set(&DataKey::NFTCounter, &counter);

        // Emit event
        let topics = (symbol_short!("mint"), owner, creator);
        env.events().publish(topics, counter);

        counter
    }

    pub fn transfer_nft(env: Env, from: Address, to: Address, token_id: u64) {
        from.require_auth();

        let mut nfts: Map<u64, NFT> = env.storage().instance().get(&DataKey::NFTs).unwrap();
        let mut nft = nfts.get(token_id).unwrap();

        if nft.owner != from {
            panic!("'from' address is not the owner");
        }

        nft.owner = to.clone();
        nfts.set(token_id, nft);
        env.storage().instance().set(&DataKey::NFTs, &nfts);

        // Emit event
        let topics = (symbol_short!("transfer"), from, to);
        env.events().publish(topics, token_id);
    }

    pub fn get_nft(env: Env, token_id: u64) -> Option<NFT> {
        let nfts: Map<u64, NFT> = env.storage().instance().get(&DataKey::NFTs).unwrap();
        nfts.get(token_id)
    }

    pub fn pay_royalty(env: Env, buyer: Address, token_id: u64, amount: u128) {
        buyer.require_auth();

        let nfts: Map<u64, NFT> = env.storage().instance().get(&DataKey::NFTs).unwrap();

        if !nfts.contains_key(token_id) {
            panic!("NFT does not exist");
        }

        let nft = nfts.get(token_id).unwrap();

        // In a real-world scenario, you would integrate with a token contract
        // to transfer the `amount` from the `buyer` to the `nft.creator`.
        // For this example, we'll just emit an event to simulate the payment.

        let topics = (symbol_short!("royalty"), nft.creator, buyer);
        env.events().publish(topics, amount);
    }

    pub fn set_royalty(env: Env, creator: Address, rate: u32) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let royalty_info = RoyaltyInfo { creator, rate };
        env.storage()
            .instance()
            .set(&DataKey::RoyaltyInfo, &royalty_info);
    }
}

// ```

// **CÁC BƯỚC BẮT BUỘC BẠN PHẢI LÀM:**

// Vì mã nguồn đã thay đổi, bạn phải bắt đầu lại quá trình triển khai:

// 1.  **Chạy `deploy.sh`:**
//     ```bash
//     ./deploy.sh
//     ```
//     Lệnh này sẽ biên dịch mã nguồn mới và trả về một **CONTRACT ID MỚI**. Hãy sao chép ID này.

// 2.  **Cập nhật các tệp Script:**
//     * Mở các tệp `initialize.sh`, `mint_nft.sh`, `transfer_nft.sh`, `get_nft_details.sh`, và `pay_royalty.sh`.
//     * Trong mỗi tệp, hãy **dán Contract ID mới** vào biến `CONTRACT_ID`.

// 3.  **Khởi tạo hợp đồng mới:**
//     ```bash
//     ./initialize.sh
//     ```

// 4.  **Đúc một NFT mới:**
//     ```bash
//     ./mint_nft.sh
