wit_bindgen::generate!({
    generate_all
});

use std::collections::HashMap;
use crate::wasi::logging::logging::{log, Level::Info};
use crate::orange::commons::types::{RatingRequest, RatingResponse};
use crate::orange::commons::error_types::GenericError;
use crate::exports::orange::ratingcoordinator::ratingcoordinator::Guest;
use futures::executor::block_on;

use build_agents_hierarchy::*;
use validation::*;
use rating::*;

mod agent_graph;
mod build_agents_hierarchy;
mod validation;
mod rating;

struct RatingCoordinator;

impl RatingCoordinator{
   async fn handle_rating_process_async(rating_request: RatingRequest, headers: HashMap<String,String>
    ) -> Result<RatingResponse, GenericError> {
        log(Info, "", "Hello I'm your rating coordinator");
        log(Info, "", format!("Current used agent is: {}",rating_request.agent_id).as_str());
        let agent_graph = build_agent_hierarchy(&rating_request).await.unwrap();
        log(Info, "","Graph generated ......");
        let validation_response_as_rating =
        match handle_validation_cycle(&rating_request, headers, &agent_graph).await {
            Ok(validation) => validation,
            Err(e) => return Err(GenericError::Validation(e.message))
        };
        if validation_response_as_rating.authorization_status.code == 401 {
            return Ok(validation_response_as_rating)
        }

       match handle_rating_cycle(&rating_request, &agent_graph).await {
            Ok(rating) => Ok(rating),
            Err(e) => Err(GenericError::Usage(e.message)),
       }
    }
}

impl Guest for RatingCoordinator {
    fn handle_rating_process(
        rating_process_request: RatingRequest,
        headers:  Vec<(String, Vec<u8>)>
    ) -> Result<RatingResponse, GenericError> {
        let client_country_bytes: HashMap<String, String> = headers.into_iter().map(|(a,b)| (a,String::from_utf8(b).unwrap())).collect();
        block_on(Self::handle_rating_process_async(rating_process_request,client_country_bytes))
    }
}

export!(RatingCoordinator); 
