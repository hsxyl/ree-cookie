
use std::str::FromStr;

use crate::{
    external::{internal_identity::get_principal, management::request_schnorr_key}, game::{game::{Game, GameAndGamer}, gamer::Gamer}, memory::{mutate_state, read_state, set_state}, state::{ExchangeState, PoolState}, utils::{tweak_pubkey_with_empty, RegisterInfo}, ExchangeError, Seconds, MIN_BTC_VALUE
};
use candid::Principal;
use ic_cdk::{api::management_canister::bitcoin::Satoshi, init, post_upgrade, query, update};
use ree_types::{
    bitcoin::{Address, Network, Psbt }, exchange_interfaces::{
        ExecuteTxArgs, ExecuteTxResponse, FinalizeTxArgs, FinalizeTxResponse,
        GetMinimalTxValueArgs, GetMinimalTxValueResponse, GetPoolInfoArgs, GetPoolInfoResponse,
        GetPoolListArgs, GetPoolListResponse, PoolInfo, RollbackTxArgs, RollbackTxResponse,
    }, CoinBalance, CoinId, Intention, Pubkey, Utxo
};
pub use ic_canister_log::log;
pub use crate::log::*;

#[init]
fn init(
    rune_id_block: u64,
    rune_id_tx: u32,
    symbol: String,
    game_duration: Seconds,
    gamer_register_fee: Satoshi,
    claim_cooling_down: Seconds,
    claimed_cookies_per_click: u128,
    max_cookies: u128,
    orchestrator: Principal,
    ii_canister: Principal,
) {
    let rune_id = CoinId {
        block: rune_id_block, 
        tx: rune_id_tx
    };
    assert_ne!(rune_id, CoinId::btc(), "Can't use btc init");
    set_state(ExchangeState::init(
        rune_id,
        symbol,
        game_duration,
        gamer_register_fee,
        claim_cooling_down,
        claimed_cookies_per_click,
        max_cookies,
        orchestrator,
        ii_canister
    ));
}

#[update]
pub async fn init_key() -> Result<String, ExchangeError> {
    let (current_address, rune_id) = read_state(|s| (s.address.clone(), s.rune_id.clone()));
    if let Some(address) = current_address {
        return Ok(address);
    } else {
        let untweaked_pubkey = request_schnorr_key("key_1", rune_id.to_bytes()).await?;
        let tweaked_pubkey = tweak_pubkey_with_empty(untweaked_pubkey.clone());
        // cfg_if::cfg_if! {
            // if #[cfg(feature = "testnet")] {
                let address = Address::p2tr_tweaked(tweaked_pubkey, Network::Testnet4);
            // } else {
                // let address = Address::p2tr_tweaked(tweaked_pubkey, Network::Bitcoin);
            // }
        // }
        mutate_state(|s| {
            s.key = Some(untweaked_pubkey.clone());
            s.address = Some(address.to_string());
        });
        Ok(address.to_string())
    }
}

#[query]
fn get_rune_deposit_address() -> Option<String> {
    read_state(|s| s.address.clone())
}

#[query]
fn get_register_info() -> RegisterInfo {
    let (key, address, register_fee, last_state_res) = read_state(|s| (s.key.clone(), s.address.clone(), s.game.gamer_register_fee,s.last_state()));
    let last_state = last_state_res.unwrap();
    let tweaked_key = tweak_pubkey_with_empty(key.clone().unwrap());
    // tweaked_key.to
    RegisterInfo { 
        untweaked_key: key.unwrap(),
        address: address.unwrap(), 
        utxo: last_state.utxo.clone(), 
        register_fee,
        tweaked_key: Pubkey::from_str(&tweaked_key.to_string()).unwrap(),
        nonce: last_state.nonce
    }

}

