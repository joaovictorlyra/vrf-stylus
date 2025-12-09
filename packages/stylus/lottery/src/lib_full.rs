//!
//! Chainlink VRF Lottery Contract in Stylus Rust
//!
//! A decentralized lottery system that uses Chainlink VRF for verifiable random winner selection.
//!

#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

use stylus_sdk::{
    alloy_primitives::{Address, Bytes, U16, U256, U32},
    alloy_sol_types::sol,
    prelude::*,
    stylus_core::calls::context::Call,
    stylus_core::log,
};

#[allow(deprecated)]
use stylus_sdk::call::Call as OldCall;

use openzeppelin_stylus::access::ownable::{self, Ownable};

// Define persistent storage
sol_storage! {
    #[entrypoint]
    pub struct Lottery {
        address i_vrf_v2_plus_wrapper;
        uint32 callback_gas_limit;
        uint16 request_confirmations;
        uint32 num_words;
        
        uint256 entry_fee;
        address[] players;
        uint256 current_prize_pool;
        bool lottery_open;
        
        mapping(uint256 => uint256) vrf_requests;
        
        address last_winner;
        uint256 last_prize;
        
        Ownable ownable;
    }
}

// VRF V2+ Wrapper interface
sol_interface! {
    interface IVRFV2PlusWrapper {
        function calculateRequestPriceNative(uint32 _callback_gas_limit, uint32 _num_words) external view returns (uint256);
        function requestRandomWordsInNative(
            uint32 _callback_gas_limit,
            uint16 _request_confirmations,
            uint32 _num_words,
            bytes calldata extra_args
        ) external payable returns (uint256 requestId);
    }
}

// Events
sol! {
    event LotteryEntered(address indexed player, uint256 indexed lotteryId, uint256 entryFee);
    event DrawStarted(uint256 indexed lotteryId, uint256 indexed requestId, uint256 playersCount);
    event WinnerSelected(uint256 indexed lotteryId, address indexed winner, uint256 prizeAmount, uint256 randomWord);
    event EntryFeeUpdated(uint256 oldFee, uint256 newFee);
    event VRFRequestSent(uint256 indexed requestId, uint32 numWords);
    event VRFRequestFulfilled(uint256 indexed requestId, uint256[] randomWords, uint256 payment);
    event Received(address indexed sender, uint256 value);
}

// Errors
sol! {
    #[derive(Debug)]
    error OnlyVRFWrapperCanFulfill(address have, address want);
    
    #[derive(Debug)]
    error LotteryNotOpen();
    
    #[derive(Debug)]
    error InsufficientEntryFee(uint256 sent, uint256 required);
    
    #[derive(Debug)]
    error NoPlayersInLottery();
    
    #[derive(Debug)]
    error LotteryAlreadyClosed();
    
    #[derive(Debug)]
    error TransferFailed();
}

#[derive(SolidityError, Debug)]
pub enum Error {
    OnlyVRFWrapperCanFulfill(OnlyVRFWrapperCanFulfill),
    LotteryNotOpen(LotteryNotOpen),
    InsufficientEntryFee(InsufficientEntryFee),
    NoPlayersInLottery(NoPlayersInLottery),
    LotteryAlreadyClosed(LotteryAlreadyClosed),
    TransferFailed(TransferFailed),
    UnauthorizedAccount(ownable::OwnableUnauthorizedAccount),
    InvalidOwner(ownable::OwnableInvalidOwner),
}

impl From<ownable::Error> for Error {
    fn from(value: ownable::Error) -> Self {
        match value {
            ownable::Error::UnauthorizedAccount(e) => Error::UnauthorizedAccount(e),
            ownable::Error::InvalidOwner(e) => Error::InvalidOwner(e),
        }
    }
}

#[public]
impl Lottery {
    /// Constructor - initializes the lottery contract
    #[constructor]
    pub fn constructor(
        &mut self,
        vrf_v2_plus_wrapper: Address,
        entry_fee: U256,
        owner: Address,
    ) -> Result<(), Error> {
        self.ownable.constructor(owner)?;
        self.i_vrf_v2_plus_wrapper.set(vrf_v2_plus_wrapper);
        self.entry_fee.set(entry_fee);
        self.current_prize_pool.set(U256::ZERO);
        self.lottery_open.set(true);
        
        self.callback_gas_limit.set(U32::from(200000));
        self.request_confirmations.set(U16::from(3));
        self.num_words.set(U32::from(1));
        
        Ok(())
    }

