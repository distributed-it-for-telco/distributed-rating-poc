wit_bindgen::generate!({
    generate_all
});

use std::collections::HashMap;
use crate::wasi::logging::logging::{log, Level::Info};
use crate::orange::rating::types::*;
use crate::exports::orange::ratingcoordinator::ratingcoordinator::Guest;
use futures::executor::block_on;

use build_agents_hierarchy::*;
use validation::*;
use rating::*;

mod agent_graph;
mod build_agents_hierarchy;
mod validation;
mod rating;
mod types;

struct RatingCoordinator;

impl RatingCoordinator{
   async fn handle_rating_process_async(rating_request: RatingRequest, headers: HashMap<String,String>
    ) -> RatingResponse {
        log(Info, "", "Hello I'm your rating coordinator");
        log(Info, "", format!("Current used agent is: {}",rating_request.agent_id).as_str());

        let agent_graph = build_agent_hierarchy(&rating_request).await.unwrap();

        log(Info, "","Graph generated ......");

       let validation_response_as_rating =
             handle_validation_cycle(&rating_request, &agent_graph).await.unwrap();
        
        if validation_response_as_rating.authorization_status.code == 401 {
            return validation_response_as_rating;
        }

        return handle_rating_cycle(&rating_request, &agent_graph).await;
    }
}

impl Guest for RatingCoordinator {
    fn handle_rating_process(
        rating_process_request: RatingRequest,
        headers:  Vec<(String, Vec<u8>)>
    ) -> RatingResponse {
        let headersMap: HashMap<_, _> = headers.into_iter().map(|(a,b)| (a.String::from_utf8(b))).collect();
        block_on(Self::handle_rating_process_async(rating_process_request,headersMap));
        RatingResponse{
            authorization_status: AuthorizationStatus{
                code: 15,
                key: "".to_string()
            },
            billing_information: BillingInformation{
                messages: vec!["".to_string()],
                price: "".to_string(),
                unit: "".to_string()
            },
            next_agent: AgentIdentification{
                name: "".to_string(),
                partner_id: "".to_string()
            }
        }
    }
}

export!(RatingCoordinator); 
