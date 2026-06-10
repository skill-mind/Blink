#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol, Address, Map};

#[contracttype]
pub enum DataKey {
    Payment(u64),
    Balance(Address),
}

#[contracttype]
pub struct Payment {
    pub id: u64,
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    pub asset: Address,
    pub timestamp: u64,
}

const PAYMENT_COUNT: Symbol = symbol_short!("PAY_CNT");

#[contract]
pub struct PaymentProcessor;

#[contractimpl]
impl PaymentProcessor {
    /// Initialize the contract
    pub fn initialize(env: Env) {
        env.storage().instance().set(&PAYMENT_COUNT, &0u64);
    }

    /// Process a payment between two addresses
    pub fn process_payment(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
        asset: Address,
    ) -> u64 {
        // Require authentication from the sender
        from.require_auth();

        // Get current payment count
        let payment_count: u64 = env.storage().instance()
            .get(&PAYMENT_COUNT)
            .unwrap_or(0);

        let payment_id = payment_count + 1;

        // Create payment record
        let payment = Payment {
            id: payment_id,
            from: from.clone(),
            to: to.clone(),
            amount,
            asset,
            timestamp: env.ledger().timestamp(),
        };

        // Store payment
        env.storage().persistent().set(&DataKey::Payment(payment_id), &payment);
        
        // Update payment count
        env.storage().instance().set(&PAYMENT_COUNT, &payment_id);

        payment_id
    }

    /// Get payment details by ID
    pub fn get_payment(env: Env, payment_id: u64) -> Option<Payment> {
        env.storage().persistent().get(&DataKey::Payment(payment_id))
    }

    /// Get total number of payments processed
    pub fn get_payment_count(env: Env) -> u64 {
        env.storage().instance().get(&PAYMENT_COUNT).unwrap_or(0)
    }
}

mod test;
