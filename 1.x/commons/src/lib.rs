wit_bindgen::generate!({
    generate_all
    // additional_derives: [Defauylt]
});

use crate::orange::commons::*;
// use crate::rating_response_builder::RatingResponseBuilder;

use exports::orange::commons::*;
use crate::commons::Guest;

struct Commons;

impl Guest for Commons {
    // pub fn create_response_builder() -> RatingResponseBuilder {
    //     RatingResponseBuilder::new()
    // }

    fn create_balance(count: f32, unit: String, party_id: String) -> Balance {
        Balance { count, unit, party_id }
    }
}

export!(Commons);