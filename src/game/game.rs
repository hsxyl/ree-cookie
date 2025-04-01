use crate::*;
use crate::{
    memory::Memory, utils::get_chain_second_timestamp, Address, ExchangeError, SecondTimestamp,
    Seconds,
};
use ic_cdk::api::management_canister::bitcoin::Satoshi;
use ic_stable_structures::StableBTreeMap;
use serde::{Deserialize, Serialize};

use super::gamer::Gamer;

#[derive(Deserialize, Serialize)]
pub struct Game {
    pub game_duration: Seconds,
    pub game_start_time: SecondTimestamp,
    pub gamer_register_fee: Satoshi,
    pub claim_cooling_down: Seconds,
    pub cookie_amount_per_claim: u128,
    pub max_cookies: u128,
    pub claimed_cookies: u128,
    #[serde(skip, default = "crate::memory::init_gamer")]
    pub gamer: StableBTreeMap<Address, Gamer, Memory>,
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Self { 
            game_duration: self.game_duration.clone(), 
            game_start_time: self.game_start_time.clone(), 
            gamer_register_fee: self.gamer_register_fee.clone(), 
            claim_cooling_down: self.claim_cooling_down.clone(), 
            cookie_amount_per_claim: self.cookie_amount_per_claim.clone(), 
            max_cookies: self.max_cookies.clone(), 
            claimed_cookies: self.claimed_cookies.clone(), 
            gamer: crate::memory::init_gamer()
         }
    }
}

impl Game {
    pub fn init(
        game_duration: Seconds,
        gamer_register_fee: Satoshi,
        claim_cooling_down: Seconds,
        claimed_cookies_per_click: u128,
        max_cookies: u128,
    ) -> Self {
        Self {
            game_duration,
            gamer_register_fee,
            game_start_time: u64::MAX,
            claim_cooling_down,
            gamer: crate::memory::init_gamer(),
            cookie_amount_per_claim: claimed_cookies_per_click,
            max_cookies,
            claimed_cookies: 0,
        }
    }

    pub fn register_new_gamer(&mut self, gamer_id: Address) {
        self.gamer.insert(gamer_id.clone(), Gamer::new(gamer_id));
    }

    pub fn is_end(&self) -> bool {
        ic_cdk::api::time() > self.game_start_time + self.game_duration
    }

    pub fn is_start(&self) -> bool {
        ic_cdk::api::time() > self.game_start_time
    }

    pub fn able_claim(&self, gamer_id: Address) -> Result<()> {
        let remind_cookies = self.max_cookies - self.claimed_cookies;
        if remind_cookies < self.cookie_amount_per_claim {
            return Err(ExchangeError::CookieBalanceInsufficient(remind_cookies));
        }
        self.gamer
            .get(&gamer_id)
            .ok_or(ExchangeError::GamerNotFound(gamer_id.clone()))
            .and_then(|gamer| {
                if get_chain_second_timestamp() > gamer.last_click_time + self.claim_cooling_down {
                    Ok(())
                } else {
                    Err(ExchangeError::GamerCoolingDown(
                        gamer_id,
                        gamer.last_click_time + self.claim_cooling_down,
                    ))
                }
            })
    }

    pub fn claim(&mut self, gamer_id: Address) -> Result<u128> {
        self.able_claim(gamer_id.clone())?;

        let mut gamer = self
            .gamer
            .get(&gamer_id)
            .ok_or(ExchangeError::GamerNotFound(gamer_id.clone()))?;

        self.claimed_cookies
            .checked_add(self.cookie_amount_per_claim)
            .ok_or(ExchangeError::Overflow)?;
        gamer.claim(self.cookie_amount_per_claim)?;

        let new_cookies_balance = gamer.cookies;    
        self.gamer.insert(gamer_id, gamer);

        Ok(new_cookies_balance)

    }

    pub fn withdraw(&mut self, gamer_id: Address) -> Result<u128> {
        let mut gamer = self
            .gamer
            .get(&gamer_id)
            .ok_or(ExchangeError::GamerNotFound(gamer_id.clone()))?;

        if self.is_end() {
            if !gamer.is_withdrawn {
                gamer.is_withdrawn = true;
                let cookies = gamer.cookies;
                self.gamer.insert(gamer_id, gamer);
                Ok(cookies)
            } else {
                Err(ExchangeError::GamerWithdrawRepeatedly(gamer_id))
            }
        } else {
            Err(ExchangeError::GameNotEnd)
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct GameAndGamer {
    pub game_duration: Seconds,
    pub game_start_time: SecondTimestamp,
    pub gamer_register_fee: Satoshi,
    pub claim_cooling_down: Seconds,
    pub cookie_amount_per_claim: u128,
    pub max_cookies: u128,
    pub claimed_cookies: u128,
    pub gamer: Option<Gamer>,
}