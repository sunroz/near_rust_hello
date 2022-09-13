/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, AccountId, env, Gas, Promise};

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    message: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
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

    #[payable]
    pub fn set_greeting_with_deposit(&mut self, message: String) {

        let deposit = self.get_attached_deposit();
        assert!(deposit >= 5, "Deposit at least 5 Near");

        let initial_storage_used = self.get_storage_usage();

        // Use env::log to record logs permanently to the blockchain!
        log!("Saving greeting {}", message);
        self.message = message;

        self.get_greeting();

        self.pay_total_storage_cost(deposit, initial_storage_used);
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

    pub fn get_gas_info(&mut self) -> (Gas, Gas) {
        let prepaid_gas = env::prepaid_gas();
        let used_gas = env::used_gas();
        (prepaid_gas, used_gas)
    }

    pub fn get_storage_usage(&mut self) -> u64 {
        let storage_usage = env::storage_usage();
        log!("get_storage_usage= {}", storage_usage);
        storage_usage
    }

    pub fn get_storage_byte_cost(&mut self) -> u128 {
        let storage_byte_cost = env::storage_byte_cost();
        log!("storage_byte_cost= {}", storage_byte_cost);
        storage_byte_cost
    }

    pub fn get_total_storage_cost(&mut self) -> u128 {
        let storage = self.get_storage_usage();
        let byte_cost = self.get_storage_byte_cost();

        let total = u128::checked_mul(byte_cost, storage.into()).unwrap();
        log!("get_total_storage_cost= {}", total);
        total
    }

    #[payable]
    pub fn pay_total_storage_cost(&mut self, deposit: u128, initial_storage: u64) {
        let current_storage = env::storage_usage();
        let byte_cost = self.get_storage_byte_cost();
        let account_id = self.get_predecessor_account_id();

        log!("current_storage= {}", current_storage);

        let payable_storage = u64::checked_sub(current_storage, initial_storage).unwrap();
        log!("payable_storage= {}", payable_storage);

        let total_storage_cost = u128::checked_mul(byte_cost, payable_storage.into()).unwrap();
        log!("get_total_storage_cost= {}", total_storage_cost);

        assert!(deposit > total_storage_cost, "insufficient balance");

        let surplus = deposit - total_storage_cost;
        log!("surplus= {}", surplus);

        Promise::new(account_id).transfer(surplus);
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
