use ic_cdk::export::Principal;
use ic_cdk::test;
use ic_cdk_macros::update;
use std::collections::HashMap;
use std::cell::RefCell;
use super::*; 

#[test]
fn test_transfer() {
    let mut token = Token::new("ICP".to_string(), "Internet Computer Token".to_string(), 1000);
    let sender = Principal::anonymous(); // The principal calling the update function
    let receiver = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(); // Replace with a valid principal

    
    token.balances.insert(sender.clone(), 500);

    
    let result = token.transfer(receiver.clone(), 100);

    // Assertions
    assert_eq!(result, Ok(()));
    assert_eq!(token.balance_of(sender), 400);
    assert_eq!(token.balance_of(receiver), 100);
}

#[test]
fn test_receive_tokens() {
    // Initialize a new token contract
    let mut token = Token::new("ICP".to_string(), "Internet Computer Token".to_string(), 1000);
    let sender = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(); // Replace with a valid principal
    let receiver = Principal::anonymous(); // The principal calling the update function

    // Set up initial balance
    token.balances.insert(sender.clone(), 500);

    // Receive tokens
    let result = token.receive_tokens(sender.clone(), 200);

    // Assertions
    assert_eq!(result, Ok(()));
    assert_eq!(token.balance_of(sender), 300);
    assert_eq!(token.balance_of(receiver), 200);
}

#[test]
fn test_balance_of() {
    // Initialize a new token contract
    let mut token = Token::new("ICP".to_string(), "Internet Computer Token".to_string(), 1000);
    let principal = Principal::anonymous(); // The principal calling the update function

    // Set up initial balance
    token.balances.insert(principal.clone(), 100);

    // Query balance
    let balance = token.balance_of(principal.clone());

    // Assertion
    assert_eq!(balance, 100);
}

#[test]
fn test_display_balances() {
    // Initialize a new token contract
    let mut token = Token::new("ICP".to_string(), "Internet Computer Token".to_string(), 1000);
    let principal1 = Principal::anonymous();
    let principal2 = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(); // Replace with a valid principal

    // Set up initial balances
    token.balances.insert(principal1.clone(), 100);
    token.balances.insert(principal2.clone(), 200);

    // Query balances
    let balances = token.display_balances();

    // Assertions
    let expected_balances = vec![
        (principal1, 100),
        (principal2, 200),
    ];
    assert_eq!(balances, expected_balances);
}
