
pub mod dto_types {
    use serde::{Serialize, Deserialize};
    use o2o::o2o;
    use crate::orange::ratingagent::types::*;

    #[derive(Clone, Serialize, Deserialize, Copy,o2o)]
    #[from_owned(ValidationResponse)]
    #[owned_try_into(ValidationResponse, std::io::Error)]
    pub struct DTOValidationResponse {
        pub valid: bool,
    }


    #[derive(Clone, Serialize, Deserialize, o2o)]
    #[from_owned(AgentIdentification)]
    #[owned_try_into(AgentIdentification, std::io::Error)]
    pub struct DTOAgentIdentification {
        pub name: String,
        pub partner_id: String,
    }

    #[derive(Clone, Serialize, Deserialize, o2o)]
    #[from_owned(UsageCharacteristic)]
    #[owned_try_into(UsageCharacteristic, std::io::Error)]
    pub struct DTOUsageCharacteristic {
        pub name: String,
        pub value: String,
        pub value_type: String,
    }

    #[derive(Clone, Serialize, Deserialize, o2o)]
    #[from_owned(Usage)]
    #[owned_try_into(Usage, std::io::Error)]
    pub struct DTOUsage {
        pub usage_characteristic_list: Vec<DTOUsageCharacteristic>,
    }

    #[derive(Clone, Serialize, Deserialize, o2o)]
    #[from_owned(GetChildrenRequest)]
    #[owned_try_into(GetChildrenRequest, std::io::Error)]
    pub struct DTOGetChildrenRequest {
        pub usage: DTOUsage,
        pub atomic_offer_id: String,
    }

    #[derive(Clone, Serialize, Deserialize, o2o)]
    #[from_owned(Agent)]
    #[owned_try_into(Agent, std::io::Error)]
    pub struct DTOAgent {
        pub identification: DTOAgentIdentification,
        pub usage: DTOUsage,
    }

    #[derive(Clone, Serialize, Deserialize, o2o)]
    #[from_owned(AgentList)]
    #[owned_try_into(AgentList, std::io::Error)]
    pub struct DTOAgentList {
        pub agents: Vec<DTOAgent>,
    }

    #[derive(Clone, Serialize, Deserialize, o2o)]
    #[from_owned(RatingRecord)]
    #[owned_try_into(RatingRecord, std::io::Error)]
    pub struct DTORatingRecord {
        pub producer: String,
        pub unit: String,
        pub price: String,
    }

    #[derive(Clone, Serialize, Deserialize, o2o)]
    #[from_owned(RatingRequest)]
    #[owned_try_into(RatingRequest, std::io::Error)]
    pub struct DTORatingRequest {
        pub customer_id: String,
        pub agent_id: String,
        pub language: String,
        pub offer_id: String,
        #[map(~.into())]
        pub usage: DTOUsage,
        pub rating_history: Vec<DTORatingRecord>,
    }

    #[derive(Clone, Serialize, Deserialize, o2o)]
    #[from_owned(ValidationRequest)]
    #[owned_try_into(ValidationRequest, std::io::Error)]
    pub struct DTOValidationRequest {
        #[map(~.into())]
        pub rating_request: DTORatingRequest,
        pub client_ip: String,
        pub client_country: String,
    }

    #[derive(Clone, Serialize, Deserialize, o2o)]
    #[from_owned(BillingInformation)]
    #[owned_try_into(BillingInformation, std::io::Error)]
    pub struct DTOBillingInformation {
        pub price: String,
        pub unit: String,
        #[map(messages, ~.iter().map(|p|p.into()).collect())]
        pub messages: Vec<String>,
    }

    #[derive(Clone, Serialize, Deserialize, o2o)]
    #[from_owned(AuthorizationStatus)]
    #[owned_try_into(AuthorizationStatus, std::io::Error)]
    pub struct DTOAuthorizationStatus {
        pub code: u16,
        pub key: String,
    }

    #[derive(Clone, Serialize, Deserialize, o2o)]
    #[from_owned(RatingResponse)]BillingInformation
    #[owned_try_into(RatingResponse, std::io::Error)]
    pub struct DTORatingResponse {
        #[map(~.into())]
        pub authorization_status: DTOAuthorizationStatus,
        #[map(~.into())]
        pub billing_information: DTOBillingInformation,
        #[map(~.into())]
        pub next_agent: DTOAgentIdentification,
    }

}
