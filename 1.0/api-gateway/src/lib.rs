wit_bindgen::generate!({
    generate_all
});

use crate::orange::ratingagent::*;
use crate::orange::ratingagent::types::{RatingRequest, RatingResponse, Usage};
use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::*;
// use wasi:://logging::*;

// use //log::*;
// use wasm_//logger::Config;

mod serializer;
use serializer::*;
// use exports::wasi:://logging:://logging::*;

struct HttpServer;

impl Guest for HttpServer {

    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
        // wasm_//logger::init(Config::new(Level::Info)); 

        // //log::info!("{}", &"1");
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
        //log::info!("{}", &"2");
        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();
        let response_body = response.body().unwrap();
        let usageResult: RatingResponse = ratingagent::rate_usage(&rating_request);
        ResponseOutparam::set(response_out, Ok(response));
        let serializedRatingReslult: SerializedRatingResponse = usageResult.into();

        //log::info!("{}", &"3");
        // let responseDTO = dtos::dto_types::DTORatingResponse::from(usageResult);

        let binding = serde_json::to_string(&serializedRatingReslult).unwrap();
        let serializedResponse  = binding.as_bytes();
        // let serializedResponse  = serde_json::to_string(&response).unwrap().as_bytes();

        //log::info!("{}", &"4");
        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(&serializedResponse)
            .unwrap();

            //log::info!("{}", &"5");
        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
        //log::info!("{}", &"6");
        
    }
}

export!(HttpServer);
