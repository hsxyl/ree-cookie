use ic_cdk::api::management_canister::bitcoin::Satoshi;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{StableBTreeMap, Storable};
use ree_types::{CoinBalance, CoinId, InputCoin, OutputCoin};
use std::borrow::Cow;

use crate::game::game::Game;
use crate::memory::{mutate_state, Memory};
use crate::*;

#[derive(Deserialize, Serialize)]
pub struct ExchangeState {
    pub rune_id: CoinId,
    pub symbol: String,
    pub key: Option<Pubkey>,
    pub address: Option<String>,
    pub game: Game,
    pub orchestrator: Principal,
    pub states: Vec<PoolState>,
    pub ii_canister: Principal,
    #[serde(skip, default = "crate::memory::init_address_principal_map")]
    pub address_principal_map: StableBTreeMap<Principal, Address, Memory>,
}

impl Clone for ExchangeState {
    fn clone(&self) -> Self {
        Self {
            rune_id: self.rune_id.clone(),
            symbol: self.symbol.clone(),
            key: self.key.clone(),
            address: self.address.clone(),
            game: self.game.clone(),
            orchestrator: self.orchestrator.clone(),
            states: self.states.clone(),
            ii_canister: self.ii_canister.clone(),
            address_principal_map: crate::memory::init_address_principal_map(),
        }
    }
}

