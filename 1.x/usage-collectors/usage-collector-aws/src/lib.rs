use crate::orange::usagecollector::types::{UsageProofDetails, UsageProofList};

use crate::exports::orange::usagecollector::usagecollector::*;
use wasi::logging::logging::log;
use serde_json; // For JSON serialization/deserialization

wit_bindgen::generate!({
    generate_all
});

struct AwsUsageCollector;

const USAGE_LIST_KEY: &str = "rating:usage:aws";

impl Guest for AwsUsageCollector {
    fn store(arg: String) {
        log(wasi::logging::logging::Level::Info, "", &"Aws Usage Collector".to_string());

        //open bucket
        let bucket = wasi::keyvalue::store::open("").expect("Failed to open bucket");
        //If usage list key doesn't exist create it
        match bucket.exists(USAGE_LIST_KEY) {
            Ok(true) => (),
            Ok(false) => {
                let _ = bucket.set(USAGE_LIST_KEY, &serde_json::to_vec::<Vec<String>>(&vec![]).unwrap());   
            },
            Err(_e) => panic!("Wasi exists returns an error"),
        }

        //get the already existing list
        let usage_list_result = bucket.get(USAGE_LIST_KEY).expect("Failed to get usage list");
        //deserialize the list
        let mut usage_list: Vec<String> = serde_json::from_str(&String::from_utf8(usage_list_result.clone().unwrap()).unwrap()).unwrap();
        //push new value and set in kv store
        usage_list.push(arg.to_string());
        bucket.set(USAGE_LIST_KEY, &serde_json::to_vec(&usage_list).unwrap()).expect("Failed to set usage list");

        return; 
    }

    fn get_list() -> UsageProofList {
        let bucket = wasi::keyvalue::store::open("").expect("Failed to open bucket");
        //If usage list key doesn't exist create it
        match bucket.exists(USAGE_LIST_KEY) {
            Ok(true) => (),
            Ok(false) => {
                panic!("Usage list key still doesn't exist, add some values first");    
            },
            Err(_e) => panic!("Wasi exists returns an error"),
        }

        //get the already existing list
        let usage_list_result = bucket.get(USAGE_LIST_KEY).expect("Failed to get usage list");
        //deserialize the list
        let usage_list: Vec<String> = serde_json::from_str(&String::from_utf8(usage_list_result.clone().unwrap()).unwrap()).unwrap();
        
        usage_list.into_iter().map(|s| UsageProofDetails { value: s }).collect()
    }
}

export!(AwsUsageCollector);