use rating_interface::*;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::*;
use wasmcloud_interface_logging::info;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, UsageCollector)]
struct UsageCollectorDropboxActor {}

const USAGE_LIST_KEY: &str = "rating:usage:dropbox";

#[async_trait]
impl UsageCollector for UsageCollectorDropboxActor {
    async fn store<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<()> {
        let kv = KeyValueSender::new();
        info!("Storing rating usage log message '{}'", arg.to_string());
        let _ = kv
            .list_add(
                ctx,
                &ListAddRequest {
                    list_name: USAGE_LIST_KEY.to_string(),
                    value: arg.to_string(),
                },
            )
            .await?;

        Ok(())
    }
}

