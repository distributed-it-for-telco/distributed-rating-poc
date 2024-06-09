use anyhow::Result;

use serde::{Deserialize, Serialize};
use wasmcloud_interface_logging::error;

mod commands;
mod events;
mod state;

use state::BalanceAggregateState;

concordance_gen::generate!({
    path: "../eventcatalog",
    role: "aggregate",
    entity: "balance"
});

impl BalanceAggregate for BalanceAggregateImpl {
    // -- Commands --
    fn handle_create_balance(
        &self,
        input: CreateBalance,
        _state: Option<BalanceAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_create_balance(input)
    }

    fn handle_deduct_balance(
        &self,
        input: DeductBalance,
        state: Option<BalanceAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_deduct_balance(input, state)
    }

    fn handle_deposit_balance(
        &self,
        input: DepositBalance,
        state: Option<BalanceAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_deposit_balance(input, state)
    }

    // -- Events --
    fn apply_balance_created(
        &self,
        input: BalanceCreated,
        _state: Option<BalanceAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_balance_created(input)
    }

    fn apply_balance_deposited(
        &self,
        input: BalanceDeposited,
        state: Option<BalanceAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_balance_deposited(input, state)
    }

    fn apply_balance_deducted(
        &self,
        input: BalanceDeducted,
        state: Option<BalanceAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_balance_deducted(input, state)
    }
}

const STREAM: &str = "balance";
