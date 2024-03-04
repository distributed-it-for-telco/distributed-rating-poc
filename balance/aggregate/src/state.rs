use crate::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct BalanceAggregateState {
    pub balance: u32, // CENTS
    pub party_id: String,
}

impl BalanceAggregateState {
    /// Returns the regular balance
    pub fn available_balance(&self) -> u32 {
        self.balance
    }

    /// Deducts a given amount
    pub fn deduct(self, amount: u32) -> Self {
        let mut new_state = self.clone();
        new_state.balance = new_state.balance.checked_sub(amount).unwrap_or(0);
        new_state
    }

    /// Deposits a given amount. Ceilings at u32::MAX
    pub fn deposit(self, amount: u32) -> Self {
        let mut new_state = self.clone();
        new_state.balance = new_state
            .balance
            .checked_add(amount)
            .unwrap_or(new_state.balance);
        new_state
    }
}
