use exports::orange::rating::ratingagent::Guest;
use wasi::logging::logging::{log,Level::Info};
use futures::executor::block_on;
use crate::orange::commons::types::*;
use crate::orange::usagecollector::usagecollector;
use crate::orange::commons::commons::{generate_rating_proof};
use crate::orange::commons::rating_response_builder;
use crate::orange::commons::error_types::*;

use uuid::Uuid;

wit_bindgen::generate!({
    generate_all
});

const OFFER_ID: &str = "1000";
const AWS_PARTY_ID_AT_PARTNER_SIDE: &str = "aws_my_partner";
const PROVIDER_AGENT_NAME: &str = "orange_connectivity";
const REPLICATION_FACTOR: u32 = 2;
const RATE_FEE: f32 = 1.0;
const RATING_PROOF_DESC: &str = "AWS Stor";
const RATING_PROOF_USAGE_TYPE: &str = "AWSStor";
const RATING_PROOF_PRODUCT_NAME: &str = "AWS Stor";

struct AwsStoreRatingagent;

impl AwsStoreRatingagent{
    async fn rate_usage_async(_request: RatingRequest) -> Result<RatingResponse, UsageError> {
        {
            if _request.usage.usage_characteristic_list.is_empty() {
                return Err(UsageError{message: "Can't rate usage, no characteristic sent!".to_string(), cause: "".to_string()});
            }
    
            log(Info,"","Hello I'm your AWS stor postpaid rating agent");
    
            let usage_date = "07/08/2023";
            let usage_id: String = Uuid::new_v4().to_string();
    
            /*
             *  Contract or Offer is 1 GB = 1 EUR
             */
    
            let mut storage_usage: f32 = 1.0;
            for characteristic in _request.usage.usage_characteristic_list.clone().iter_mut() {
                storage_usage *= characteristic.value.parse::<f32>().unwrap();
            }
    
            let rating = RATE_FEE * storage_usage;
    
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
    
            log(Info,"",format!(
                "Sending usage proof to usage collector for party with id: {}",
                _request.customer_id.to_string()
            ).as_str());
            usagecollector::store(&usage_template_str);

            let mut handler = rating_response_builder::create_builder();
            handler = rating_response_builder::unit(handler, &"EUR");
            handler = rating_response_builder::price(handler, &rating.to_string());
            handler = rating_response_builder::authorized(handler);

            let rating_response = rating_response_builder::build(handler);

            Ok(rating_response)
        }
    }

    async fn validate_async(request: ValidationRequest) -> Result<ValidationResponse, ValidationError> {
        let validation_response = ValidationResponse {            
            valid: !request.client_country.is_empty() && request.client_country == "EG"
        };
        Ok(validation_response)
    }

     async fn get_children_async(arg: &GetChildrenRequest) -> AgentList {
        let mut connectivity: f32 = 1.0;
        for characteristic in arg.usage.usage_characteristic_list.to_owned().iter_mut() {
            connectivity *= characteristic.value.parse::<f32>().unwrap();
        }

        let connectivity_usage = UsageCharacteristic {
            name: "connectivity".to_string(),
            value: connectivity.to_string(),
            value_type: "float".to_string(),
        };

        let mut translated_usage = arg.usage.to_owned();
        let characteristics_vector = vec![connectivity_usage];
        translated_usage.usage_characteristic_list = characteristics_vector;

        let child = Agent {
            identification: AgentIdentification {
                name: PROVIDER_AGENT_NAME.to_string(),
                partner_id: AWS_PARTY_ID_AT_PARTNER_SIDE.to_string(),
            },
            usage: translated_usage,
        };

        let mut children_list = AgentList{
            agents: vec![]
        };
        children_list.agents.push(child);

        children_list
    }
    
}

impl Guest for AwsStoreRatingagent {
    fn rate_usage(request: RatingRequest) -> Result<RatingResponse, UsageError> {
        block_on(Self::rate_usage_async(request))
    }
    fn validate(request: ValidationRequest) -> Result<ValidationResponse, ValidationError> {
        block_on(Self::validate_async(request))
    }
    fn get_children(request: GetChildrenRequest) -> AgentList {
        block_on(Self::get_children_async(&request))
    }
}

export!(AwsStoreRatingagent);
