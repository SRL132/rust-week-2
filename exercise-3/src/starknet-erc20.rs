use starknet::ContractAddress;

/*SUMMARY:
This is a contract for an ERC20 token. It allows users to transfer tokens between accounts.
The contract has a function to get the name of the token.
The contract has a function to get the symbol of the token.
The contract has a function to get the decimals of the token.
The contract has a function to get the total supply of the token.
The contract has a function to get the balance of a user.
The contract has a function to get the allowance of a user.
*/

/*TODO:
An in-depth analysis of the contract. Comments should be added to the code snippet to explain the concepts shown in Lecture of Week 2.
*/

#[starknet::interface] //Defines an interface for the ERC20 token
pub trait IERC20<TContractState> {
    fn get_name(self: @TContractState) -> felt252; //a felt252 is a 252-bit integer that represents an element in a finite field, which is a mathematical concept used in zero-knowledge proofs and cryptography.
    fn get_symbol(self: @TContractState) -> felt252;
    fn get_decimals(self: @TContractState) -> u8;
    fn get_total_supply(self: @TContractState) -> felt252; // The @ symbol in Cairo is similar to & in Rust - it indicates a reference.
    fn balance_of(self: @TContractState, account: ContractAddress) -> felt252; //felt252 (Field Element) is a fundamental data type in Cairo, the language used for Starknet smart contracts. It's a 252-bit integer that represents an element in a finite field, which is a mathematical concept used in zero-knowledge proofs and cryptography.
    fn allowance(
        self: @TContractState, owner: ContractAddress, spender: ContractAddress
    ) -> felt252;
    fn transfer(ref self: TContractState, recipient: ContractAddress, amount: felt252);
    fn transfer_from(
        ref self: TContractState,
        sender: ContractAddress,
        recipient: ContractAddress,
        amount: felt252
    );
    fn approve(ref self: TContractState, spender: ContractAddress, amount: felt252);
    fn increase_allowance(ref self: TContractState, spender: ContractAddress, added_value: felt252);
    fn decrease_allowance(
        ref self: TContractState, spender: ContractAddress, subtracted_value: felt252
    );
}

#[starknet::interface] //Defines an interface for the SimpleVault contract
pub trait ISimpleVault<TContractState> {
    fn deposit(ref self: TContractState, amount: u256);
    fn withdraw(ref self: TContractState, shares: u256);
    fn user_balance_of(ref self: TContractState, account: ContractAddress) -> u256;
    fn contract_total_supply(ref self: TContractState) -> u256;
}

#[starknet::contract]
pub mod SimpleVault {
    use super::{IERC20Dispatcher, IERC20DispatcherTrait}; //Imports the IERC20Dispatcher and IERC20DispatcherTrait traits from the super module
    use starknet::{ContractAddress, get_caller_address, get_contract_address}; //Imports the ContractAddress, get_caller_address, and get_contract_address functions from the starknet module

    #[storage]
    struct Storage {//Defines a struct for the storage of the SimpleVault contract
        token: IERC20Dispatcher,
        total_supply: u256,
        balance_of: LegacyMap<ContractAddress, u256>
    }

    #[constructor] //Defines a constructor for the SimpleVault contract
    fn constructor(ref self: ContractState, token: ContractAddress) {
        self.token.write(IERC20Dispatcher { contract_address: token }); //Writes the token address to the token field of the storage
    }

    #[generate_trait] //Defines a trait for the private functions of the SimpleVault contract
    impl PrivateFunctions of PrivateFunctionsTrait {
        fn _mint(ref self: ContractState, to: ContractAddress, shares: u256) {
            //update total supply and balance of to
            self.total_supply.write(self.total_supply.read() + shares);
            self.balance_of.write(to, self.balance_of.read(to) + shares);
        }

        fn _burn(ref self: ContractState, from: ContractAddress, shares: u256) {
            //update total supply and balance of from
            self.total_supply.write(self.total_supply.read() - shares);
            self.balance_of.write(from, self.balance_of.read(from) - shares);
        }
        
    }

    #[abi(embed_v0)] //Defines an ABI for the SimpleVault contract
    impl SimpleVault of super::ISimpleVault<ContractState> {

        fn user_balance_of(ref self: ContractState, account: ContractAddress) -> u256 {
            self.balance_of.read(account)
        }

        fn contract_total_supply(ref self: ContractState) -> u256 {
            self.total_supply.read()
        }


        fn deposit(ref self: ContractState, amount: u256){
            let caller = get_caller_address();
            let this = get_contract_address();

            let mut shares = 0;
            if self.total_supply.read() == 0 {
                shares = amount;
            } else {
                let balance: u256 = self.token.read().balance_of(this).try_into()
                .unwrap();
                shares = (amount * self.total_supply.read()) / balance;
            }
            
           PrivateFunctions::_mint(ref self, caller, shares);
           
            let amount_felt252: felt252 = amount.low.into();
            self.token.read().transfer_from(caller, this, amount_felt252);
        }

        fn withdraw(ref self: ContractState, shares: u256) {
            let caller = get_caller_address();
            let this = get_contract_address();

            let balance = self.user_balance_of(this);
            let amount = (shares * balance) / self.total_supply.read();
            PrivateFunctions::_burn(ref self, caller, shares);
            let amount_felt252: felt252 = amount.low.into();
            self.token.read().transfer(caller, amount_felt252);
        }
    }
}