wit_bindgen::generate!({
    generate_all
});

use crate::orange::rating::types::*;
use crate::orange::rating::*;
use wasi::logging::logging::log;

mod agent_graph;
mod build_agents_hierarchy;
mod validation;
mod types;

struct HttpServer;

impl HttpServer {
    // fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
    //     let response = OutgoingResponse::new(Fields::new());
    //     response.set_status_code(200).unwrap();
    //     let response_body = response.body().unwrap();
    //     ResponseOutparam::set(response_out, Ok(response));
    //     response_body
    //         .write()
    //         .unwrap()
    //         .blocking_write_and_flush(b"Hello from Rust!\n")
    //         .unwrap();
    //     OutgoingBody::finish(response_body, None).expect("failed to finish response body");
    // }
}

// export!(HttpServer);
