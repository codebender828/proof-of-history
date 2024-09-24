use std::{
    io::Read,
    str::FromStr,
    time::{Duration, Instant},
};

use blake3::Hash;
use runtime::{Blockchain, Transaction};

mod account;
mod poh;
mod runtime;
mod state;
mod validator;

fn main() {
    let mut blockchain = Blockchain::new();

    // simulate some transactions
    blockchain.add_transaction(Transaction {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount: 50,                 // 50 SOL
        recent_block_hash: [0; 32], //  this will be overwritten in `add_tranaction`
    });

    let slot = blockchain.create_slot();
    println!(
        "New slot created: count: {:?}, hash: {:?}",
        slot.slot_number,
        Hash::try_from(slot.slot_hash).unwrap().to_hex()
    );

    // Simulate passage of time and more transactions
    for _ in 0..1000 {
        blockchain.poh.tick();
    }

    blockchain.add_transaction(Transaction {
        from: "Bob".to_string(),
        to: "Charlie".to_string(),
        amount: 10,                 // 10 SOL
        recent_block_hash: [0; 32], // this will be overwritten in `add_transaction`
    });

    let slot = blockchain.create_slot();
    println!(
        "Another slot created: count: {:?}, hash: {:?}",
        slot.slot_number,
        Hash::try_from(slot.slot_hash).unwrap().to_hex()
    );
}
