use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::poh::ProofOfHistory;

pub type SlotNumber = u64;
pub type SlotHash = [u8; 32];
pub type BlockHash = SlotHash;

/// Minimal Transaction object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    // transaction origin
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub recent_block_hash: BlockHash,
}

/// Minimal Slot implementation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Slot {
    pub slot_number: SlotNumber,
    pub slot_hash: SlotHash,
    pub transactions: Vec<Transaction>,
    pub end_poh_count: u64,
    pub end_poh_hash: SlotHash,
}

/// Ideally this should be our blockchain.
pub struct Blockchain {
    slots: Vec<Slot>,
    current_transactions: Vec<Transaction>,
    pub poh: ProofOfHistory,
    current_slot_number: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut poh = ProofOfHistory::new();
        let (genesis_hash, _) = poh.tick();

        let genesis_slot = Slot {
            slot_number: 0,
            slot_hash: genesis_hash,
            transactions: Vec::new(),
            end_poh_hash: genesis_hash,
            end_poh_count: 0,
        };

        Blockchain {
            slots: vec![genesis_slot],
            current_transactions: Vec::new(),
            poh,
            current_slot_number: 1,
        }
    }

    pub fn add_transaction(&mut self, mut transaction: Transaction) {
        // use most recent slot hash for the transaction
        transaction.recent_block_hash = self.slots.last().unwrap().slot_hash;
        self.current_transactions.push(transaction);
    }

    pub fn create_slot(&mut self) -> Slot {
        // get the PoH hash at the start of the slot
        let (slot_hash, _) = self.poh.tick();
        let transactions = std::mem::take(&mut self.current_transactions);
        let serialized_transactions = bincode::serialize(&transactions).unwrap();
        let (end_poh_hash, end_poh_count) = self.poh.record_event(&serialized_transactions);

        let slot = Slot {
            slot_number: self.current_slot_number,
            slot_hash,
            transactions,
            end_poh_hash,
            end_poh_count,
        };

        self.slots.push(slot.clone());
        self.current_slot_number += 1;
        slot
    }
}
