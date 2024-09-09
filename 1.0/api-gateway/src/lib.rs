wit_bindgen::generate!({
    generate_all
});

use crate::orange::ratingagent::*;
use crate::orange::ratingagent::types::{RatingRequest, RatingResponse, Usage};
use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::*;

mod serializer;
use serializer::*;

use std::io::*;

struct HttpServer;

impl Guest for HttpServer {

    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
       
        let body: IncomingBody = _request.consume().unwrap();
        let body_stream = body.stream().unwrap().blocking_read(10000).unwrap();
        let serialized_rating_request: SerializedRatingRequest = serde_json::from_str(&String::from_utf8(body_stream).unwrap()).unwrap();
        let rating_request: RatingRequest =  serialized_rating_request.into();

        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();
        let response_body = response.body().unwrap();
        let usageResult: RatingResponse = ratingagent::rate_usage(&rating_request);

        ResponseOutparam::set(response_out, Ok(response));
        let serializedRatingReslult: SerializedRatingResponse = usageResult.into();
        let binding = serde_json::to_string(&serializedRatingReslult).unwrap();
        let serializedResponse  = binding.as_bytes();
        
        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(&serializedResponse)
            .unwrap();

        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
    }
}

export!(HttpServer);
