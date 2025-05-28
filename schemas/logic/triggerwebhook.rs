
use std::collections::HashMap;
use poh_stack::component_01_validator_core::schema_executor::schema_executor::Action;

pub fn execute(action: &Action, ledger: &mut HashMap<String, (u64, u64)>) {
    // Simulated logic for triggerwebhook
    let entry = ledger.entry(action.sender.clone()).or_insert((0, 0));
    entry.0 += action.amount;
    entry.1 += 1;

    if let Some(receiver) = &action.receiver {
        let receiver_entry = ledger.entry(receiver.clone()).or_insert((0, 0));
        receiver_entry.0 += action.amount / 2;  // Simulated distribution
    }

    println!("Executed triggerwebhook for sender: {}", action.sender);
}
