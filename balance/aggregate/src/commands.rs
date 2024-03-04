use crate::*;

pub(crate) fn handle_create_balance(input: CreateBalance) -> Result<EventList> {
    Ok(vec![Event::new(
        BalanceCreated::TYPE,
        STREAM,
        &BalanceCreated {
            balance: input.balance,
            party_id: input.party_id,
        },
    )])
}

pub(crate) fn handle_deduct_balance(
    input: DeductBalance,
    state: Option<BalanceAggregateState>,
) -> Result<EventList> {
    let Some(state) = state else {
        return Err(anyhow::anyhow!(
            "Rejected command to deduct balance for party {}.",
            input.party_id
        ));
    };

    if state.available_balance() < input.amount as u32 {
        error!(
                "Rejecting command to deduct balance, party {} balance does not have sufficient amount. Available {}",
                &input.party_id, state.available_balance()
            );
        Ok(vec![])
    } else {
        Ok(vec![Event::new(
            BalanceDeducted::TYPE,
            STREAM,
            &BalanceDeducted {
                amount: input.amount,
                party_id: input.party_id,
            },
        )])
    }
}

pub(crate) fn handle_deposit_balance(
    input: DepositBalance,
    state: Option<BalanceAggregateState>,
) -> Result<EventList> {
    if state.is_none() {
        return Err(anyhow::anyhow!(
            "Rejected command to deposit balance. party {} does not exist.",
            input.party_id
        ));
    };

    if input.amount < 0 {
        error!("Rejecting command to deposit balance with negative amount");
        Ok(vec![])
    } else {
        Ok(vec![Event::new(
            BalanceDeposited::TYPE,
            STREAM,
            &BalanceDeposited {
                amount: input.amount,
                party_id: input.party_id,
            },
        )])
    }
}
