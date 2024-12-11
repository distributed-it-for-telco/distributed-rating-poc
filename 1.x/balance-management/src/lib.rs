use crate::orange::commons::types::Balance;
use crate::orange::commons::error_types::{GenericError, OtherError};
use crate::orange::commons::mappers;

use crate::exports::orange::balancemanager::balancemanager::*;
use wasi::logging::logging::{log,Level::Info};
use wasi::keyvalue::store;

wit_bindgen::generate!({
    generate_all
});



struct BalanceManager;

impl Guest for BalanceManager {
    fn get_balance(customer_id: String, offer_id: String) -> Balance {
       
        let bucket = store::open("").expect("failed to open empty bucket");
        let object_name = format!("{}:{}:{}", "balance", customer_id,offer_id);
        log(Info, "", &object_name);
        
        let balance_utf8 = bucket.get(&object_name).expect("couldn't retrieve count");
        let balance_str = String::from_utf8(balance_utf8.clone().unwrap()).unwrap();
        
        log(Info, "", &balance_str);
        mappers::string_to_balance(&balance_str)
    }
    
    // Function to check if the balance is sufficient
    fn has_sufficient_balance(balance: Balance, amount: f32) -> bool {
        balance.count >= amount
    }

    // Function to purchase and decrement the balance
    fn purchase(mut balance: Balance, amount: f32, customer_id: String, offer_id: String) -> Result<Balance, GenericError> {
        if Self::has_sufficient_balance(balance.clone() ,amount) {
            balance.count -= amount;

            let bucket = store::open("").expect("failed to open empty bucket");
            let object_name = format!("{}:{}:{}", "balance", customer_id,offer_id);
            let _ =  bucket.set(&object_name, &mappers::balance_to_stringified_array(&balance));

            Ok(Balance { count: balance.count, unit: balance.unit.clone(), party_id: balance.party_id.clone() })
        } else {
            Err(GenericError::Other(OtherError{message: "Insufficient balance to purchase".to_string()}))
        }
    }

}

export!(BalanceManager);