    /// Players can enter the lottery
    #[payable]
    pub fn enter_lottery(&mut self) -> Result<(), Error> {
        if !self.lottery_open.get() {
            return Err(Error::LotteryNotOpen(LotteryNotOpen {}));
        }
        
        let msg_value = self.vm().msg_value();
        let required_fee = self.entry_fee.get();
        
        if msg_value < required_fee {
            return Err(Error::InsufficientEntryFee(InsufficientEntryFee {
                sent: msg_value,
                required: required_fee,
            }));
        }
        
        let player = self.vm().msg_sender();
        
        self.players.push(player);
        
        // Add to prize pool
        let current_pool = self.current_prize_pool.get();
        self.current_prize_pool.set(current_pool + msg_value);
        
        log(
            self.vm(),
            LotteryEntered {
                player,
                lotteryId: U256::from(1),
                entryFee: msg_value,
            },
        );
        
        Ok(())
    }

    /// Owner starts the draw
    pub fn start_draw(&mut self) -> Result<U256, Error> {
        self.ownable.only_owner()?;
        
        if !self.lottery_open.get() {
            return Err(Error::LotteryAlreadyClosed(LotteryAlreadyClosed {}));
        }
        
        let players_count = self.players.len();
        if players_count == 0 {
            return Err(Error::NoPlayersInLottery(NoPlayersInLottery {}));
        }
        
        self.lottery_open.set(false);
        
        let callback_gas_limit = self.callback_gas_limit.get().try_into().unwrap_or(200000);
        let request_confirmations = self.request_confirmations.get().try_into().unwrap_or(3);
        let num_words = self.num_words.get().try_into().unwrap_or(1);
        
        let (request_id, req_price) = self.request_randomness_pay_in_native(
            callback_gas_limit,
            request_confirmations,
            num_words,
        )?;
        
        self.vrf_requests.insert(request_id, U256::from(1));
        
        log(
            self.vm(),
            DrawStarted {
                lotteryId: U256::from(1),
                requestId: request_id,
                playersCount: U256::from(players_count),
            },
        );
        
        log(
            self.vm(),
            VRFRequestSent {
                requestId: request_id,
                numWords: num_words,
            },
        );
        
        Ok(request_id)
    }

    /// Internal function to request randomness
    fn request_randomness_pay_in_native(
        &mut self,
        callback_gas_limit: u32,
        request_confirmations: u16,
        num_words: u32,
    ) -> Result<(U256, U256), Error> {
        let external_vrf_wrapper_address = self.i_vrf_v2_plus_wrapper.get();
        let external_vrf_wrapper = IVRFV2PlusWrapper::new(external_vrf_wrapper_address);

        // Calculate request price
        let request_price = external_vrf_wrapper
            .calculate_request_price_native(&mut *self, callback_gas_limit, num_words)
            .map_err(|_| Error::TransferFailed(TransferFailed {}))?;

        let extra_args = get_extra_args_for_native_payment();

        #[allow(deprecated)]
        let config = OldCall::new().value(request_price);

        // Request random words
        let request_id = external_vrf_wrapper
            .request_random_words_in_native(
                config,
                callback_gas_limit,
                request_confirmations,
                num_words,
                extra_args,
            )
            .map_err(|_| Error::TransferFailed(TransferFailed {}))?;

        Ok((request_id, request_price))
    }

    /// Callback called by VRF wrapper
    pub fn raw_fulfill_random_words(
        &mut self,
        request_id: U256,
        random_words: Vec<U256>,
    ) -> Result<(), Error> {
        let vrf_wrapper_addr = self.i_vrf_v2_plus_wrapper.get();
        let msg_sender = self.vm().msg_sender();
        
        if msg_sender != vrf_wrapper_addr {
            return Err(Error::OnlyVRFWrapperCanFulfill(OnlyVRFWrapperCanFulfill {
                have: msg_sender,
                want: vrf_wrapper_addr,
            }));
        }

        log(
            self.vm(),
            VRFRequestFulfilled {
                requestId: request_id,
                randomWords: random_words.clone(),
                payment: U256::ZERO,
            },
        );

        self.fulfill_random_words(request_id, random_words)
    }

