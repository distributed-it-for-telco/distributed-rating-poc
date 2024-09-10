wit_bindgen::generate!({ generate_all });

use crate::orange::ratingagent::types::{RatingRequest, RatingResponse, Usage};
use crate::orange::ratingagent::*;
use exports::wasi::http::incoming_handler::Guest;
use serializer::*;
use wasi::http::types::*;
use wasi::io::streams::InputStream;
use wasi::logging::logging::log;
mod serializer;

struct ApiGateway;

impl ApiGateway {
    pub fn readInputStream(_input: &InputStream) -> String {
        return Self::readInputStreamWithBufferSize(_input, 100);
    }
    pub fn readInputStreamWithBufferSize(_input: &InputStream, _buffer_size: u64) -> String {
        let mut stream_data: Vec<u8> = Vec::new();
        loop {
            let stream_read = _input.blocking_read(_buffer_size).unwrap();
            let dataLength = stream_read.len();
            stream_data.extend(stream_read);
            if dataLength < _buffer_size.try_into().unwrap() {
                break;
            }
        }
        let _result = String::from_utf8(stream_data).unwrap();
        return _result;
    }
}

impl Guest for ApiGateway {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
        let body: IncomingBody = _request.consume().unwrap();
        let body_stream = body.stream().unwrap();
        let serialized_rating_request: SerializedRatingRequest =
            serde_json::from_str(&(Self::readInputStream(&body_stream))).unwrap();
        let rating_request: RatingRequest = serialized_rating_request.into();

        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();
        let response_body = response.body().unwrap();
        let usageResult: RatingResponse = ratingagent::rate_usage(&rating_request);

        ResponseOutparam::set(response_out, Ok(response));
        let serializedRatingReslult: SerializedRatingResponse = usageResult.into();
        let binding = serde_json::to_string(&serializedRatingReslult).unwrap();
        let serializedResponse = binding.as_bytes();

        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(&serializedResponse)
            .unwrap();

        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
    }
}

export!(ApiGateway);
