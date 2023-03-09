use wasmbus_rpc::actor::prelude::*;
use rating_interface::{
    MockAgent, MockAgentSender, MockAgentReceiver
};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, MockAgent)]
struct OrangeCustomerProductInventoryActor {}

impl OrangeCustomerProductInventoryActor {
}

/// Implementation of Rating Mock trait methods
#[async_trait]
impl MockAgent for OrangeCustomerProductInventoryActor {
    async fn seed(&self, ctx: &Context) -> RpcResult<()> {
        Ok(())
    }
}

