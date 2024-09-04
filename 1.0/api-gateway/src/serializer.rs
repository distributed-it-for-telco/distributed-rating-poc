use serde::{Serialize, Deserialize};
use crate::orange::ratingagent::types::*;

// -----------------------------------------------------------------------------
// Serialization Structs
// -----------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub struct SerializedRatingRequest {
    #[serde(rename = "customer-id")]
    customer_id: String,
    #[serde(rename = "agent-id")]
    agent_id: String,
    language: String,
    #[serde(rename = "offer-id")]
    offer_id: String,
    usage: SerializedUsage,
    #[serde(rename = "rating-history")]
    rating_history: Vec<SerializedRatingRecord>,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedRatingResponse {
    #[serde(rename = "authorization-status")]
    authorization_status: SerializedAuthorizationStatus,
    #[serde(rename = "billing-information")]
    billing_information: SerializedBillingInformation,
    #[serde(rename = "next-agent")]
    next_agent: SerializedAgentIdentification,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedValidationRequest {
    #[serde(rename = "rating-request")]
    rating_request: SerializedRatingRequest,
    #[serde(rename = "client-ip")]
    client_ip: String,
    #[serde(rename = "client-country")]
    client_country: String,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedValidationResponse {
    valid: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedGetChildrenRequest {
    usage: SerializedUsage,
    #[serde(rename = "atomic-offer-id")]
    atomic_offer_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedAgentList {
    agents: Vec<SerializedAgent>,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedAgent {
    identification: SerializedAgentIdentification,
    usage: SerializedUsage,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedAgentIdentification {
    name: String,
    #[serde(rename = "partner-id")]
    partner_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedUsage {
    #[serde(rename = "usage-characteristic-list")]
    usage_characteristic_list: Vec<SerializedUsageCharacteristic>,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedUsageCharacteristic {
    name: String,
    value: String,
    #[serde(rename = "value-type")]
    value_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedRatingRecord {
    producer: String,
    unit: String,
    price: String,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedBillingInformation {
    price: String,
    unit: String,
    messages: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedAuthorizationStatus {
    code: u16,
    key: String,
}

// -----------------------------------------------------------------------------
// From Implementations for Serialization
// -----------------------------------------------------------------------------

impl From<RatingRequest> for SerializedRatingRequest {
    fn from(req: RatingRequest) -> Self {
        Self {
            customer_id: req.customer_id,
            agent_id: req.agent_id,
            language: req.language,
            offer_id: req.offer_id,
            usage: req.usage.into(),
            rating_history: req.rating_history.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<RatingResponse> for SerializedRatingResponse {
    fn from(res: RatingResponse) -> Self {
        Self {
            authorization_status: res.authorization_status.into(),
            billing_information: res.billing_information.into(),
            next_agent: res.next_agent.into(),
        }
    }
}

impl From<ValidationRequest> for SerializedValidationRequest {
    fn from(req: ValidationRequest) -> Self {
        Self {
            rating_request: req.rating_request.into(),
            client_ip: req.client_ip,
            client_country: req.client_country,
        }
    }
}

impl From<ValidationResponse> for SerializedValidationResponse {
    fn from(res: ValidationResponse) -> Self {
        Self { valid: res.valid }
    }
}

impl From<GetChildrenRequest> for SerializedGetChildrenRequest {
    fn from(req: GetChildrenRequest) -> Self {
        Self {
            usage: req.usage.into(),
            atomic_offer_id: req.atomic_offer_id,
        }
    }
}

impl From<AgentList> for SerializedAgentList {
    fn from(list: AgentList) -> Self {
        Self {
            agents: list.agents.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<Agent> for SerializedAgent {
    fn from(agent: Agent) -> Self {
        Self {
            identification: agent.identification.into(),
            usage: agent.usage.into(),
        }
    }
}

impl From<AgentIdentification> for SerializedAgentIdentification {
    fn from(id: AgentIdentification) -> Self {
        Self {
            name: id.name,
            partner_id: id.partner_id,
        }
    }
}

impl From<Usage> for SerializedUsage {
    fn from(usage: Usage) -> Self {
        Self {
            usage_characteristic_list: usage
                .usage_characteristic_list
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<UsageCharacteristic> for SerializedUsageCharacteristic {
    fn from(uc: UsageCharacteristic) -> Self {
        Self {
            name: uc.name,
            value: uc.value,
            value_type: uc.value_type,
        }
    }
}

impl From<RatingRecord> for SerializedRatingRecord {
    fn from(record: RatingRecord) -> Self {
        Self {
            producer: record.producer,
            unit: record.unit,
            price: record.price,
        }
    }
}

impl From<BillingInformation> for SerializedBillingInformation {
    fn from(info: BillingInformation) -> Self {
        Self {
            price: info.price,
            unit: info.unit,
            messages: info.messages,
        }
    }
}

impl From<AuthorizationStatus> for SerializedAuthorizationStatus {
    fn from(status: AuthorizationStatus) -> Self {
        Self {
            code: status.code,
            key: status.key,
        }
    }
}

// -----------------------------------------------------------------------------
// From Implementations for Deserialization
// -----------------------------------------------------------------------------

impl From<SerializedRatingRequest> for RatingRequest {
    fn from(req: SerializedRatingRequest) -> Self {
        Self {
            customer_id: req.customer_id,
            agent_id: req.agent_id,
            language: req.language,
            offer_id: req.offer_id,
            usage: req.usage.into(),
            rating_history: req.rating_history.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<SerializedRatingResponse> for RatingResponse {
    fn from(res: SerializedRatingResponse) -> Self {
        Self {
            authorization_status: res.authorization_status.into(),
            billing_information: res.billing_information.into(),
            next_agent: res.next_agent.into(),
        }
    }
}

impl From<SerializedValidationRequest> for ValidationRequest {
    fn from(req: SerializedValidationRequest) -> Self {
        Self {
            rating_request: req.rating_request.into(),
            client_ip: req.client_ip,
            client_country: req.client_country,
        }
    }
}

impl From<SerializedValidationResponse> for ValidationResponse {
    fn from(res: SerializedValidationResponse) -> Self {
        Self { valid: res.valid }
    }
}

impl From<SerializedGetChildrenRequest> for GetChildrenRequest {
    fn from(req: SerializedGetChildrenRequest) -> Self {
        Self {
            usage: req.usage.into(),
            atomic_offer_id: req.atomic_offer_id,
        }
    }
}

impl From<SerializedAgentList> for AgentList {
    fn from(list: SerializedAgentList) -> Self {
        Self {
            agents: list.agents.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<SerializedAgent> for Agent {
    fn from(agent: SerializedAgent) -> Self {
        Self {
            identification: agent.identification.into(),
            usage: agent.usage.into(),
        }
    }
}

impl From<SerializedAgentIdentification> for AgentIdentification {
    fn from(id: SerializedAgentIdentification) -> Self {
        Self {
            name: id.name,
            partner_id: id.partner_id,
        }
    }
}

impl From<SerializedUsage> for Usage {
    fn from(usage: SerializedUsage) -> Self {
        Self {
            usage_characteristic_list: usage
                .usage_characteristic_list
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<SerializedUsageCharacteristic> for UsageCharacteristic {
    fn from(uc: SerializedUsageCharacteristic) -> Self {
        Self {
            name: uc.name,
            value: uc.value,
            value_type: uc.value_type,
        }
    }
}

impl From<SerializedRatingRecord> for RatingRecord {
    fn from(record: SerializedRatingRecord) -> Self {
        Self {
            producer: record.producer,
            unit: record.unit,
            price: record.price,
        }
    }
}

impl From<SerializedBillingInformation> for BillingInformation {
    fn from(info: SerializedBillingInformation) -> Self {
        Self {
            price: info.price,
            unit: info.unit,
            messages: info.messages,
        }
    }
}

impl From<SerializedAuthorizationStatus> for AuthorizationStatus {
    fn from(status: SerializedAuthorizationStatus) -> Self {
        Self {
            code: status.code,
            key: status.key,
        }
    }
}
