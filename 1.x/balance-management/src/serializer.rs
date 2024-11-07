use serde::{Serialize, Deserialize};
use crate::orange::commons::types::*;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct BalanceDTO {
    count: f32,
    unit: String,
    party_id: String,
}

impl fmt::Display for BalanceDTO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} (Party ID: {})", self.count, self.unit, self.party_id)
    }
}   

impl From<Balance> for BalanceDTO {
    fn from(balance: Balance) -> Self {
        BalanceDTO {
            count: balance.count,
            unit: balance.unit,
            party_id: balance.party_id,
        }
    }
}

impl From<BalanceDTO> for Balance {
    fn from(dto: BalanceDTO) -> Self {
        Balance {
            count: dto.count,
            unit: dto.unit,
            party_id: dto.party_id,
        }
    }
}