use crate::orange::commons::types::*;
use crate::dtos::{SerializedRatingRequest, SerializedRatingResponse, SerializedBalance};

pub fn rating_request_to_string(request: RatingRequest) -> String{
    let serialized_rating_result: SerializedRatingRequest = request.into();
    serde_json::to_string(&serialized_rating_result).unwrap()
}

pub fn string_to_rating_request(value: String) -> RatingRequest{
    let serialized_rating_request: SerializedRatingRequest =
        serde_json::from_str::<SerializedRatingRequest>(&value).unwrap().into();
        serialized_rating_request.into()
}

pub fn rating_response_to_string(request: RatingResponse) -> String{
    let serialized_rating_result: SerializedRatingResponse = request.into();
    serde_json::to_string(&serialized_rating_result).unwrap()
}

pub fn string_to_rating_response(value: String) -> RatingResponse{
    let serialized_rating_response: SerializedRatingResponse =
        serde_json::from_str::<SerializedRatingResponse>(&value).unwrap().into();
        serialized_rating_response.into()
}

pub fn balance_to_string(value: Balance) -> String{
    let serialized_balance: SerializedBalance = value.into();
    serde_json::to_string(&serialized_balance).unwrap()
}

pub fn string_to_balance(value: String) -> Balance{
    let serialized_balance: SerializedBalance =
    serde_json::from_str::<SerializedBalance>(&value).unwrap().into();
    serialized_balance.into()
}