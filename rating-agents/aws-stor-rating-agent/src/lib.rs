use rating_interface::{
    AgentIdentifiation, RatingAgent, RatingAgentReceiver, RatingRequest, RatingResponse,
    RatingResponseBuilder, UsageCollector, UsageCollectorSender, UsageProofHandler,
    UsageProofRequest, ValidationRequest, ValidationResponse,
};
use wasmbus_rpc::actor::prelude::*;

const OFFER_ID: &str = "100";
const DROPBOX_PARTY_ID_AT_PARTNER_SIDE: &str = "dropbox_my_partner";
const PROVIDER_AGENT_NAME: &str = "aws_stor";
const REPLICATION_FACTOR: u32 = 2;
const RATE_FEE: f64 = ;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct AwsStorRatingAgentActor {}

#[async_trait]
impl RatingAgent for AwsStorRatingAgentActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        todo!()
    }

    /// Validate 
    /// 1- apply business validation.
    /// 2- translate usage.
    /// 3- return validation status and tranlated usage.
    async fn validate(
        &self,
        ctx: &Context,
        arg: &ValidationRequest,
    ) -> RpcResult<ValidationResponse> {
        let mut validation_response: ValidationResponse = ValidationResponse::default();

        if arg.client_country.is_some() && arg.client_country.to_owned().unwrap() == "EG" {
            validation_response.valid = true;
        } else {
            validation_response.valid = false;
        }


        let mut next_agent: AgentIdentifiation = AgentIdentifiation::default();

        next_agent.name = PROVIDER_AGENT_NAME.to_string();
        next_agent.partner_id = DROPBOX_PARTY_ID_AT_PARTNER_SIDE.to_string();

        validation_response.next_agent = Some(next_agent);

        validation_response.translated_usage = Some(arg.rating_request.usage.to_owned());
        Ok(validation_response)
    }
}