#[update]
pub async fn deposit(utxo_for_rune: Utxo, utxo_for_btc: Utxo) -> Result<(), ExchangeError> {
    let rune_balance = utxo_for_rune
        .maybe_rune
        .clone()
        .ok_or(ExchangeError::CustomError(
            "rune info not found".to_string(),
        ))?;
    assert_ne!(rune_balance.id, CoinId::btc(), "Can't use btc init");

    mutate_state(|es| {
        if es.game.max_cookies != rune_balance.value {
            return Err(ExchangeError::DepositRuneBalanceIncorrect(
                es.game.max_cookies.to_string(),
                rune_balance.value.to_string(),
            ));
        }

        es.game.game_start_time = ic_cdk::api::time();
        let rune_amount = rune_balance.value;
        es.states.push(PoolState {
            id: None,
            nonce: 0,
            utxo: utxo_for_btc,
            rune_utxo: utxo_for_rune,
            rune_balance: rune_amount,
            user_action: crate::state::UserAction::Init,
        });
        Ok(())
    })?;

    Ok(())
}

#[update]
pub fn claim()->Result<u128, ExchangeError>{

    let principal = ic_cdk::caller();

    let address = read_state(
        |s| s.address_principal_map.get(&principal).ok_or(
            ExchangeError::GamerNotFound(principal.to_text().clone())
        )
    )?;

    mutate_state(
        |s| {
            s.game.claim(address)
        }
    )
}

/// REE API

#[query]
fn get_minimal_tx_value(_args: GetMinimalTxValueArgs) -> GetMinimalTxValueResponse {
    MIN_BTC_VALUE
}

#[query]
pub fn get_pool_states()->Vec<PoolState> {
    read_state(|s| {
        s.states.clone()
    })
}

#[query]
pub fn get_pool_info(args: GetPoolInfoArgs) -> GetPoolInfoResponse {
    let pool_address = args.pool_address;

    read_state(|es| match es.last_state() {
        Ok(last_state) => pool_address
            .eq(&es.address.clone().unwrap())
            .then_some(PoolInfo {
                key: es.key.clone().unwrap(),
                key_derivation_path: vec![es.rune_id.clone().to_bytes()],
                name: es.symbol.clone(),
                address: es.address.clone().unwrap(),
                nonce: last_state.nonce,
                coin_reserved: vec![CoinBalance {
                    id: es.rune_id.clone(),
                    value: last_state.rune_balance,
                }],
                btc_reserved: last_state.btc_balance(),
                utxos: vec![last_state.utxo.clone(), last_state.rune_utxo.clone()],
                attributes: "".to_string(),
            }),
        Err(_) => {
            return None;
        }
    })
}

#[query]
pub fn get_game_and_gamer_infos(gamer_id: crate::Address) -> GameAndGamer {

    read_state(|s| {
        GameAndGamer { 
            game_duration: s.game.game_duration, 
            game_start_time: s.game.game_start_time, 
            gamer_register_fee: s.game.gamer_register_fee, 
            claim_cooling_down: s.game.claim_cooling_down, 
            cookie_amount_per_claim: s.game.cookie_amount_per_claim, 
            max_cookies: s.game.max_cookies, 
            claimed_cookies: s.game.claimed_cookies, 
            gamer: s.game.gamer.get(&gamer_id) 
        }
    }) 
}

#[query]
pub fn get_pool_list(args: GetPoolListArgs) -> GetPoolListResponse {
    let (key, address) = read_state(|s| (s.key.clone().unwrap(), s.address.clone().unwrap()));
    if args.limit == 0 && args.from.is_some_and(|e| e.ne(&key)) {
        return vec![];
    } else {
        get_pool_info(GetPoolInfoArgs {
            pool_address: address,
        })
        .map(|e| vec![e])
        .unwrap_or(vec![])
    }
}

