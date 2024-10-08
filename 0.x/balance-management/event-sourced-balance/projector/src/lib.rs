use anyhow::Result;
use serde::{Deserialize, Serialize};

concordance_gen::generate!({
    path: "../eventcatalog",
    role: "projector",
    entity: "balance"
});

mod store;

#[async_trait]
impl BalanceProjector for BalanceProjectorImpl {
    async fn handle_balance_created(&self, input: BalanceCreated) -> Result<()> {
        store::initialize_balance(input).await
    }

    async fn handle_balance_deposited(&self, input: BalanceDeposited) -> Result<()> {
        store::record_balance_deposited(input).await
    }

    async fn handle_balance_deducted(&self, input: BalanceDeducted) -> Result<()> {
        store::record_balance_deducted(input).await
    }
}

