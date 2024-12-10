use crate::commons_struct::Commons;
use crate::exports::orange::commons::rating_response_builder::Guest as RatingResponseBuilder;
use crate::orange::commons::types::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;


struct SimpleTypeBuilder {
    authorized: bool,
    authorization_message: String,
    unit: String,
    price: String,
    messages: Vec<String>,
    next_agent: AgentIdentification,
}

static BUILDERS: Lazy<Mutex<HashMap<u32, SimpleTypeBuilder>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));
static NEXT_ID: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

impl RatingResponseBuilder for Commons{
    fn create_builder() -> u32 {
        let id = *NEXT_ID.lock().unwrap();
        *NEXT_ID.lock().unwrap() += 1;
        
        BUILDERS.lock().unwrap().insert(id, SimpleTypeBuilder {
            authorized: true,
            authorization_message: "".to_string(),
            unit: "".to_string(),
            price: "".to_string(),
            messages: vec![],
            next_agent: AgentIdentification{
                name: "".to_string(),
                partner_id: "".to_string()
            }
        });
        id
    }

    fn unit(handle: u32, value: String) -> u32 {
        if let Some(builder) = BUILDERS.lock().unwrap().get_mut(&handle) {
            builder.unit = value;
        }
        handle
    }
    fn price(handle: u32, value: String) -> u32 {
        if let Some(builder) = BUILDERS.lock().unwrap().get_mut(&handle) {
            builder.price = value;
        }
        handle
    }
    fn message(handle: u32, value: String) -> u32 {
        if let Some(builder) = BUILDERS.lock().unwrap().get_mut(&handle) {
            builder.messages.push(value);
        }
        handle
    }
    fn authorized(handle: u32) -> u32 {
        if let Some(builder) = BUILDERS.lock().unwrap().get_mut(&handle) {
            builder.authorized = true;
        }
        handle
    }
    fn not_authorized(handle: u32) -> u32 {
        if let Some(builder) = BUILDERS.lock().unwrap().get_mut(&handle) {
            builder.authorized = false;
        }
        handle
    }
    fn next_agent(handle: u32, value: AgentIdentification) -> u32 {
        if let Some(builder) = BUILDERS.lock().unwrap().get_mut(&handle) {
            builder.next_agent = value;
        }
        handle
    }

    fn build(handle: u32) -> RatingResponse {
        let builder = BUILDERS.lock().unwrap().remove(&handle).unwrap();
        let mut authorization_status: AuthorizationStatus = AuthorizationStatus{
            code: 0,
            key: "".to_string()
        };
        if builder.authorized {
            authorization_status.code = 200;
        } else {
            authorization_status.code = 401;
        }
        authorization_status.key = builder.authorization_message;

        RatingResponse{
            authorization_status: authorization_status,
            billing_information: BillingInformation{
                messages: builder.messages,
                price: builder.price,
                unit: builder.unit
            },
            next_agent: AgentIdentification{
                name: "".to_string(),
                partner_id: "".to_string()
            }
        }
    }
}