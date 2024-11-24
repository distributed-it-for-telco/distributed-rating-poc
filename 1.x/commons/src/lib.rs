wit_bindgen::generate!({
    generate_all
});

mod dtos;

use crate::orange::commons::types::*;

use crate::exports::orange::commons::commons::Guest;
use crate::exports::orange::commons::mappers::Guest as Mappers;
use dtos::SerializedRatingRequest;
use serde_json::json;

struct Commons;

impl Guest for Commons {
    // pub fn create_response_builder() -> RatingResponseBuilder {
    //     RatingResponseBuilder::new()
    // }

    fn create_balance(count: f32, unit: String, party_id: String) -> Balance {
        Balance { count, unit, party_id }
    }
    fn generate_rating_proof(usage_proof_request: UsageProofRequest) -> String{
        let rating_date = "04/04/2023";

        let usage_template_str = json!({
            "id": usage_proof_request.usage_id,
            "usageDate": usage_proof_request.usage_date,
            "description": usage_proof_request.description,
            "usageType": usage_proof_request.usage_type,
            "ratedProductUsage": {
                "isBilled": false,
                "ratingAmountType": "Total",
                "ratingDate": rating_date,
                "bucketValueConvertedInAmount": {
                    "unit": "EUR",
                    "value": usage_proof_request.rating
                },
                "productRef": {
                    "id": usage_proof_request.offer_id,
                    "name": usage_proof_request.product_name
                }
            },
            "relatedParty": {
                "id": usage_proof_request.party_id
            },
            // "usageCharacteristic": usage_proof_request.usage_characteristic_list
        });

        usage_template_str.to_string()

    }
}

impl Mappers for Commons{
    fn rating_request_to_string(request: RatingRequest) -> String{
        let serialized_rating_result: SerializedRatingRequest = request.into();
        serde_json::to_string(&serialized_rating_result).unwrap()
    }
    
    fn string_to_rating_request(request: String) -> RatingRequest{
        let serialized_rating_request: SerializedRatingRequest =
            serde_json::from_str::<SerializedRatingRequest>(&request).unwrap().into();
            serialized_rating_request.into()
    }
}
export!(Commons);