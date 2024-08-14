use ic_cdk::export::Principal;
use ic_cdk_macros::{query, update};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Token {
    symbol: String,
    name: String,
    total_supply: u64,
    balances: HashMap<Principal, u64>,
}

impl Token {
    
    fn new(symbol: String, name: String, total_supply: u64) -> Self {
        let mut balances = HashMap::new();
        
        let caller = ic_cdk::caller();
        balances.insert(caller, total_supply);
        Token {
            symbol,
            name,
            total_supply,
            balances,
        }
    }

    
    fn transfer(&mut self, to: Principal, amount: u64) -> Result<(), String> {
        let from = ic_cdk::caller();
        let from_balance = self.balances.entry(from).or_insert(0);

        if *from_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        
        *from_balance -= amount;

        
        let to_balance = self.balances.entry(to).or_insert(0);
        *to_balance += amount;

        Ok(())
    }

    
    fn receive_tokens(&mut self, from: Principal, amount: u64) -> Result<(), String> {
        let to = ic_cdk::caller();
        let from_balance = self.balances.entry(from).or_insert(0);

        if *from_balance < amount {
            return Err("Insufficient balance from sender".to_string());
        }

        
        *from_balance -= amount;

        
        let to_balance = self.balances.entry(to).or_insert(0);
        *to_balance += amount;

        Ok(())
    }

    
    fn balance_of(&self, owner: Principal) -> u64 {
        *self.balances.get(&owner).unwrap_or(&0)
    }

    
    fn display_balances(&self) -> Vec<(Principal, u64)> {
        self.balances.iter().map(|(&k, &v)| (k, v)).collect()
    }
}

thread_local! {
    static TOKEN: RefCell<Token> = RefCell::new(Token::new("ICP".to_string(), "Internet Computer Token".to_string(), 1000000));
}

#[update]
async fn transfer(to: Principal, amount: u64) -> Result<(), String> {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token.transfer(to, amount)
    })
}

#[update]
async fn receive_tokens(from: Principal, amount: u64) -> Result<(), String> {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token.receive_tokens(from, amount)
    })
}

#[query]
async fn balance_of(owner: Principal) -> u64 {
    TOKEN.with(|token| {
        let token = token.borrow();
        token.balance_of(owner)
    })
}

#[query]
async fn display_balances() -> Vec<(Principal, u64)> {
    TOKEN.with(|token| {
        let token = token.borrow();
        token.display_balances()
    })
}
