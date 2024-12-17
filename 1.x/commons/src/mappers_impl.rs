use crate::commons_struct::Commons;
use crate::exports::orange::commons::mappers::Guest as Mappers;
use crate::orange::commons::types::*;
use crate::mappers;

impl Mappers for Commons{
    fn rating_request_to_string(request: RatingRequest) -> String{
        mappers::rating_request_to_string(request)
    }
    
    fn string_to_rating_request(value: String) -> RatingRequest{
        mappers::string_to_rating_request(value)
    }

    fn rating_response_to_string(request: RatingResponse) -> String{
        mappers::rating_response_to_string(request)
    }

    fn string_to_rating_response(value: String) -> RatingResponse{
        mappers::string_to_rating_response(value)
    }

    fn balance_to_string(value: Balance) -> String{
        mappers::balance_to_string(value)
    }

    fn string_to_balance(value: String) -> Balance{
        mappers::string_to_balance(value)
    }
 fn balance_to_stringified_array(value: Balance) -> Vec<u8>{
        mappers::balance_to_stringified_array(value)
    }
}

