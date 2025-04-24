use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::collections::{Vector, LookupMap};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Contract {
    // your contract state
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn new() -> Self {
        Self {
            // initialize your contract state
        }
    }

    // your contract methods
}

fn main() {
    println!("Hello, world!");
}
