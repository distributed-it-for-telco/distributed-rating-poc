// This file is @generated by wasmcloud/weld-codegen 0.5.0.
// It is not intended for manual editing.
// namespace: co.uk.orange.rating.mock

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
pub struct Customer {
    #[serde(default)]
    pub value: String,
}

// Encode Customer as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_customer<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Customer,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(1)?;
    e.str("value")?;
    e.str(&val.value)?;
    Ok(())
}

// Decode Customer from cbor input stream
#[doc(hidden)]
pub fn decode_customer(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Customer, RpcError> {
    let __result = {
        let mut value: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Customer, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => value = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "value" => value = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        Customer {
            value: if let Some(__x) = value {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Customer.value (#0)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DataItem {
    #[serde(default)]
    pub value: String,
}

// Encode DataItem as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_data_item<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &DataItem,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(1)?;
    e.str("value")?;
    e.str(&val.value)?;
    Ok(())
}

// Decode DataItem from cbor input stream
#[doc(hidden)]
pub fn decode_data_item(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<DataItem, RpcError> {
    let __result = {
        let mut value: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct DataItem, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => value = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "value" => value = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        DataItem {
            value: if let Some(__x) = value {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field DataItem.value (#0)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListOffersRequest {
    #[serde(rename = "partyId")]
    #[serde(default)]
    pub party_id: String,
    #[serde(default)]
    pub vendor: String,
}

// Encode ListOffersRequest as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_list_offers_request<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ListOffersRequest,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("partyId")?;
    e.str(&val.party_id)?;
    e.str("vendor")?;
    e.str(&val.vendor)?;
    Ok(())
}

// Decode ListOffersRequest from cbor input stream
#[doc(hidden)]
pub fn decode_list_offers_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ListOffersRequest, RpcError> {
    let __result = {
        let mut party_id: Option<String> = None;
        let mut vendor: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ListOffersRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => party_id = Some(d.str()?.to_string()),
                    1 => vendor = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "partyId" => party_id = Some(d.str()?.to_string()),
                    "vendor" => vendor = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        ListOffersRequest {
            party_id: if let Some(__x) = party_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ListOffersRequest.party_id (#0)".to_string(),
                ));
            },

            vendor: if let Some(__x) = vendor {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ListOffersRequest.vendor (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OfferDetails {
    #[serde(rename = "agentId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "offerId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
}

// Encode OfferDetails as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_offer_details<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &OfferDetails,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    if let Some(val) = val.agent_id.as_ref() {
        e.str("agentId")?;
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
    Ok(())
}

// Decode OfferDetails from cbor input stream
#[doc(hidden)]
pub fn decode_offer_details(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<OfferDetails, RpcError> {
    let __result = {
        let mut agent_id: Option<Option<String>> = Some(None);
        let mut offer_id: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct OfferDetails, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        agent_id = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    1 => {
                        offer_id = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
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
                    "agentId" => {
                        agent_id = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
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
                    _ => d.skip()?,
                }
            }
        }
        OfferDetails {
            agent_id: agent_id.unwrap(),
            offer_id: offer_id.unwrap(),
        }
    };
    Ok(__result)
}
pub type OffersList = Vec<OfferDetails>;

// Encode OffersList as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_offers_list<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &OffersList,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_offer_details(e, item)?;
    }
    Ok(())
}

// Decode OffersList from cbor input stream
#[doc(hidden)]
pub fn decode_offers_list(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<OffersList, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<OfferDetails> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(decode_offer_details(d).map_err(|e| {
                    format!("decoding 'co.uk.orange.rating.mock#OfferDetails': {}", e)
                })?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<OfferDetails> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(decode_offer_details(d).map_err(|e| {
                        format!("decoding 'co.uk.orange.rating.mock#OfferDetails': {}", e)
                    })?),
                }
            }
            arr
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct UsageProofDetails {
    #[serde(default)]
    pub value: String,
}

// Encode UsageProofDetails as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_usage_proof_details<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &UsageProofDetails,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(1)?;
    e.str("value")?;
    e.str(&val.value)?;
    Ok(())
}

// Decode UsageProofDetails from cbor input stream
#[doc(hidden)]
pub fn decode_usage_proof_details(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<UsageProofDetails, RpcError> {
    let __result = {
        let mut value: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct UsageProofDetails, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => value = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "value" => value = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        UsageProofDetails {
            value: if let Some(__x) = value {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field UsageProofDetails.value (#0)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
pub type UsageProofList = Vec<UsageProofDetails>;

// Encode UsageProofList as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_usage_proof_list<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &UsageProofList,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_usage_proof_details(e, item)?;
    }
    Ok(())
}

// Decode UsageProofList from cbor input stream
#[doc(hidden)]
pub fn decode_usage_proof_list(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<UsageProofList, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<UsageProofDetails> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(decode_usage_proof_details(d).map_err(|e| {
                    format!(
                        "decoding 'co.uk.orange.rating.mock#UsageProofDetails': {}",
                        e
                    )
                })?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<UsageProofDetails> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(decode_usage_proof_details(d).map_err(|e| {
                        format!(
                            "decoding 'co.uk.orange.rating.mock#UsageProofDetails': {}",
                            e
                        )
                    })?),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// Description of the customer inventory mock service
/// wasmbus.actorReceive
#[async_trait]
pub trait CustomerInventoryAgent {
    async fn get_customer<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<Customer>;
}

/// CustomerInventoryAgentReceiver receives messages defined in the CustomerInventoryAgent service trait
/// Description of the customer inventory mock service
#[doc(hidden)]
#[async_trait]
pub trait CustomerInventoryAgentReceiver: MessageDispatch + CustomerInventoryAgent {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        match message.method {
            "GetCustomer" => {
                let value: String = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'String': {}", e)))?;

                let resp = CustomerInventoryAgent::get_customer(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "CustomerInventoryAgent::{}",
                message.method
            ))),
        }
    }
}

/// CustomerInventoryAgentSender sends messages to a CustomerInventoryAgent service
/// Description of the customer inventory mock service
/// client for sending CustomerInventoryAgent messages
#[derive(Debug)]
pub struct CustomerInventoryAgentSender<T: Transport> {
    transport: T,
}

impl<T: Transport> CustomerInventoryAgentSender<T> {
    /// Constructs a CustomerInventoryAgentSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> CustomerInventoryAgentSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl CustomerInventoryAgentSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> CustomerInventoryAgent
    for CustomerInventoryAgentSender<T>
{
    #[allow(unused)]
    async fn get_customer<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<Customer> {
        let buf = wasmbus_rpc::common::serialize(&arg.to_string())?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "CustomerInventoryAgent.GetCustomer",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: Customer = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': Customer", e)))?;
        Ok(value)
    }
}

/// Description of the rating mock service
/// wasmbus.actorReceive
#[async_trait]
pub trait MockAgent {
    async fn seed(&self, ctx: &Context) -> RpcResult<()>;
    async fn get_data_item<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<DataItem>;
}

/// MockAgentReceiver receives messages defined in the MockAgent service trait
/// Description of the rating mock service
#[doc(hidden)]
#[async_trait]
pub trait MockAgentReceiver: MessageDispatch + MockAgent {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        match message.method {
            "Seed" => {
                let _resp = MockAgent::seed(self, ctx).await?;
                let buf = Vec::new();
                Ok(buf)
            }
            "GetDataItem" => {
                let value: String = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'String': {}", e)))?;

                let resp = MockAgent::get_data_item(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "MockAgent::{}",
                message.method
            ))),
        }
    }
}

/// MockAgentSender sends messages to a MockAgent service
/// Description of the rating mock service
/// client for sending MockAgent messages
#[derive(Debug)]
pub struct MockAgentSender<T: Transport> {
    transport: T,
}

impl<T: Transport> MockAgentSender<T> {
    /// Constructs a MockAgentSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> MockAgentSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl MockAgentSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> MockAgent for MockAgentSender<T> {
    #[allow(unused)]
    async fn seed(&self, ctx: &Context) -> RpcResult<()> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "MockAgent.Seed",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        Ok(())
    }
    #[allow(unused)]
    async fn get_data_item<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<DataItem> {
        let buf = wasmbus_rpc::common::serialize(&arg.to_string())?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "MockAgent.GetDataItem",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: DataItem = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': DataItem", e)))?;
        Ok(value)
    }
}

/// Description of the usage collector service
/// wasmbus.actorReceive
#[async_trait]
pub trait UsageCollector {
    async fn store<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<()>;
    async fn list(&self, ctx: &Context) -> RpcResult<UsageProofList>;
}

/// UsageCollectorReceiver receives messages defined in the UsageCollector service trait
/// Description of the usage collector service
#[doc(hidden)]
#[async_trait]
pub trait UsageCollectorReceiver: MessageDispatch + UsageCollector {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        match message.method {
            "Store" => {
                let value: String = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'String': {}", e)))?;

                let _resp = UsageCollector::store(self, ctx, &value).await?;
                let buf = Vec::new();
                Ok(buf)
            }
            "List" => {
                let resp = UsageCollector::list(self, ctx).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "UsageCollector::{}",
                message.method
            ))),
        }
    }
}

/// UsageCollectorSender sends messages to a UsageCollector service
/// Description of the usage collector service
/// client for sending UsageCollector messages
#[derive(Debug)]
pub struct UsageCollectorSender<T: Transport> {
    transport: T,
}

impl<T: Transport> UsageCollectorSender<T> {
    /// Constructs a UsageCollectorSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> UsageCollectorSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl UsageCollectorSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> UsageCollector
    for UsageCollectorSender<T>
{
    #[allow(unused)]
    async fn store<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<()> {
        let buf = wasmbus_rpc::common::serialize(&arg.to_string())?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "UsageCollector.Store",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        Ok(())
    }
    #[allow(unused)]
    async fn list(&self, ctx: &Context) -> RpcResult<UsageProofList> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "UsageCollector.List",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: UsageProofList = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': UsageProofList", e)))?;
        Ok(value)
    }
}
