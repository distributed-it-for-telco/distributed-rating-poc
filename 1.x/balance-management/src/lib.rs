use crate::orange::commons::types::{Balance};

use crate::exports::orange::balancemanager::balancemanager::*;
use wasi::logging::logging::{log,Level::Info};
use serde_json; // For JSON serialization/deserialization
mod serializer;
use serializer::*;

wit_bindgen::generate!({
    generate_all
});



struct BalanceManager;

impl Guest for BalanceManager {
    fn get_balance(customer_id: String, offer_id: String) -> Balance {
       
        let bucket = wasi::keyvalue::store::open("").expect("failed to open empty bucket");
        let object_name = format!("{}:{}:{}", "balance", customer_id,offer_id);
        log(Info, "", &object_name);
        
        let balance_utf8 = bucket.get(&object_name).expect("couldn't retrieve count");
        let balance_str = String::from_utf8(balance_utf8.clone().unwrap()).unwrap();
        
        log(Info, "", &balance_str);

        let balance_dto: BalanceDTO = serde_json::from_str(&balance_str).unwrap();
        log(Info, "", &balance_dto.to_string());
        
        balance_dto.into()
    }
    
    // Function to check if the balance is sufficient
    fn has_sufficient_balance(balance: Balance, amount: f32) -> bool {
        balance.count >= amount
    }

    // Function to purchase and decrement the balance
    fn purchase(mut balance: Balance, amount: f32, customer_id: String, offer_id: String) -> Result<Balance, String> {
        if Self::has_sufficient_balance(balance.clone() ,amount) {
            balance.count -= amount;

            let bucket = wasi::keyvalue::store::open("").expect("failed to open empty bucket");
            let object_name = format!("{}:{}:{}", "balance", customer_id,offer_id);
            let balance_dto:BalanceDTO = balance.clone().into();
            let _ =  bucket.set(&object_name, &serde_json::to_vec(&balance_dto).unwrap());

            Ok(Balance { count: balance.count, unit: balance.unit.clone(), party_id: balance.party_id.clone() })
        } else {
            Err(String::from("Insufficient balance to purchase"))
        }
    }

}

export!(BalanceManager);
