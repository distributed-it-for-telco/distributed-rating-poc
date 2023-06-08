use rating_interface::{
    RatingAgent, RatingAgentSender, RatingResponse, RatingCoordinator, RatingProcessRequest,RatingCoordinatorReceiver, AgentIdentifiation, ValidationRequest
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor,RatingCoordinator)]
struct RatingAgentCoordinatorActor {}

#[async_trait]
/// Implementation of Rating Coodinator
impl  RatingCoordinator for RatingAgentCoordinatorActor{
    async fn handle_rating_process(&self, _ctx: &Context, _arg: &RatingProcessRequest) -> RpcResult<RatingResponse> {
        info!("Hello I'm your rating coordinator");
        info!("Current used agent is: {}", _arg.rating_request.agent_id);
        info!("Headers {:?}",_arg.headers);

        // let mut next_Agent = Some(AgentIdentifiation {
        //        name: _arg.rating_request.agent_id.to_string(),
        //        partner_id: _arg.rating_request.customer_id.to_string()
        //     });
      
        // let mut valid= true;


        // while valid && next_Agent!=None {
            
        // }



        let mut rating_agent: RatingAgentSender<WasmHost> = RatingAgentSender::to_actor(&format!("agent/{}", _arg.rating_request.agent_id));

        RpcResult::from(match rating_agent.rate_usage(_ctx, &_arg.rating_request).await {
            Ok(rating) => Ok(rating),
            Err(e) => Err(e),
        })
    }
}
