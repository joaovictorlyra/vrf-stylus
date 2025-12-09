//! Minimal Lottery Contract with Chainlink VRF

#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use] extern crate alloc;
use alloc::vec::Vec;

use stylus_sdk::{
    alloy_primitives::{Address, Bytes, U16, U256, U32},
    alloy_sol_types::sol,
    prelude::*,
    stylus_core::calls::context::Call,
};

#[allow(deprecated)]
use stylus_sdk::call::Call as OldCall;

sol_storage! {
    #[entrypoint]
    pub struct SimpleLottery {
        address vrf_wrapper;
        address owner;
        uint256 entry_fee;
        address[] players;
        uint256 prize_pool;
        bool is_open;
        address last_winner;
    }
}

sol_interface! {
    interface IVRFV2PlusWrapper {
        function requestRandomWordsInNative(
            uint32 _callback_gas_limit,
            uint16 _request_confirmations,
            uint32 _num_words,
            bytes calldata extra_args
        ) external payable returns (uint256 requestId);
    }
}

fn get_extra_args() -> Bytes {
    Bytes::from(vec![0u8])
}

#[public]
impl SimpleLottery {
    pub fn init(&mut self, vrf: Address, fee: U256, owner: Address) {
        self.vrf_wrapper.set(vrf);
        self.entry_fee.set(fee);
        self.owner.set(owner);
        self.is_open.set(true);
    }

    #[payable]
    pub fn enter(&mut self) -> Result<(), Vec<u8>> {
        if !self.is_open.get() {
            return Err(vec![1u8]);
        }
        
        let value = self.vm().msg_value();
        if value < self.entry_fee.get() {
            return Err(vec![2u8]);
        }
        
        let player = self.vm().msg_sender();
        self.players.push(player);
        
        let pool = self.prize_pool.get();
        self.prize_pool.set(pool + value);
        
        Ok(())
    }

    pub fn draw(&mut self) -> Result<U256, Vec<u8>> {
        if self.vm().msg_sender() != self.owner.get() {
            return Err(vec![3u8]);
        }
        
        if self.players.len() == 0 {
            return Err(vec![4u8]);
        }
        
        self.is_open.set(false);
        
        let wrapper = IVRFV2PlusWrapper::new(self.vrf_wrapper.get());
        let extra = get_extra_args();
        
        let config = OldCall::new_in(self).value(U256::from(100000000000000u64));
        
        let req_id = wrapper.request_random_words_in_native(config, 200000, 3, 1, extra)?;
        
        Ok(req_id)
    }

    pub fn raw_fulfill_random_words(
        &mut self,
        _request_id: U256,
        random_words: Vec<U256>,
    ) -> Result<(), Vec<u8>> {
        let sender = self.vm().msg_sender();
        let wrapper = self.vrf_wrapper.get();
        
        if sender != wrapper {
            return Err(vec![5u8]);
        }
        
        let count = self.players.len();
        if count == 0 {
            return Ok(());
        }
        
        let random = random_words[0];
        let idx: usize = (random % U256::from(count)).try_into().unwrap_or(0);
        let winner = self.players.get(idx).unwrap();
        let prize = self.prize_pool.get();
        
        self.last_winner.set(winner);
        
        self.vm()
            .call(&Call::new().value(prize), winner, &[])
            .map_err(|_| vec![6u8])?;
        
        // Reset
        while self.players.len() > 0 {
            self.players.pop();
        }
        self.prize_pool.set(U256::ZERO);
        self.is_open.set(true);
        
        Ok(())
    }

    pub fn get_players_count(&self) -> U256 {
        U256::from(self.players.len())
    }

    pub fn get_prize_pool(&self) -> U256 {
        self.prize_pool.get()
    }

    pub fn get_last_winner(&self) -> Address {
        self.last_winner.get()
    }

    pub fn get_entry_fee(&self) -> U256 {
        self.entry_fee.get()
    }

    pub fn is_lottery_open(&self) -> bool {
        self.is_open.get()
    }

    pub fn get_owner(&self) -> Address {
        self.owner.get()
    }
}
