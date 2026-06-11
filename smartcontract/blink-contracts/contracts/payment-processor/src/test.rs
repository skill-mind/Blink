#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PaymentProcessor);
    let client = PaymentProcessorClient::new(&env, &contract_id);

    client.initialize();
    
    let count = client.get_payment_count();
    assert_eq!(count, 0);
}

#[test]
fn test_process_payment() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PaymentProcessor);
    let client = PaymentProcessorClient::new(&env, &contract_id);

    client.initialize();

    let from = Address::generate(&env);
    let to = Address::generate(&env);
    let asset = Address::generate(&env);
    let amount = 1000i128;

    env.mock_all_auths();

    let payment_id = client.process_payment(&from, &to, &amount, &asset);
    assert_eq!(payment_id, 1);

    let payment = client.get_payment(&payment_id);
    assert!(payment.is_some());
    
    let payment_details = payment.unwrap();
    assert_eq!(payment_details.from, from);
    assert_eq!(payment_details.to, to);
    assert_eq!(payment_details.amount, amount);
    assert_eq!(payment_details.asset, asset);

    let count = client.get_payment_count();
    assert_eq!(count, 1);
}

#[test]
fn test_multiple_payments() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PaymentProcessor);
    let client = PaymentProcessorClient::new(&env, &contract_id);

    client.initialize();

    let from = Address::generate(&env);
    let to = Address::generate(&env);
    let asset = Address::generate(&env);

    env.mock_all_auths();

    // Process first payment
    let payment_id_1 = client.process_payment(&from, &to, &100i128, &asset);
    assert_eq!(payment_id_1, 1);

    // Process second payment
    let payment_id_2 = client.process_payment(&from, &to, &200i128, &asset);
    assert_eq!(payment_id_2, 2);

    let count = client.get_payment_count();
    assert_eq!(count, 2);

    // Verify both payments exist
    let payment_1 = client.get_payment(&payment_id_1);
    let payment_2 = client.get_payment(&payment_id_2);

    assert!(payment_1.is_some());
    assert!(payment_2.is_some());

    assert_eq!(payment_1.unwrap().amount, 100i128);
    assert_eq!(payment_2.unwrap().amount, 200i128);
}
