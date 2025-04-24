use near_sdk::store::LookupMap;
use near_sdk::{env, near, require, AccountId};

pub type Id = u8;

#[near(contract_state)]
pub struct Contract {
    pub tokens: LookupMap<Id, AccountId>,
    pub approvals: LookupMap<Id, AccountId>,
    pub supply: u16,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            tokens: {
                let mut a = LookupMap::new(b"tokens".to_vec());
                a.insert(0, "admin.near".parse().unwrap());
                a
            },
            approvals: LookupMap::new(b"approvals".to_vec()),
            supply: 1,
        }
    }
}

#[near]
impl Contract {
    #[init]
    #[private] // only callable by the contract's account
    pub fn init(
        admin: AccountId
    ) -> Self {
        Self {
            tokens: {
                let mut a = LookupMap::new(b"tokens".to_vec());
                a.insert(0, admin);
                a
            },
            approvals: LookupMap::new(b"approvals".to_vec()),
            supply: 1,
        }
    }

    pub fn owner_of(&self, id: Id) -> Option<AccountId> {
        self.tokens.get(&id).cloned()
    }
    //@audit-issue: the mint function is not protected by the admin account
    pub fn mint(&mut self) -> Id {
        self.tokens.insert(self.supply.to_le_bytes()[0], env::predecessor_account_id());
        let id = self.supply;
        self.supply += 1;
        id as Id
    }

    pub fn approve(&mut self, id: Id, delegatee: AccountId) {
        require!(self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id(), "not owner!");
        self.approvals.insert(id, delegatee);
    }
    //@audit-issue: approvals do not reset on transfer, so you can approve to yourself and transfer token back to yourself
    pub fn transfer(&mut self, id: Id, receiver: AccountId) {
        require!(
            self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id()
            || self.approvals.get(&id).unwrap().clone() == env::predecessor_account_id()
            , "not owner!"
        );
     //   self.approvals.remove(&id);
        self.tokens.insert(id, receiver);
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env};
    use super::*;

    #[test]
    fn exploit_todo() {
        let bob: AccountId = "bob.near".parse().unwrap();
        set_context(bob.clone());
        // init
        let admin: AccountId = "admin.near".parse().unwrap();
        let mut contract = Contract::init(admin.clone());
        assert_eq!(contract.owner_of(0).unwrap(), admin);
        
    }

    #[test]
    fn test_mint() {
        //Write a recommendation to fix the code!
        let admin: AccountId = "admin.near".parse().unwrap();
        let mut contract = Contract::init(admin.clone());
        assert_eq!(contract.owner_of(0).unwrap(), admin);

        let bob: AccountId = "bob.near".parse().unwrap();
        set_context(bob.clone());
        // init
        let admin: AccountId = "admin.near".parse().unwrap();
        let mut contract = Contract::init(admin.clone());
        assert_eq!(contract.owner_of(0).unwrap(), admin);
        contract.mint();
        assert_eq!(contract.owner_of(1).unwrap(), bob);
    }

    #[test]
    fn test_can_transfer_twice_if_approved() {
        let admin: AccountId = "admin.near".parse().unwrap();
        let mut contract = Contract::init(admin.clone());
        assert_eq!(contract.owner_of(0).unwrap(), admin);
        let bob: AccountId = "bob.near".parse().unwrap();
        set_context(bob.clone());
        contract.mint();
        contract.approve(1, bob.clone());
        contract.transfer(1, admin.clone());
        assert_eq!(contract.owner_of(1).unwrap(), admin.clone());
        contract.transfer(1, bob.clone());
        assert_eq!(contract.owner_of(1).unwrap(), bob.clone());
    }

    // Auxiliar fn: create a mock context
    fn set_context(predecessor: AccountId) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);

        testing_env!(builder.build());
    }

}