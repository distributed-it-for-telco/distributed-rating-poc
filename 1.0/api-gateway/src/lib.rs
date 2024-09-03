wit_bindgen::generate!({
    generate_all
});

use crate::orange::ratingagent::*;
use crate::orange::ratingagent::types::{RatingRequest, RatingResponse, Usage};
use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::*;
mod dtos;
// use exports::wasi::logging::logging::*;

struct HttpServer;

impl Guest for HttpServer {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
        
        let rating_request = RatingRequest {
            customer_id: "Mariem".to_string(),
            agent_id: "agent1".to_string(),
            language: "arabicss".to_string(),
            offer_id: "ay7agaba2a".to_string(),
            usage: Usage {
                usage_characteristic_list: (&[]).to_vec(),
            },
            rating_history: (&[]).to_vec(),
        };
        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();
        let response_body = response.body().unwrap();
        ResponseOutparam::set(response_out, Ok(response));
        let usageResult: RatingResponse = ratingagent::rate_usage(&rating_request);
        

        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(serde_json::to_string(&dtos::types::DTORatingResponse::from(usageResult)).unwrap())
            .unwrap();
        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
    }
}

export!(HttpServer);