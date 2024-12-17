use crate::commons_struct::Commons;
use crate::orange::commons::types::{RatingResponse, BillingInformation, AuthorizationStatus, AgentIdentification};
use crate::exports::orange::commons::some_builder::{Builder, GuestBuilder, Guest};

#[derive(Clone)]
pub struct MyBuilder {
    authorized: bool,
    authorization_message: String,
    unit: String,
    price: String,
    messages: Vec<String>,
    next_agent: AgentIdentification,
}

impl GuestBuilder for MyBuilder {
    fn new() -> Self {
        MyBuilder { 
            authorized: true,
            authorization_message: "".to_string(),
            unit: "".to_string(),
            price: "".to_string(),
            messages: vec![],
            next_agent: AgentIdentification{
                name: "".to_string(),
                partner_id: "".to_string()
            }
         }
    }

    fn unit(&self, value: String) -> Builder {
        let mut new_builder = self.clone();
        new_builder.unit =  value;
        Builder::new(new_builder)
    }
    fn price(&self, value: String) -> Builder {
        let mut new_builder = self.clone();
        new_builder.price =  value;
        Builder::new(new_builder)
    }
    fn message(&self, value: String) -> Builder {
        let mut new_builder = self.clone();
        new_builder.messages.push(value);
        Builder::new(new_builder)
    }
    fn authorized(&self) -> Builder {
        let mut new_builder = self.clone();
        new_builder.authorized =  true;
        Builder::new(new_builder)
    }
    fn not_authorized(&self) -> Builder {
        let mut new_builder = self.clone();
        new_builder.authorized =  false;
        Builder::new(new_builder)
    }
    fn next_agent(&self, value: AgentIdentification) -> Builder {
        let mut new_builder = self.clone();
        new_builder.next_agent =  value;
        Builder::new(new_builder)
    }

    fn build(&self) -> RatingResponse {
        RatingResponse{
            authorization_status: AuthorizationStatus{
                code: 0,
                key: "".to_string()
            },
            billing_information: BillingInformation{
                messages: vec![],
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

impl Guest for Commons {
    type Builder = MyBuilder;
}