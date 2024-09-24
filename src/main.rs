use blake3::Hash;
use ledger::{Ledger, Transaction};

mod account;
mod ledger;
mod poh;
mod state;
mod validator;

fn main() {
    let mut ledger = Ledger::new();

    // simulate some transactions
    ledger.add_transaction(Transaction {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount: 50,                 // 50 SOL
        recent_block_hash: [0; 32], //  this will be overwritten in `add_tranaction`
    });

    let slot = ledger.create_slot();
    println!(
        "New slot created: count: {:?}, hash: {:?}",
        slot.slot_number,
        Hash::try_from(slot.slot_hash).unwrap().to_hex()
    );

    // Simulate passage of time and more transactions
    for _ in 0..1000 {
        ledger.poh.tick();
    }

    ledger.add_transaction(Transaction {
        from: "Bob".to_string(),
        to: "Charlie".to_string(),
        amount: 10,                 // 10 SOL
        recent_block_hash: [0; 32], // this will be overwritten in `add_transaction`
    });

    ledger.add_transaction(Transaction {
        from: "Charlie".to_string(),
        to: "Alice".to_string(),
        amount: 10,
        recent_block_hash: [0; 32],
    });

    let slot = ledger.create_slot();
    println!(
        "Another slot created: count: {:?}, hash: {:?}",
        slot.slot_number,
        Hash::try_from(slot.slot_hash).unwrap().to_hex()
    );

    ledger.log_ledger();

    let verification_result =
        ledger.verify_proof_of_history_between_slots(0, ledger.get_slots_height() - 1);

    println!(
        "PoH verification between slots 0 and 1: {}",
        verification_result
    );
}
