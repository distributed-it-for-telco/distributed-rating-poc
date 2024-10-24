wit_bindgen::generate!({
    generate_all
});

use crate::wasi::logging::logging::{log, Level::Info};
use crate::orange::rating::types::*;
use crate::exports::orange::rating_coordinator::ratingcoordinator::{Guest,RatingProcessRequest};

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
   async fn handle_rating_process_async(rating_process_request: RatingProcessRequest,
    ) -> RatingResponse {
        log(Info, "", "Hello I'm your rating coordinator");
        log(Info, "", format!("Current used agent is: {}",rating_process_request.rating_request.agent_id).as_str());

        let agent_graph = build_agent_hierarchy(&rating_process_request.rating_request).await.unwrap();

        log(Info, "","Graph generated ......");

       let validation_response_as_rating =
             handle_validation_cycle(&rating_process_request, &agent_graph).await.unwrap();
        
        if validation_response_as_rating.authorization_status.code == 401 {
            return validation_response_as_rating;
        }

        return handle_rating_cycle(&rating_process_request, &agent_graph).await;
    }
}

impl Guest for RatingCoordinator {
    fn handle_rating_process(
        rating_process_request: RatingProcessRequest,
    ) -> RatingResponse {
        return block_on(Self::handle_rating_process_async(rating_process_request));
    }
}

export!(RatingCoordinator);
