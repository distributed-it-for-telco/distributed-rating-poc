wit_bindgen::generate!({ 
    generate_all 
});

use wasi::io::streams::InputStream;
use wasi::logging::logging::{log, Level::Info};
use wasi::http::types::*;
use exports::wasi::http::incoming_handler::Guest;

use crate::orange::ratingcoordinator::ratingcoordinator;
use crate::orange::commons::mappers;
use crate::orange::commons::types::{RatingRequest, RatingResponse};


struct ApiGateway;
struct HttpRequestParts {
    method: Method,
    path: String,
    // query: Vec<(&str, &str)>
}
impl ApiGateway {
    fn get_request_parts(path_with_query: String, _method: Method) -> HttpRequestParts {
        // At first, we can assume the path_without_query will be the path with query
        // (ex. simple paths like '/some-key-here')
        let mut path_without_query = path_with_query.clone();
        // If query parameters were supplied, then we need to recalculate the path_without_query
        // and extract query paramters to separate map
        if let Some((path, query)) = path_with_query.split_once('?') {
            path_without_query = path.to_string();
            let _query_params = query
                .split('&')
                .filter_map(|v| v.split_once('='))
                .collect::<Vec<(&str, &str)>>();
        }
        return HttpRequestParts {
            path: path_without_query,
            // query: query_params,
            method: _method,
        };
    }

    fn map_request_to_service(_request: IncomingRequest, response_out: ResponseOutparam) {
        log(Info, "", "Http Request recieved");
        let request_parts =
            Self::get_request_parts(_request.path_with_query().unwrap(), _request.method());
        let http_request_body: IncomingBody = _request.consume().unwrap();
        let body = StreamReader::read_input_stream(&http_request_body.stream().unwrap());
        log(Info, "", &request_parts.path);
        
        match (request_parts.method, request_parts.path.as_str()) {
            // ("OPTIONS", _) => get_options_response(ctx).await,
            (wasi::http::types::Method::Post, "/usage/rating") => {
                
                Self::request_rate(_request.headers(),body,response_out);
            }
           

            (_, _) => Self::not_found(response_out),
        }
        
    }
    fn not_found(response_out: ResponseOutparam){
        let headers = Fields::new();
        let _ = headers.set(&"Content-Type".to_string(),  &vec![b"application/json".to_vec()]);
        let response = OutgoingResponse::new(headers);
        response.set_status_code(404).unwrap();
        
        let response_body = response.body().unwrap();
        ResponseOutparam::set(response_out, Ok(response));

        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(b"no API found for this request")
            .unwrap();

        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
    }

    fn request_rate(request_headers:Fields, body: String ,response_out: ResponseOutparam){

        log(Info, "", &"requesting rate");

        let rating_request: RatingRequest = mappers::string_to_rating_request(&body);
        
        let headers = Fields::new();
        let _ = headers.set(&"Content-Type".to_string(),  &vec![b"application/json".to_vec()]);
        let response = OutgoingResponse::new(headers);

        response.set_status_code(200).unwrap();
        let response_body = response.body().unwrap();
        log(Info, "", "before calling rating agent");

        let usage_result: RatingResponse = ratingcoordinator::handle_rating_process(&rating_request,&request_headers.entries()).expect("Failed to call rating agent");

        log(Info, "", &"after calling rating agent");
        log(Info, "", &usage_result.billing_information.unit);

        ResponseOutparam::set(response_out, Ok(response));
        let binding = mappers::rating_response_to_string(&usage_result);
        let serialized_response = binding.as_bytes();
        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(&serialized_response)
            .unwrap();

        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
    }
}



struct StreamReader;

impl StreamReader {
    pub fn read_input_stream(_input: &InputStream) -> String {
        return Self::read_input_stream_with_buffer_size(_input, 100);
    }
    pub fn read_input_stream_with_buffer_size(_input: &InputStream, _buffer_size: u64) -> String {
        let mut stream_data: Vec<u8> = Vec::new();
        loop {
            let stream_read = _input.blocking_read(_buffer_size).unwrap();
            let data_length = stream_read.len();
            stream_data.extend(stream_read);
            if data_length < _buffer_size.try_into().unwrap() {
                break;
            }
        }
        let _result = String::from_utf8(stream_data).unwrap();
        return _result;
    }
}

impl Guest for ApiGateway {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
        Self::map_request_to_service(_request, response_out);
    }
}

export!(ApiGateway);


 // ("GET", ["usage", "rating-proofs", usage_collector_id]) => {
            //     list_usage_proofs(ctx, usage_collector_id).await
            // }
            // ("POST", ["seed", "orange", "customer", "inventory"]) => {
            //     seed_data_for_orange_cust_inventory(ctx).await
            // }
            // ("GET", ["party", party_id, "offers", inventory_agent_id]) => {
            //     get_party_offers(ctx, party_id, inventory_agent_id).await
            // }

            // ("POST", ["balance", "topup"]) => topup_balance(ctx, deser(&req.body)?).await,