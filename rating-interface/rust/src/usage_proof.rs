use crate::UsageProofRequest;
use serde_json::json;

pub struct UsageProofHandler {}

impl UsageProofHandler {
    pub fn generate_rating_proof(usage_proof_request: &UsageProofRequest) -> String {
        let rating_date = "04/04/2023";

        let usage_template_str = json!({
            "id": usage_proof_request.usage_id,
            "usageDate": usage_proof_request.usage_date,
            "description": "Video on Demand with Bucket",
            "usageType": "VoD",
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
                    "name": "Video on Demand with Bucket"
                }
            },
            "relatedParty": {
                "id": usage_proof_request.party_id
            },
            "usageCharacteristic": usage_proof_request.usage_characteristic_list
        });

        usage_template_str.to_string()
    }
}