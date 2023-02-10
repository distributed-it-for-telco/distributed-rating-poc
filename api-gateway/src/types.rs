// Types used for exchange of data over public internet for request/response pairs with
// customer devices using the rating SDK

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A request for authorization to consume the amount and type of service
/// as indicated by the usage data field
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ServiceUsageRequest {
    pub customer_id: String,
    pub language: Option<String>,
    pub offer_id: Option<String>,
    // Arbitrary JSON payload indicating the details of the usage request
    pub usage_data: Value,
}

/// The response from a request to authorize rated usage. Indicates
/// the status of the authorization (which can be denial), billing data,
/// and a list of approval data
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ServiceUsageResponse {
    pub authorization_status: AuthorizationStatus,
    pub billing_information: BillingInformation,
    pub request_approvals: Vec<RequestApproval>,
}

/// A request to record rated usage (past tense) from a device using the rating SDK.
/// Indicates consumed usage and can therefore exceed previously authorized values. Must
/// also submit the authorization key from a valid service usage response to indicate that
/// the usage has been vetted
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ServiceUsageRatingRequest {
    pub customer_id: String,
    pub language: Option<String>,
    pub offer_id: Option<String>,
    pub authorization_key: String,
    pub effective_usage: Value,
}

/// A reply to a ServiceUsageRatingRequest
#[derive(Debug, Serialize, Deserialize, Clone, Default)]

pub struct ServiceUsageRatingResponse {
    pub authorization_status: AuthorizationStatus,
    pub billing_information: BillingInformation,
}

/// Billing data returned in a service usage authorization response
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BillingInformation {
    pub price: u32,
    pub unit: PriceUnit,
    pub messages: Vec<String>,
}

/// Authorization status returned in response to a service usage request
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AuthorizationStatus {
    status: AuthorizationStatusType,
    key: String,
    error_messages: Vec<String>,
}

/// The type of authorization status. Defaults to None so that absence of data on the wire
/// cannot be misconstrued as denial or grant.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum AuthorizationStatusType {
    #[default]
    None,
    Denied,
    Granted,
    StatusOne,
    StatusTwo,
}

/// Monetary unit used in billing data
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum PriceUnit {
    #[default]
    GBP,
    USD,
    EUR,
}

/// An approval indicator from a request. A list of these are returned in response to a request
/// for service usage
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RequestApproval {
    pub approval_type: String,
    pub data: HashMap<String, String>,
}
