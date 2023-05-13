#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
// collections of types and functions, grouping them in a single large scope.
mod my_token {
    use ink::storage::Mapping;

    // be strictly conservative because of fees
    #[ink(storage)]
    // initialize with default values
    #[derive(Default)]
    pub struct MyToken {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
    }

    pub enum Result<T, E> {
        Ok(T), // Expected type
        Err(E), // Error type of choice when initializing a Result
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
    }

    impl MyToken {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            // needs to be initialized in rust
            let mut balances = Mapping::default();
            // access the address of the contract owner/creator
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            Self {
                total_supply,
                balances
            }
        }

        #[ink(message)]
        // methods with this macro needs to be declared as public
        // getter for total supply
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        // get balance of specified account
        pub fn balance_of(&self, account: AccountId) -> Balance {
            self.balances.get(&account).unwrap_or_default()
        }

        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result <(), Error> {
            todo!()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_function() {
            assert_eq!(1, 1)
        }

    }


}

