/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, AccountId, env};

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    message: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{message: DEFAULT_MESSAGE.to_string()}
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, message: String) {
        // Use env::log to record logs permanently to the blockchain!
        log!("Saving greeting {}", message);
        self.message = message;
    }

    pub fn get_predecessor_account_id(&mut self) -> AccountId {
        let id = env::predecessor_account_id();
        log!("get_predecessor_account_id= {}", id);
        id
    }

    pub fn get_signer_account_id(&mut self) -> AccountId {
        let id = env::signer_account_id();
        log!("get_signer_account_id= {}", id);
        id
    }

    pub fn get_current_account_id(&mut self) -> AccountId {
        let id = env::current_account_id();
        log!("get_current_account_id= {}", id);
        id
    }

    #[payable]
    pub fn get_attached_deposit(&mut self) -> u128 {
        let deposit = env::attached_deposit();
        log!("get_attached_deposit= {}", deposit);
        deposit
    }

    pub fn get_account_balance(&mut self) -> u128 {
        let balance = env::account_balance();
        log!("get_account_balance= {}", balance);
        balance
    }

}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }
}
