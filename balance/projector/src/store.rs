use std::collections::HashMap;

use crate::*;

use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::{GetResponse, KeyValue, KeyValueSender, SetRequest};
use wasmcloud_interface_logging::{debug, error};

// Note an invariant: the last() element in a ledger's effective_balance field is
// always the same as the balance stored in the balance.{account} key.

/// Creates a new BalanceLedger instance with an initial transaction as a deposit,
/// sets the current balance to the initial amount
pub async fn initialize_balance(event: BalanceCreated) -> Result<()> {
    debug!("Initializing balance for party {}", event.party_id);
    let kv = KeyValueSender::new();

    let party_id = event.party_id.to_string();
    let ctx = Context::default();

    let initial_balance = event.balance as u32;

    // Set up the initial ledger
    let ledger_key = format!("ledger.{party_id}");
    let ledger = BalanceLedger::new(event.party_id, initial_balance);
    let ledger_json = serde_json::to_string(&ledger).unwrap(); // we know this won't fail

    // set the current balance
    let balance_key = format!("balance.{party_id}");

    set(&ctx, &kv, ledger_key, ledger_json).await;
    set(&ctx, &kv, balance_key, initial_balance.to_string()).await;

    Ok(())
}

/// Records a deposit by adding a `LedgerLine` to the end of the previously stored
/// ledger and recording the new balance.
pub async fn record_balance_deposited(event: BalanceDeposited) -> Result<()> {
    debug!("Recording deposit in balance for party {}", event.party_id);
    let party_id = event.party_id.to_string();
    let ctx = Context::default();

    let kv = KeyValueSender::new();
    let ledger_key = format!("ledger.{party_id}");

    let new_ledger = get(&ctx, &kv, &ledger_key).await.map(|ledger_raw| {
        serde_json::from_str::<BalanceLedger>(&ledger_raw).map(|mut ledger| {
            let last_balance = ledger.ledger_lines.last().unwrap().effective_balance;
            ledger.ledger_lines.push(LedgerLine {
                amount: event.amount as u32,
                tx_type: TransactionType::Deposit,
                effective_balance: last_balance + event.amount as u32,
            });
            ledger
        })
    });
    if let Some(Ok(ledger)) = new_ledger {
        let new_balance = ledger
            .ledger_lines
            .last()
            .map(|l| l.effective_balance)
            .unwrap_or(0);
        set_ledger(&ctx, &kv, ledger_key, ledger).await;
        let balance_key = format!("balance.{party_id}");
        set(&ctx, &kv, balance_key, new_balance.to_string()).await;
    } else {
        error!("Unable to save projection for deposit balance for party {party_id}");
    }

    Ok(())
}

/// Records a deduction from a balance by adding a deposit ledger item to the
/// ledger and recording the new balance
pub async fn record_balance_deducted(event: BalanceDeducted) -> Result<()> {
    debug!("Recording deduction from balance for party {}", event.party_id);
    let party_id = event.party_id.to_string();

    let kv = KeyValueSender::new();
    let ledger_key = format!("ledger.{party_id}");

    let ctx = Context::default();

    // Note:the aggregate would prevent the creation of an event that would violate
    // business rules, so we can safely do the subtraction here without any guards

    let new_ledger = get(&ctx, &kv, &ledger_key).await.map(|ledger_raw| {
        serde_json::from_str::<BalanceLedger>(&ledger_raw).map(|mut ledger| {
            let last_balance = ledger.ledger_lines.last().unwrap().effective_balance;
            ledger.ledger_lines.push(LedgerLine {
                amount: event.amount as u32,
                tx_type: TransactionType::Deduction,
                effective_balance: last_balance - event.amount as u32,
            });
            ledger
        })
    });
    if let Some(Ok(ledger)) = new_ledger {
        let new_balance = ledger
            .ledger_lines
            .last()
            .map(|l| l.effective_balance)
            .unwrap_or(0);
        set_ledger(&ctx, &kv, ledger_key, ledger).await;
        let balance_key = format!("balance.{party_id}");
        set(&ctx, &kv, balance_key, new_balance.to_string()).await;
    } else {
        error!("Unable to save projection for balance deduction for party {party_id}");
    }

    Ok(())
}

async fn set(ctx: &Context, kv: &KeyValueSender<WasmHost>, key: String, value: String) {
    if let Err(e) = kv
        .set(
            ctx,
            &SetRequest {
                key: key.clone(),
                value,
                expires: 0,
            },
        )
        .await
    {
        error!("Failed to set {key} in store: {e}");
    }
}

async fn set_ledger(
    ctx: &Context,
    kv: &KeyValueSender<WasmHost>,
    key: String,
    ledger: BalanceLedger,
) {
    set(ctx, kv, key, serde_json::to_string(&ledger).unwrap()).await
}

async fn get(ctx: &Context, kv: &KeyValueSender<WasmHost>, key: &str) -> Option<String> {
    match kv.get(ctx, key).await {
        Ok(GetResponse {
            value: v,
            exists: true,
        }) => Some(v),
        Ok(GetResponse { exists: false, .. }) => None,
        Err(e) => {
            error!("Failed to get {key} from store: {e}");
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BalanceLedger {
    pub party_id: String,
    pub ledger_lines: Vec<LedgerLine>,
    pub holds: HashMap<String, u32>,
}

impl BalanceLedger {
    fn new(party_id: String, initial_balance: u32) -> BalanceLedger {
        BalanceLedger {
            party_id,
            holds: HashMap::new(),
            ledger_lines: vec![LedgerLine {
                amount: initial_balance,
                tx_type: TransactionType::Deposit,
                effective_balance: initial_balance,
            }],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LedgerLine {
    pub amount: u32,
    pub tx_type: TransactionType,
    pub effective_balance: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
enum TransactionType {
    Deduction,
    Deposit,
    Unknown,
}
