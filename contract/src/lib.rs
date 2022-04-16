use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct HelloWorld {
    //
}

#[near_bindgen]
impl HelloWorld {

    pub fn get_hello_message(&self, name: &String) -> String {
        "Hello ".to_string() + name + "!"
    }

}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn get_hello_message(){
        // Get Alice as an account ID
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice);
        testing_env!(context.build());

        let contract = HelloWorld::default();
        let message = contract.get_hello_message(&"John".to_string());
        assert_eq!("Hello John!".to_string(), message);
        println!("Let's debug: {:?}", message);
    }
}
