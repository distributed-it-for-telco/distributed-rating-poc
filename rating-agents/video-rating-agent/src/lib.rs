use rating_interface::*;
use wasmbus_rpc::actor::prelude::*;

const OFFER_ID: &str = "video";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct VideoRatingAgentActor {}

/// Implementation of Rating Agent trait methods
#[async_trait]
impl RatingAgent for VideoRatingAgentActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        info!("Hello I'm your video provider postpaid rating agent");

        let usage_date = "21/06/2023";
        let usage_id: String = generate_guid().await?;

        /*
         *  Contract or Offer is one Movie equal one EURO
         */
        let rating = _arg.usage.parse::<i32>().unwrap() * 1;

        let usage_template_str = UsageProofHandler::generate_rating_proof(&UsageProofRequest {
            party_id: _arg.customer_id.to_owned(),
            rating: rating.to_string(),
            usage: _arg.usage.to_owned(),
            usage_id: usage_id.as_str().to_owned(),
            usage_date: usage_date.to_owned(),
            offer_id: OFFER_ID.to_owned()
        });

        info!(
            "Sending usage proof to usage collector for party with id: {}",
            _arg.customer_id.to_string()
        );

        UsageCollectorSender::to_actor(&format!("mock/{}", "usage_collector"))
            .store(_ctx, &usage_template_str)
            .await?;

        let mut rating_response_builder = RatingResponseBuilder::new();
        
        if _arg.offer_id.is_some() && OFFER_PROVIDERS_OFFERS_IDS_TO_AGENTS.contains_key(_arg.offer_id.to_owned().unwrap().as_str()) {
            let mut next_agent: AgentIdentifiation = AgentIdentifiation::default();

            next_agent.name = OFFER_PROVIDERS_OFFERS_IDS_TO_AGENTS.get(_arg.offer_id.to_owned().unwrap().as_str()).unwrap().to_string();
            next_agent.partner_id = _arg.offer_id.to_owned().unwrap();

            rating_response_builder.next_agent(next_agent);
        }

        let rating_response = rating_response_builder
            .unit((&"EUR").to_string())
            .price(rating.to_string())
            .authorized()
            .build();

        RpcResult::Ok(rating_response)
    }

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

        if arg.rating_request.offer_id.is_some() && OFFER_PROVIDERS_OFFERS_IDS_TO_AGENTS.contains_key(arg.rating_request.offer_id.to_owned().unwrap().as_str()) {
            let mut next_agent: AgentIdentifiation = AgentIdentifiation::default();

            next_agent.name = OFFER_PROVIDERS_OFFERS_IDS_TO_AGENTS.get(arg.rating_request.offer_id.to_owned().unwrap().as_str()).unwrap().to_string();
            next_agent.partner_id = arg.rating_request.offer_id.to_owned().unwrap();

            validation_response.next_agent = Some(next_agent);
        }

        Ok(validation_response)
    }
}
