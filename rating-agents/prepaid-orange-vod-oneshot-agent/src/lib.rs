use rating_interface::{Balance, BalanceAccessManager,RatingResponseBuilder, RatingAgent, RatingAgentReceiver, RatingRequest,
    RatingResponse
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;


#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct PrepaidOrangeVodOneshotAgentActor {}

const UNIT_COST: f32 = 2.0;
const OFFER_ID: &str ="4";


#[async_trait]
impl RatingAgent for PrepaidOrangeVodOneshotAgentActor {

    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        info!("Hello I'm your orange prepaid vod One Shot agent");
        let usage: f32 = _arg.usage.parse().unwrap();
        let customer_id = &_arg.customer_id;


        let mut rating_response_builder = RatingResponseBuilder::new();
        let  balance_access_manager = BalanceAccessManager::default();
        

        let balance  = balance_access_manager.get_balance(_ctx, customer_id,OFFER_ID).await?;
        let rating = calc_rate(usage);

        // not depending on the balance <=0  but calc rate and keep sufficient 
        //balance in has_sufficient_balance to centralize the decision..
        // may be we have a case the customer has 0 balance but he still can use the service ...... [to be validated]
        
    
        if has_sufficient_balance(balance.balance_characteristic.count,rating ) {
          
                   
            info!(
                "Usage {} , Rating {} , & balance {}",
                usage, rating, balance.balance_characteristic.count
            );

            let new_balance : Balance =balance_access_manager.withdraw(_ctx,customer_id, OFFER_ID,rating).await?;


            rating_response_builder
            .unit((&"EUR").to_string())
            .price(rating.to_string())
            .message(&format!(
                " {} will be deducted from your balance",
                rating.to_string()
            ))
            .message(&format!(
                " Your Balance now is {} {}",
                new_balance.balance_characteristic.count,
                new_balance.balance_characteristic.unit
            )).authorized();

           
            info!(
                "Usage {} , Rating {} , & balance {}",
                usage, rating, new_balance.balance_characteristic.count
            );
           

        } else {
            rating_response_builder
            .message(&"You have no sufficient balance")
            .not_authorized();
        
        }
       let rating_response =rating_response_builder.build(); 

        RpcResult::Ok(rating_response)
    }



}
fn has_sufficient_balance(balance :f32, charge:f32) -> bool {
    if balance< charge {
        return false;
    }else{
        return true;
    } 
 }

 fn calc_rate(usage:f32) ->f32{
    UNIT_COST * usage
 }