impl Storable for ExchangeState {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(bincode::serialize(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        bincode::deserialize(bytes.as_ref()).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl ExchangeState {
    pub fn init(
        rune_id: CoinId,
        symbol: String,
        game_duration: Seconds,
        gamer_register_fee: Satoshi,
        claim_cooling_down: Seconds,
        claimed_cookies_per_click: u128,
        max_cookies: u128,
        orchestrator: Principal,
        ii_canister: Principal,
    ) -> Self {
        Self {
            rune_id,
            symbol,
            key: None,
            address: None,
            game: Game::init(
                game_duration,
                gamer_register_fee,
                claim_cooling_down,
                claimed_cookies_per_click,
                max_cookies,
            ),
            orchestrator,
            states: vec![],
            ii_canister,
            address_principal_map: crate::memory::init_address_principal_map()
        }
    }

    pub fn last_state(&self) -> Result<PoolState> {
        // The last state should always exist
        self.states
            .last()
            .cloned()
            .ok_or(ExchangeError::LastStateNotFound)
            .inspect_err(|e| log!(ERROR, "{}", e))
    }

    pub fn validate_register(
        &self,
        txid: Txid,
        nonce: u64,
        pool_utxo_spend: Vec<String>,
        pool_utxo_receive: Vec<String>,
        input_coins: Vec<InputCoin>,
        output_coins: Vec<OutputCoin>,
        address: Address,
    ) -> Result<(PoolState, Utxo)> {
        if self.game.gamer.contains_key(&address) {
            return Err(ExchangeError::GamerAlreadyExist(address.clone()));
        }

        // check input and output coin
        let pool_expected_receive_btc = CoinBalance {
            id: CoinId::btc(),
            value: self.game.gamer_register_fee as u128,
        };

        // the input coins should be only one and the value should be equal to the register fee
        (input_coins.len() == 1
            && output_coins.is_empty()
            && input_coins[0].coin.id.eq(&CoinId::btc())
            && input_coins[0].coin.value > self.game.gamer_register_fee as u128
        )
        .then(|| ())
        .ok_or(ExchangeError::InvalidSignPsbtArgs(format!(
            "input_coins: {:?}, output_coins: {:?}",
            input_coins, output_coins
        )))?;

        // the pool_utxo_spend should be equal to the utxo of the last state
        let last_state = self.last_state()?;

        // check nonce matches
        (last_state.nonce == nonce)
            .then(|| ())
            .ok_or(ExchangeError::PoolStateExpired(last_state.nonce))?;

        last_state
            .utxo
            .outpoint()
            .eq(pool_utxo_spend
                .last()
                .ok_or(ExchangeError::InvalidSignPsbtArgs(
                    "pool_utxo_spend is empty".to_string(),
                ))?)
            .then(|| ())
            .ok_or(ExchangeError::InvalidSignPsbtArgs(format!(
                "pool_utxo_spend: {:?}, last_state_utxo: {:?}",
                pool_utxo_spend, last_state.utxo
            )))?;

        // the pool_utxo_receive should exist
        let pool_new_outpoint = pool_utxo_receive.last().map(|s| s.clone()).ok_or(
            ExchangeError::InvalidSignPsbtArgs("pool_utxo_receive not found".to_string()),
        )?;

        let new_utxo = Utxo::try_from(
            pool_new_outpoint,
            None,
            last_state
                .utxo
                .sats
                .checked_add(self.game.gamer_register_fee)
                .ok_or(ExchangeError::Overflow)?,
        )
        .map_err(|e| ExchangeError::InvalidSignPsbtArgs(e.to_string()))?;
        let new_state = PoolState {
            id: Some(txid),
            nonce: last_state
                .nonce
                .checked_add(1)
                .ok_or(ExchangeError::Overflow)?,
            utxo: new_utxo,
            rune_utxo: last_state.rune_utxo,
            rune_balance: last_state.rune_balance,
            user_action: UserAction::Register(address.clone()),
        };

        Ok((new_state, last_state.utxo.clone()))
    }

    // pub fn validate_withdraw(
    //     &self,
    //     _txid: Txid,
    //     _nonce: u64,
    //     _pool_utxo_spend: Vec<String>,
    //     _pool_utxo_receive: Vec<String>,
    //     input_coins: Vec<InputCoin>,
    //     output_coins: Vec<OutputCoin>,
    //     _address: Address,
    // ) -> Result<(PoolState, Utxo)> {
    //     // check input and output coin
    //     (input_coins.len() == 0 && output_coins.len() == 2)
    //         .then(|| ())
    //         .ok_or(ExchangeError::InvalidSignPsbtArgs(format!(
    //             "input_coins: {:?}, output_coins: {:?}",
    //             input_coins, output_coins
    //         )))?;

    //     // let expect_transfer_to_

    //     let output_btc = output_coins
    //         .iter()
    //         .find(|c| c.coin.id == CoinId::btc())
    //         .ok_or(ExchangeError::InvalidSignPsbtArgs(
    //             "output_btc not found".to_string(),
    //         ))?;
    //     let output_rune = output_coins
    //         .iter()
    //         .find(|c| c.coin.id == self.rune_id)
    //         .ok_or(ExchangeError::InvalidSignPsbtArgs(
    //             "output_rune not found".to_string(),
    //         ))?;
    //     // output_btc.to.eq()

    //     todo!()
    // }

    pub(crate) fn commit(&mut self, state: PoolState) {
        self.states.push(state);
    }

    pub(crate) fn finalize(&mut self, txid: Txid) -> Result<()> {
        let idx = self
            .states
            .iter()
            .position(|s| s.id == Some(txid))
            .ok_or(ExchangeError::InvalidState("txid not found".to_string()))?;

        if idx == 0 {
            return Ok(());
        }

        self.states.rotate_left(idx);
        self.states.truncate(self.states.len() - idx);

        Ok(())
    }

    pub(crate) fn rollback(&mut self, txid: Txid) -> Result<()> {
        let idx = self
            .states
            .iter()
            .position(|state| state.id == Some(txid))
            .ok_or(ExchangeError::InvalidState("txid not found".to_string()))?;
        if idx == 0 {
            // why impossible to rollback index 0 state?
            // In init case, the state is empty, so the first state pushed in deposit interface which needn't finalize or rollback
            // In other case, the finalize will always keep the last finalized state in vec, so the rollback should be impossible to rollback index 0 state 
            return Err(ExchangeError::InvalidState("Should not rollback index 0 state".to_string()));
        }

        while self.states.len() > idx {
            let state = self.states.pop().ok_or(
                ExchangeError::InvalidState("Should not rollback index 0 state".to_string()),
            )?;
            match state.user_action {
                UserAction::Init => {
                    // impossible to rollback init state
                    return Err(ExchangeError::InvalidState("Should not rollback init action".to_string()));
                },
                UserAction::Register(address) => {
                    mutate_state(|es| {
                        es.game.gamer.remove(&address);
                    });
                },
                UserAction::Withdraw(address) => {
                    mutate_state(|es| {
                        let mut gamer = es.game.gamer.get(&address).ok_or(ExchangeError::GamerNotFound(address.clone()))?;
                        gamer.is_withdrawn = false;
                        es.game.gamer.insert(address, gamer.clone());

                        Ok(())
                    })?;
                },
            }
        }

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PoolState {
    pub id: Option<Txid>,
    pub nonce: u64,
    pub utxo: Utxo,
    pub rune_utxo: Utxo,
    pub rune_balance: u128,
    pub user_action: UserAction,
}

impl PoolState {
    pub fn btc_balance(&self) -> u64 {
        self.utxo.sats
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum UserAction {
    Init,
    Register(Address),
    Withdraw(Address),
}

thread_local! {
  
}

// pub fn mutate_state<F, R>(f: F) -> R
// where
//     F: FnOnce(&mut ExchangeState) -> R,
// {
//     STATE.with(|s| f(s.borrow_mut().as_mut().expect("State not initialized!")))
// }

// pub fn read_state<F, R>(f: F) -> R
// where
//     F: FnOnce(&ExchangeState) -> R,
// {
//     STATE.with(|s| f(s.borrow().as_ref().expect("State not initialized!")))
// }

// pub fn replace_state(state: ExchangeState) {
//     STATE.with(|s| {
//         *s.borrow_mut() = Some(state);
//     });
// }
