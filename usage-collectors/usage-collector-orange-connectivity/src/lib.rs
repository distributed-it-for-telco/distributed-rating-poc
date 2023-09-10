use rating_interface::*;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::*;
use wasmcloud_interface_logging::info;


#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, UsageCollector)]
struct UsageCollectorOrangeConnectivityActor {}

const USAGE_LIST_KEY: &str = "rating:usage:orange:connectivity";


/// Implementation of the HttpServer capability contract
#[async_trait]
impl UsageCollector for UsageCollectorOrangeConnectivityActor {
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

    async fn list(&self, ctx: &Context) -> RpcResult<UsageProofList> {
        let kv = KeyValueSender::new();
        info!("Retrieving rating usage proofs for Orange connectivity");
        let res = kv
            .list_range(
                ctx,
                &ListRangeRequest {
                    list_name: USAGE_LIST_KEY.to_string(),
                    start: 0,
                    stop: 100000
                },
            )
            .await
            .map(|res: Vec<String>| {
                res.iter()
                    .filter_map(|s| match serde_json::from_str(s.as_str()) {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    })
                    .collect::<UsageProofList>()
            });

        Ok(match res {
            Ok(v) => v,
            Err(_) => vec![],
        })
    }
}

