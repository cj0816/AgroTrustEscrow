#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, token};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum EscrowStatus {
    Created,
    Funded,
    Completed,
    Cancelled,
}

#[contracttype]
pub enum DataKey {
    Buyer,
    Farmer,
    Verifier,
    Token,
    Amount,
    Status,
}

#[contract]
pub struct AgroTrustContract;

#[contractimpl]
impl AgroTrustContract {
    /// Initializes the escrow terms between buyer, farmer, and verifier
    pub fn initialize(
        env: Env,
        buyer: Address,
        farmer: Address,
        verifier: Address,
        token: Address,
        amount: i128,
    ) {
        buyer.require_auth();
        assert!(!env.storage().instance().has(&DataKey::Status), "Already initialized");
        assert!(amount > 0, "Amount must be positive");

        env.storage().instance().set(&DataKey::Buyer, &buyer);
        env.storage().instance().set(&DataKey::Farmer, &farmer);
        env.storage().instance().set(&DataKey::Verifier, &verifier);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Amount, &amount);
        env.storage().instance().set(&DataKey::Status, &EscrowStatus::Created);
    }

    /// Deposits payment token from buyer into contract storage context
    pub fn fund_escrow(env: Env) {
        let buyer: Address = env.storage().instance().get(&DataKey::Buyer).unwrap();
        buyer.require_auth();

        let status: EscrowStatus = env.storage().instance().get(&DataKey::Status).unwrap();
        assert_eq!(status, EscrowStatus::Created, "Escrow not in Created state");

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let amount: i128 = env.storage().instance().get(&DataKey::Amount).unwrap();

        let client = token::Client::new(&env, &token_addr);
        client.transfer(&buyer, &env.current_contract_address(), &amount);

        env.storage().instance().set(&DataKey::Status, &EscrowStatus::Funded);
    }

    /// Executed by delivery verifier to confirm delivery and trigger token transfer to farmer
    pub fn release_payment(env: Env) {
        let verifier: Address = env.storage().instance().get(&DataKey::Verifier).unwrap();
        verifier.require_auth();

        let status: EscrowStatus = env.storage().instance().get(&DataKey::Status).unwrap();
        assert_eq!(status, EscrowStatus::Funded, "Escrow not funded");

        let farmer: Address = env.storage().instance().get(&DataKey::Farmer).unwrap();
        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let amount: i128 = env.storage().instance().get(&DataKey::Amount).unwrap();

        let client = token::Client::new(&env, &token_addr);
        client.transfer(&env.current_contract_address(), &farmer, &amount);

        env.storage().instance().set(&DataKey::Status, &EscrowStatus::Completed);
    }

    /// Fetches the current lifecycle state of the escrow
    pub fn get_status(env: Env) -> EscrowStatus {
        env.storage().instance().get(&DataKey::Status).unwrap()
    }
}