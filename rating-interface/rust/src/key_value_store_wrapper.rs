use wasmbus_rpc::{actor::prelude::RpcResult, common::Context};
use wasmcloud_interface_keyvalue::{KeyValue, KeyValueSender, SetRequest};

#[derive(Clone, Debug, Default)]
pub struct KeyValueStoreWrapper {}

impl KeyValueStoreWrapper {
    pub async fn put(_ctx: &Context, key: &str, value: &str) -> RpcResult<()> {
        let kv = KeyValueSender::new();
        kv.set(
            _ctx,
            &SetRequest {
                key: key.to_string(),
                value: value.to_string(),
                expires: 0,
            },
        )
        .await?;
        Ok(())
    }

    pub async fn get(_ctx: &Context, key: &str) -> RpcResult<String> {
        let kv = KeyValueSender::new();
        let value_json_str = kv.get(_ctx, &key).await?.value;

        Ok(value_json_str)
    }
}
