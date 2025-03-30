use ic_cdk::api::management_canister::bitcoin::Satoshi;
use ree_types::bitcoin::key::{Secp256k1, TapTweak, TweakedPublicKey};

use crate::*;

pub(crate) fn tweak_pubkey_with_empty(untweaked: Pubkey) -> TweakedPublicKey {
    let secp = Secp256k1::new();
    let (tweaked, _) = untweaked.to_x_only_public_key().tap_tweak(&secp, None);
    tweaked
}

pub(crate) fn get_chain_second_timestamp()-> SecondTimestamp {
    ic_cdk::api::time() / 1000_000_000
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct RegisterInfo {
    pub untweaked_key: Pubkey,
    pub tweaked_key: Pubkey,
    pub address: String,
    pub utxo: Utxo,
    pub register_fee: Satoshi,
    pub nonce: u64,
}

#[test]
pub fn test_tweak_pubkey() {
    let mock_raw_pubkey: Vec<u8> = vec![
        0x02, 
        0x79, 0xBE, 0x66, 0x7E, 0xF9, 0xDC, 0xBB, 0xAC, 
        0x55, 0xA0, 0x62, 0x95, 0xCE, 0x87, 0x0B, 0x07, 0x02, 0x9B, 0xFC, 0xDB, 0x2D, 0xCE, 0x28,
        0xD9, 0x59, 0xF2, 0x81, 0x5B, 0x16, 0xF8, 0x17, 0x98,
    ];

    let pubkey = Pubkey::from_raw(mock_raw_pubkey).unwrap();
    let tweaked_pubkey = tweak_pubkey_with_empty(pubkey.clone());
    let addr = ree_types::bitcoin::Address::p2tr_tweaked(tweaked_pubkey, ree_types::bitcoin::Network::Bitcoin);
    dbg!(&addr);

}
