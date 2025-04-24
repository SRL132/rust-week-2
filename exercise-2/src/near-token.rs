

/*SUMMARY:
This is a simple contract for a near token. It allows users to send messages to the contract.
The messages are stored in a vector.
The contract has a function to add a message to the vector.
The contract has a function to get the messages from the vector.
The contract has a function to get the total number of messages in the vector.
*/

/*TODO:
An in-depth analysis of the contract. Comments should be added to the code snippet to explain the concepts shown in Lecture of Week 2.
*/

//Import near_sdk modules
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U64;
use near_sdk::serde::Serialize;
use near_sdk::store::Vector;
use near_sdk::{env, near_bindgen, AccountId, NearToken};

//Define the constant POINT_ONE
const POINT_ONE: NearToken = NearToken::from_millinear(100); //100 millinear is 0.1 NEAR

//Define the PostedMessage struct
#[derive(BorshDeserialize, BorshSerialize, Serialize)] //Derive the necessary traits for the struct
#[serde(crate = "near_sdk::serde")] //serde is a serailization and deserialization library for Rust
#[borsh(crate = "near_sdk::borsh")] //borsh is a serialization library for Rust
pub struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near_bindgen] //Creates the necessary code to interact with the NEAR blockchain
#[derive(BorshDeserialize, BorshSerialize)] //Derive the necessary traits for the struct
#[borsh(crate = "near_sdk::borsh")] //borsh is a serialization library for Rust
pub struct Contract {
    messages: Vector<PostedMessage>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            messages: Vector::new(b"m"), //creates a new NEAR-specific vector (not to be confused with Rust's standard Vec) that's used for persistent storage on the NEAR blockchain

            //The b"m" parameter:
            //b"m" is a byte string literal. It's used to create a byte slice (a sequence of bytes) that represents the string "m".
            //In this case, it's used to create a byte slice that represents the string "m", which is the name of the vector. 
        }
    }
}

#[near_bindgen] //Creates the necessary code to interact with the NEAR blockchain
impl Contract {
    #[payable] //This attribute indicates that the function can receive NEAR tokens as attached deposits
    pub fn add_message(&mut self, text: String) {
        let premium = env::attached_deposit() >= POINT_ONE; //Checks if the attached deposit is greater than or equal to POINT_ONE
        let sender = env::predecessor_account_id(); //Gets the account ID of the caller

        let message = PostedMessage {
            premium,
            sender,
            text,
        };

        self.messages.push(message); //Adds the message to the vector
    }

    pub fn get_messages(&self, from_index: Option<U64>, limit: Option<U64>) -> Vec<&PostedMessage> {
        let from = u64::from(from_index.unwrap_or(U64(0)));
        let limit = u64::from(limit.unwrap_or(U64(10)));

        self.messages
        .iter()           // Creates an iterator over the messages
        .skip(from as usize)  // Skips the first 'from' number of messages
        .take(limit as usize) // Takes only 'limit' number of messages
        .collect() 
    }

    pub fn total_messages(&self) -> u32 {
        self.messages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_message() {
        let mut contract = Contract::default();
        contract.add_message("A message".to_string());

        let posted_message = &contract.get_messages(None, None)[0];
        assert_eq!(posted_message.premium, false);
        assert_eq!(posted_message.text, "A message".to_string());
    }

    #[test]
    fn iters_messages() {
        let mut contract = Contract::default();
        contract.add_message("1st message".to_string());
        contract.add_message("2nd message".to_string());
        contract.add_message("3rd message".to_string());

        let total = &contract.total_messages();
        assert!(*total == 3);

        let last_message = &contract.get_messages(Some(U64::from(1)), Some(U64::from(2)))[1];
        assert_eq!(last_message.premium, false);
        assert_eq!(last_message.text, "3rd message".to_string());
    }
}