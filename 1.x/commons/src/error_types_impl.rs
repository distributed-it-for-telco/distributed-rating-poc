use crate::commons_struct::Commons;
use crate::exports::orange::commons::error_types::*;

impl Guest for Commons{
    fn set_validation_error(_s: ValidationError){

    }
    fn set_usage_error(_s: UsageError){

    }
    fn set_other_error(_s: OtherError){

    }
}