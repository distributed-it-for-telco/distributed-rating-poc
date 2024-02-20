use crate::*;

impl From<BalanceCreated> for BalanceAggregateState {
    fn from(input: BalanceCreated) -> BalanceAggregateState {
        BalanceAggregateState {
            balance: input.balance.unwrap_or(0) as _,
            party_id: input.party_id,
        }
    }
}

pub(crate) fn apply_balance_created(input: BalanceCreated) -> Result<StateAck> {
    Ok(StateAck::ok(Some(BalanceAggregateState::from(input))))
}

pub(crate) fn apply_balance_deposited(
    input: BalanceDeposited,
    state: Option<BalanceAggregateState>,
) -> Result<StateAck> {
    let Some(state) = state else {
        error!(
            "Rejecting balance deposited event. party {} does not exist.",
            input.party_id
        );
        return Ok(StateAck::error(
            "Party does not exist",
            None::<BalanceAggregateState>,
        ));
    };

    let state = state.deposit(input.amount as u32);
  
    Ok(StateAck::ok(Some(state)))
}

pub(crate) fn apply_balance_deducted(
    input: BalanceDeducted,
    state: Option<BalanceAggregateState>,
) -> Result<StateAck> {
    let Some(state) = state else {
        error!(
            "Rejecting balance deducted event. Party {} does not exist.",
            input.party_id
        );
        return Ok(StateAck::error(
            "Party does not exist",
            None::<BalanceAggregateState>,
        ));
    };
    let state = state.withdraw(input.amount as u32);
    Ok(StateAck::ok(Some(state)))
}

