// This file is @generated by wasmcloud/weld-codegen 0.5.0.
// It is not intended for manual editing.
// namespace: co.uk.orange.rating.agent

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthorizationStatus {
    #[serde(default)]
    pub code: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

// Encode AuthorizationStatus as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_authorization_status<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &AuthorizationStatus,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("code")?;
    e.u16(val.code)?;
    if let Some(val) = val.key.as_ref() {
        e.str("key")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode AuthorizationStatus from cbor input stream
#[doc(hidden)]
pub fn decode_authorization_status(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<AuthorizationStatus, RpcError> {
    let __result = {
        let mut code: Option<u16> = None;
        let mut key: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct AuthorizationStatus, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => code = Some(d.u16()?),
                    1 => {
                        key = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "code" => code = Some(d.u16()?),
                    "key" => {
                        key = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        AuthorizationStatus {
            code: if let Some(__x) = code {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field AuthorizationStatus.code (#0)".to_string(),
                ));
            },
            key: key.unwrap(),
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct BillingInformation {
    pub messages: MessageList,
    #[serde(default)]
    pub price: String,
    #[serde(default)]
    pub unit: String,
}

// Encode BillingInformation as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_billing_information<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &BillingInformation,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(3)?;
    e.str("messages")?;
    encode_message_list(e, &val.messages)?;
    e.str("price")?;
    e.str(&val.price)?;
    e.str("unit")?;
    e.str(&val.unit)?;
    Ok(())
}

// Decode BillingInformation from cbor input stream
#[doc(hidden)]
pub fn decode_billing_information(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<BillingInformation, RpcError> {
    let __result = {
        let mut messages: Option<MessageList> = None;
        let mut price: Option<String> = None;
        let mut unit: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct BillingInformation, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        messages = Some(decode_message_list(d).map_err(|e| {
                            format!("decoding 'co.uk.orange.rating.agent#MessageList': {}", e)
                        })?)
                    }
                    1 => price = Some(d.str()?.to_string()),
                    2 => unit = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "messages" => {
                        messages = Some(decode_message_list(d).map_err(|e| {
                            format!("decoding 'co.uk.orange.rating.agent#MessageList': {}", e)
                        })?)
                    }
                    "price" => price = Some(d.str()?.to_string()),
                    "unit" => unit = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        BillingInformation {
            messages: if let Some(__x) = messages {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field BillingInformation.messages (#0)".to_string(),
                ));
            },

            price: if let Some(__x) = price {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field BillingInformation.price (#1)".to_string(),
                ));
            },

            unit: if let Some(__x) = unit {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field BillingInformation.unit (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Bucket {
    #[serde(rename = "bucketCharacteristic")]
    pub bucket_characteristic: BucketCharacteristic,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "offerId")]
    #[serde(default)]
    pub offer_id: String,
    #[serde(rename = "partyId")]
    #[serde(default)]
    pub party_id: String,
}

// Encode Bucket as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_bucket<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Bucket,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(4)?;
    e.str("bucketCharacteristic")?;
    encode_bucket_characteristic(e, &val.bucket_characteristic)?;
    e.str("name")?;
    e.str(&val.name)?;
    e.str("offerId")?;
    e.str(&val.offer_id)?;
    e.str("partyId")?;
    e.str(&val.party_id)?;
    Ok(())
}

// Decode Bucket from cbor input stream
#[doc(hidden)]
pub fn decode_bucket(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Bucket, RpcError> {
    let __result =
        {
            let mut bucket_characteristic: Option<BucketCharacteristic> = None;
            let mut name: Option<String> = None;
            let mut offer_id: Option<String> = None;
            let mut party_id: Option<String> = None;

            let is_array = match d.datatype()? {
                wasmbus_rpc::cbor::Type::Array => true,
                wasmbus_rpc::cbor::Type::Map => false,
                _ => {
                    return Err(RpcError::Deser(
                        "decoding struct Bucket, expected array or map".to_string(),
                    ))
                }
            };
            if is_array {
                let len = d.fixed_array()?;
                for __i in 0..(len as usize) {
                    match __i {
                        0 => bucket_characteristic =
                            Some(decode_bucket_characteristic(d).map_err(|e| {
                                format!(
                                    "decoding 'co.uk.orange.rating.agent#BucketCharacteristic': {}",
                                    e
                                )
                            })?),
                        1 => name = Some(d.str()?.to_string()),
                        2 => offer_id = Some(d.str()?.to_string()),
                        3 => party_id = Some(d.str()?.to_string()),
                        _ => d.skip()?,
                    }
                }
            } else {
                let len = d.fixed_map()?;
                for __i in 0..(len as usize) {
                    match d.str()? {
                        "bucketCharacteristic" => bucket_characteristic =
                            Some(decode_bucket_characteristic(d).map_err(|e| {
                                format!(
                                    "decoding 'co.uk.orange.rating.agent#BucketCharacteristic': {}",
                                    e
                                )
                            })?),
                        "name" => name = Some(d.str()?.to_string()),
                        "offerId" => offer_id = Some(d.str()?.to_string()),
                        "partyId" => party_id = Some(d.str()?.to_string()),
                        _ => d.skip()?,
                    }
                }
            }
            Bucket {
                bucket_characteristic: if let Some(__x) = bucket_characteristic {
                    __x
                } else {
                    return Err(RpcError::Deser(
                        "missing field Bucket.bucket_characteristic (#0)".to_string(),
                    ));
                },

                name: if let Some(__x) = name {
                    __x
                } else {
                    return Err(RpcError::Deser(
                        "missing field Bucket.name (#1)".to_string(),
                    ));
                },

                offer_id: if let Some(__x) = offer_id {
                    __x
                } else {
                    return Err(RpcError::Deser(
                        "missing field Bucket.offer_id (#2)".to_string(),
                    ));
                },

                party_id: if let Some(__x) = party_id {
                    __x
                } else {
                    return Err(RpcError::Deser(
                        "missing field Bucket.party_id (#3)".to_string(),
                    ));
                },
            }
        };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct BucketCharacteristic {
    #[serde(default)]
    pub count: u16,
    #[serde(default)]
    pub unit: String,
}

// Encode BucketCharacteristic as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_bucket_characteristic<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &BucketCharacteristic,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("count")?;
    e.u16(val.count)?;
    e.str("unit")?;
    e.str(&val.unit)?;
    Ok(())
}

// Decode BucketCharacteristic from cbor input stream
#[doc(hidden)]
pub fn decode_bucket_characteristic(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<BucketCharacteristic, RpcError> {
    let __result = {
        let mut count: Option<u16> = None;
        let mut unit: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct BucketCharacteristic, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => count = Some(d.u16()?),
                    1 => unit = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "count" => count = Some(d.u16()?),
                    "unit" => unit = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        BucketCharacteristic {
            count: if let Some(__x) = count {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field BucketCharacteristic.count (#0)".to_string(),
                ));
            },

            unit: if let Some(__x) = unit {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field BucketCharacteristic.unit (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
pub type MessageList = Vec<String>;

// Encode MessageList as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_message_list<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &MessageList,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(val.len() as u64)?;
    for item in val.iter() {
        e.str(item)?;
    }
    Ok(())
}

// Decode MessageList from cbor input stream
#[doc(hidden)]
pub fn decode_message_list(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<MessageList, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<String> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(d.str()?.to_string())
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<String> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(d.str()?.to_string()),
                }
            }
            arr
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RatingRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "customerId")]
    #[serde(default)]
    pub customer_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "offerId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(default)]
    pub usage: String,
}

// Encode RatingRequest as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_rating_request<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &RatingRequest,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(5)?;
    e.str("agentId")?;
    e.str(&val.agent_id)?;
    e.str("customerId")?;
    e.str(&val.customer_id)?;
    if let Some(val) = val.language.as_ref() {
        e.str("language")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.offer_id.as_ref() {
        e.str("offerId")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    e.str("usage")?;
    e.str(&val.usage)?;
    Ok(())
}

// Decode RatingRequest from cbor input stream
#[doc(hidden)]
pub fn decode_rating_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<RatingRequest, RpcError> {
    let __result = {
        let mut agent_id: Option<String> = None;
        let mut customer_id: Option<String> = None;
        let mut language: Option<Option<String>> = Some(None);
        let mut offer_id: Option<Option<String>> = Some(None);
        let mut usage: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct RatingRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => agent_id = Some(d.str()?.to_string()),
                    1 => customer_id = Some(d.str()?.to_string()),
                    2 => {
                        language = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    3 => {
                        offer_id = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    4 => usage = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "agentId" => agent_id = Some(d.str()?.to_string()),
                    "customerId" => customer_id = Some(d.str()?.to_string()),
                    "language" => {
                        language = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "offerId" => {
                        offer_id = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "usage" => usage = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        RatingRequest {
            agent_id: if let Some(__x) = agent_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field RatingRequest.agent_id (#0)".to_string(),
                ));
            },

            customer_id: if let Some(__x) = customer_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field RatingRequest.customer_id (#1)".to_string(),
                ));
            },
            language: language.unwrap(),
            offer_id: offer_id.unwrap(),

            usage: if let Some(__x) = usage {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field RatingRequest.usage (#4)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RatingResponse {
    #[serde(rename = "authorizationStatus")]
    pub authorization_status: AuthorizationStatus,
    #[serde(rename = "billingInformation")]
    pub billing_information: BillingInformation,
}

// Encode RatingResponse as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_rating_response<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &RatingResponse,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("authorizationStatus")?;
    encode_authorization_status(e, &val.authorization_status)?;
    e.str("billingInformation")?;
    encode_billing_information(e, &val.billing_information)?;
    Ok(())
}

// Decode RatingResponse from cbor input stream
#[doc(hidden)]
pub fn decode_rating_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<RatingResponse, RpcError> {
    let __result =
        {
            let mut authorization_status: Option<AuthorizationStatus> = None;
            let mut billing_information: Option<BillingInformation> = None;

            let is_array = match d.datatype()? {
                wasmbus_rpc::cbor::Type::Array => true,
                wasmbus_rpc::cbor::Type::Map => false,
                _ => {
                    return Err(RpcError::Deser(
                        "decoding struct RatingResponse, expected array or map".to_string(),
                    ))
                }
            };
            if is_array {
                let len = d.fixed_array()?;
                for __i in 0..(len as usize) {
                    match __i {
                        0 => authorization_status =
                            Some(decode_authorization_status(d).map_err(|e| {
                                format!(
                                    "decoding 'co.uk.orange.rating.agent#AuthorizationStatus': {}",
                                    e
                                )
                            })?),
                        1 => billing_information =
                            Some(decode_billing_information(d).map_err(|e| {
                                format!(
                                    "decoding 'co.uk.orange.rating.agent#BillingInformation': {}",
                                    e
                                )
                            })?),
                        _ => d.skip()?,
                    }
                }
            } else {
                let len = d.fixed_map()?;
                for __i in 0..(len as usize) {
                    match d.str()? {
                        "authorizationStatus" => authorization_status =
                            Some(decode_authorization_status(d).map_err(|e| {
                                format!(
                                    "decoding 'co.uk.orange.rating.agent#AuthorizationStatus': {}",
                                    e
                                )
                            })?),
                        "billingInformation" => billing_information =
                            Some(decode_billing_information(d).map_err(|e| {
                                format!(
                                    "decoding 'co.uk.orange.rating.agent#BillingInformation': {}",
                                    e
                                )
                            })?),
                        _ => d.skip()?,
                    }
                }
            }
            RatingResponse {
                authorization_status: if let Some(__x) = authorization_status {
                    __x
                } else {
                    return Err(RpcError::Deser(
                        "missing field RatingResponse.authorization_status (#0)".to_string(),
                    ));
                },

                billing_information: if let Some(__x) = billing_information {
                    __x
                } else {
                    return Err(RpcError::Deser(
                        "missing field RatingResponse.billing_information (#1)".to_string(),
                    ));
                },
            }
        };
    Ok(__result)
}
/// Description of the rating agent service
/// wasmbus.actorReceive
#[async_trait]
pub trait RatingAgent {
    async fn rate_usage(&self, ctx: &Context, arg: &RatingRequest) -> RpcResult<RatingResponse>;
}

/// RatingAgentReceiver receives messages defined in the RatingAgent service trait
/// Description of the rating agent service
#[doc(hidden)]
#[async_trait]
pub trait RatingAgentReceiver: MessageDispatch + RatingAgent {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        match message.method {
            "RateUsage" => {
                let value: RatingRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'RatingRequest': {}", e)))?;

                let resp = RatingAgent::rate_usage(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "RatingAgent::{}",
                message.method
            ))),
        }
    }
}

/// RatingAgentSender sends messages to a RatingAgent service
/// Description of the rating agent service
/// client for sending RatingAgent messages
#[derive(Debug)]
pub struct RatingAgentSender<T: Transport> {
    transport: T,
}

impl<T: Transport> RatingAgentSender<T> {
    /// Constructs a RatingAgentSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> RatingAgentSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl RatingAgentSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> RatingAgent for RatingAgentSender<T> {
    #[allow(unused)]
    async fn rate_usage(&self, ctx: &Context, arg: &RatingRequest) -> RpcResult<RatingResponse> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "RatingAgent.RateUsage",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: RatingResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': RatingResponse", e)))?;
        Ok(value)
    }
}
