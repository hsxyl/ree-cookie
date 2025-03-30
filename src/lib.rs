pub mod canister;
pub mod errors;
pub mod external;
pub mod game;
pub mod memory;
pub mod state;
pub mod utils;
pub mod log;
pub mod psbt;

pub use candid::{Principal, CandidType};
pub use errors::*;
pub use external::rune_indexer::{RuneEntry, Service as RuneIndexer};
pub use ic_stable_structures::StableBTreeMap;
pub use ree_types::Pubkey;
pub use ree_types::{Txid, Utxo};
pub use serde::{Deserialize, Serialize};
pub use ic_canister_log::log;
pub use log::*;

pub const RUNE_INDEXER_CANISTER: &'static str = "kzrva-ziaaa-aaaar-qamyq-cai";
pub const TESTNET_RUNE_INDEXER_CANISTER: &'static str = "f2dwm-caaaa-aaaao-qjxlq-cai";
pub const ORCHESTRATOR_CANISTER: &'static str = "kqs64-paaaa-aaaar-qamza-cai";
pub type Seconds = u64;
pub type SecondTimestamp = u64;
pub type PoolId = Pubkey;
pub type Address = String;
pub const MIN_BTC_VALUE: u64 = 10000;
