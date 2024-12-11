use crate::orange::commons::types::{ RatingRequest, RatingResponse,
    ValidationRequest, ValidationResponse
};
use crate::orange::commons::error_types::{ UsageError, ValidationError};
use exports::orange::rating::ratingagent::*;
use wasi::logging::logging::{log, Level::Info};

wit_bindgen::generate!({
    generate_all,
});
struct StreamingRatingagent;

impl Guest for StreamingRatingagent {

    fn rate_usage(_request: RatingRequest) -> Result<RatingResponse, UsageError> {
        log(Info, "", &"Streaming Rating Agent".to_string());
        todo!()
    }

    fn validate(_request: ValidationRequest) -> Result<ValidationResponse, ValidationError> {
        todo!()
    }

    fn get_children(_request: GetChildrenRequest) -> AgentList {
        todo!()
    }
}

export!(StreamingRatingagent);