    /// Internal function to select winner
    fn fulfill_random_words(
        &mut self,
        request_id: U256,
        random_words: Vec<U256>,
    ) -> Result<(), Error> {
        if random_words.is_empty() {
            return Ok(());
        }

        let random_word = random_words[0];
        let players_count = self.players.len();

        if players_count == 0 {
            return Ok(());
        }

        // Select winner
        let winner_index = (random_word % U256::from(players_count)).try_into().unwrap_or(0);
        let winner = self.players.get(winner_index).unwrap();

        // Get prize pool
        let prize_pool = self.current_prize_pool.get();

        // Store last winner info
        self.last_winner.set(winner);
        self.last_prize.set(prize_pool);

        // Transfer prize to winner
        self.vm()
            .call(&Call::new().value(prize_pool), winner, &[])
            .map_err(|_| Error::TransferFailed(TransferFailed {}))?;

        log(
            self.vm(),
            WinnerSelected {
                lotteryId: U256::from(1),
                winner,
                prizeAmount: prize_pool,
                randomWord: random_word,
            },
        );

        // Reset for next lottery
        while self.players.len() > 0 {
            self.players.pop();
        }
        self.current_prize_pool.set(U256::ZERO);
        self.lottery_open.set(true);

        Ok(())
    }

    /// Owner can update entry fee
    pub fn set_entry_fee(&mut self, new_fee: U256) -> Result<(), Error> {
        self.ownable.only_owner()?;
        let old_fee = self.entry_fee.get();
        self.entry_fee.set(new_fee);
        
        log(
            self.vm(),
            EntryFeeUpdated {
                oldFee: old_fee,
                newFee: new_fee,
            },
        );
        
        Ok(())
    }

    /// View functions
    pub fn get_entry_fee(&self) -> U256 {
        self.entry_fee.get()
    }

    pub fn get_last_winner(&self) -> Address {
        self.last_winner.get()
    }
    
    pub fn get_last_prize(&self) -> U256 {
        self.last_prize.get()
    }

    pub fn is_lottery_open(&self) -> bool {
        self.lottery_open.get()
    }

    pub fn get_players_count(&self) -> U256 {
        U256::from(self.players.len())
    }

    pub fn get_player(&self, index: U256) -> Address {
        let idx: usize = index.try_into().unwrap_or(0);
        self.players.get(idx).unwrap_or(Address::ZERO)
    }

    pub fn get_prize_pool(&self) -> U256 {
        self.current_prize_pool.get()
    }



    pub fn owner(&self) -> Address {
        self.ownable.owner()
    }

    pub fn i_vrf_v2_plus_wrapper(&self) -> Address {
        self.i_vrf_v2_plus_wrapper.get()
    }

    pub fn callback_gas_limit(&self) -> U32 {
        self.callback_gas_limit.get()
    }

    pub fn request_confirmations(&self) -> U16 {
        self.request_confirmations.get()
    }

    pub fn num_words(&self) -> U32 {
        self.num_words.get()
    }

    /// Withdraw native tokens (owner only)
    pub fn withdraw_native(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        self.ownable.only_owner()?;

        self.vm()
            .call(&Call::new().value(amount), self.ownable.owner(), &[])?;

        Ok(())
    }

    /// Receive function
    #[receive]
    #[payable]
    pub fn receive(&mut self) -> Result<(), Vec<u8>> {
        log(
            self.vm(),
            Received {
                sender: self.vm().msg_sender(),
                value: self.vm().msg_value(),
            },
        );
        Ok(())
    }

    /// Fund contract for VRF payments
    #[payable]
    pub fn fund_contract(&mut self) -> Result<(), Vec<u8>> {
        Ok(())
    }
}

fn get_extra_args_for_native_payment() -> Bytes {
    let mut extra_args_vec = Vec::new();
    extra_args_vec.extend_from_slice(&[0x92, 0xfd, 0x13, 0x38]);
    extra_args_vec.extend_from_slice(&[0x00; 28]);
    extra_args_vec.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
    extra_args_vec.extend_from_slice(&[0x00; 28]);
    Bytes::from(extra_args_vec)
}
