use crate::{AuthorizationStatus, BillingInformation, RatingResponse};

#[derive(Debug, PartialEq, Default /*,Copy */, Clone)]
pub struct RatingResponseBuilder {
    // Probably lots of optional fields.
    authorized: bool,
    authorization_message: String,
    unit: String,
    price: String,
    messages: Vec<String>,
}

// impl Copy for RatingResponseBuilder{

//     fn copy (&self) ->self {

//     }
// }

impl RatingResponseBuilder {
    pub fn new() -> RatingResponseBuilder {
        RatingResponseBuilder::default()
    }

    pub fn unit(&mut self, unit: String) -> &mut Self {
        self.unit = unit;
        self
    }
    pub fn price(&mut self, price: String) -> &mut Self {
        self.price = price;
        self
    }
    pub fn message(&mut self, message: &str) -> &mut Self {
        self.messages.push(message.to_string());
        self
    }
    pub fn authorized(&mut self) -> &mut Self {
        self.authorized = true;
        self.authorization_message = "This user is authorized to use this service".to_string();
        self
    }
    pub fn not_authorized(&mut self) -> &mut Self {
        self.authorized = false;
        self.authorization_message = "This user is not authorized to use this service".to_string();
        self
    }

    pub fn build(&mut self) -> RatingResponse {
        let mut billing_info = BillingInformation::default();
        let mut authorization_status = AuthorizationStatus::default();
        billing_info.unit = self.unit.to_string();
        billing_info.price = self.price.to_string();

        for message in self.messages.iter() {
            billing_info.messages.push(message.to_string());
        }

        if self.authorized {
            authorization_status.code = 200;
        } else {
            authorization_status.code = 401;
        }
        authorization_status.key = Some(self.authorization_message.to_string());

        RatingResponse {
            authorization_status: authorization_status,
            billing_information: billing_info,
        }
    }
}
