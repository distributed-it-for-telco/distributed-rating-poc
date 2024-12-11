

use crate::orange::commons::types::UsageProofRequest;
use crate::orange::commons::error_types::{
    UsageError, ValidationError
};
use crate::orange::commons::commons::generate_rating_proof;
use crate::orange::commons::rating_response_builder as RatingResponseBuilder;
use exports::orange::rating::ratingagent::*;
use wasi::logging::logging::{log,Level::Info};
use crate::orange::usagecollector::usagecollector;
use uuid::Uuid;

wit_bindgen::generate!({
    generate_all,
});

const OFFER_ID: &str = "video";
const RATING_PROOF_DESC: &str = "Streamzie Movies on demand";
const RATING_PROOF_USAGE_TYPE: &str = "VoDVend";
const RATING_PROOF_PRODUCT_NAME: &str = "Streamzie Movies on demand";

struct VideoRatingagent;

impl Guest for VideoRatingagent {
    /// Say hello!
    fn rate_usage(_request: RatingRequest) -> Result<RatingResponse, UsageError> {

        log(wasi::logging::logging::Level::Info, "", "Hello I'm your video provider postpaid rating agent");

        let usage_date = "21/06/2023";
        let usage_id: String = Uuid::new_v4().to_string();
        let mut rating: f32 = 0.0;

        if let Some(first) = _request.usage.usage_characteristic_list.first() {
            rating = first.value.parse::<i32>().unwrap() as f32;
        }
        
        let usage_template_str = generate_rating_proof(&UsageProofRequest {
            party_id: _request.customer_id.to_owned(),
            rating: rating.to_string(),
            usage_characteristic_list: _request.usage.usage_characteristic_list.to_owned(),
            usage_id: usage_id.as_str().to_owned(),
            usage_date: usage_date.to_owned(),
            offer_id: OFFER_ID.to_owned(),
            description: RATING_PROOF_DESC.to_owned(),
            usage_type: RATING_PROOF_USAGE_TYPE.to_owned(),
            product_name: RATING_PROOF_PRODUCT_NAME.to_owned(),
        });

        log(Info, "", "Sending usage proof to video provider usage collector");

        usagecollector::store(&usage_template_str);

        log(wasi::logging::logging::Level::Info, "", "Retrieving usage list from video provider usage collector\n\n\n");
        usagecollector::get_list().iter().for_each(|usage| {
            log(wasi::logging::logging::Level::Info, "", &usage.value);
        });


        let mut response_builder_handle = RatingResponseBuilder::create_builder();
        response_builder_handle= RatingResponseBuilder::unit(response_builder_handle, &"EUR");
        response_builder_handle= RatingResponseBuilder::price(response_builder_handle, &rating.to_string());
        response_builder_handle= RatingResponseBuilder::authorized(response_builder_handle);
        Ok(RatingResponseBuilder::build(response_builder_handle))
    }

    fn validate(_request: ValidationRequest) -> Result<ValidationResponse, ValidationError> {
        let mut validation_response: ValidationResponse = ValidationResponse{valid: true};

        if !_request.client_country.is_empty() && _request.client_country.to_owned() == "EG" {
            validation_response.valid = true;
        } else {
            validation_response.valid = false;
        }

        Ok(validation_response)
    }

    fn get_children(_request: GetChildrenRequest) -> AgentList {
        AgentList {
            agents: vec![],
        }
    }
}

export!(VideoRatingagent);
