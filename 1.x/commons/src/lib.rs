
wit_bindgen::generate!({
    generate_all
});

pub mod dtos;
pub mod mappers;
mod mappers_impl;
mod some_builders_impl;
mod builders_impl;
mod commons_impl;
mod error_types_impl;
mod commons_struct;
use commons_struct::Commons;


export!(Commons);