#[update(guard = "ensure_orchestrator")]
pub async fn execute_tx(args: ExecuteTxArgs) -> ExecuteTxResponse {
    let ExecuteTxArgs {
        psbt_hex,
        txid,
        intention_set,
        intention_index,
        zero_confirmed_tx_queue_length: _zero_confirmed_tx_queue_length,
    } = args;
    let raw = hex::decode(&psbt_hex).map_err(|_| "invalid psbt".to_string())?;
    let mut psbt = Psbt::deserialize(raw.as_slice()).map_err(|_| "invalid psbt".to_string())?;
    let intention = intention_set.intentions[intention_index as usize].clone();
    let initiator = intention_set.initiator_address.clone();
    let Intention {
        exchange_id: _,
        action,
        action_params: _,
        pool_address,
        nonce,
        pool_utxo_spend,
        pool_utxo_receive,
        input_coins,
        output_coins,
    } = intention;

    read_state(|s| {
        return s
            .address
            .clone()
            .ok_or("Exchange address not init".to_string())
            .and_then(|address| {
                address
                    .eq(&pool_address)
                    .then(|| ())
                    .ok_or("address not match".to_string())
            });
    })?;

    match action.as_str() {
        "register" => {
            let (new_state, consumed) = read_state(|es| {
                es.validate_register(
                    txid.clone(),
                    nonce,
                    pool_utxo_spend,
                    pool_utxo_receive,
                    input_coins,
                    output_coins,
                    initiator.clone(),
                )
            })
            .map_err(|e| e.to_string())?;
            let rune_id = read_state(|s| s.rune_id.clone());
            crate::psbt::sign(&mut psbt, &consumed, rune_id.to_bytes())
                .await
                .map_err(|e| e.to_string())?;

            let principal_from_ii = get_principal(initiator.clone()).await.map_err(
                |e| format!("get_principal failed: {:?}", e)
            )?.0;

            mutate_state(|s| {
                s.game.register_new_gamer(initiator.clone());
                s.commit(new_state);
                s.address_principal_map.insert(principal_from_ii, initiator.clone());
            });

        
        }
        // "withdraw" => {
        //     let (new_state, consumed) = read_state(|es| {
        //         es.validate_withdraw(
        //             txid.clone(),
        //             nonce,
        //             pool_utxo_spend,
        //             pool_utxo_receive,
        //             input_coins,
        //             output_coins,
        //             initiator.clone(),
        //         )
        //     })
        //     .map_err(|e| e.to_string())?;
        //     let rune_id = read_state(|s| s.rune_id.clone());
        //     crate::psbt::sign(&mut psbt, &consumed, rune_id.to_bytes())
        //         .await
        //         .map_err(|e| e.to_string())?;
        //     mutate_state(|s| {
        //         s.commit(new_state);
        //         s.game.withdraw(initiator.clone())
        //     })
        //     .map_err(|e| e.to_string())?;
        // }
        _ => {
            return Err("invalid method".to_string());
        }
    }

    Ok(psbt.serialize_hex())
}

/// REE API
#[update(guard = "ensure_orchestrator")]
pub fn finalize_tx(args: FinalizeTxArgs) -> FinalizeTxResponse {
    read_state(|s| 
        s.key.clone().ok_or("key not init".to_string())?
        .eq(&args.pool_key)
        .then(|| ())
        .ok_or("key not match".to_string())
    )?;

    mutate_state(|es| es.finalize(args.txid)).map_err(|e| e.to_string())
}

/// REE API
#[update(guard = "ensure_orchestrator")]
pub fn rollback_tx(args: RollbackTxArgs) -> RollbackTxResponse {
    read_state(|s| 
        s.key.clone().ok_or("key not init".to_string())?
        .eq(&args.pool_key)
        .then(|| ())
        .ok_or("key not match".to_string())
    )?;

    mutate_state(|es| es.rollback(args.txid)).map_err(|e| e.to_string())
    
}

fn ensure_orchestrator() -> std::result::Result<(), String> {
    read_state(|s| {
        s.orchestrator
            .eq(&ic_cdk::caller())
            .then(|| ())
            .ok_or("Access denied".to_string())
    })
}

#[post_upgrade]
fn post_upgrade() {

    // mutate_state(|s| {
    //     s.game.game_start_time = 1742412833;
    //     s.game.game_duration = 60 * 60 * 24 * 50;
    //     s.game.claim_cooling_down = 60 * 60 * 1;
    // });
   
    log!(
        INFO,
        "Finish Upgrade current version: {}",
        env!("CARGO_PKG_VERSION")
    );
}


// Enable Candid export
ic_cdk::export_candid!();

