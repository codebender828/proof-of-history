use serde::{Deserialize, Serialize};

use crate::poh::ProofOfHistory;

pub type SlotNumber = u64;
pub type SlotHash = [u8; 32];
pub type BlockHash = SlotHash;

// Helper function to convert [u8; 32] to a hex string
pub fn hash_to_hex(hash: &[u8; 32]) -> String {
    hash.iter().map(|b| format!("{:02x}", b)).collect()
}

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
pub struct Ledger {
    slots: Vec<Slot>,
    current_transactions: Vec<Transaction>,
    pub poh: ProofOfHistory,
    current_slot_number: u64,
}

impl Ledger {
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

        Ledger {
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

    /// Gets the ledger's block height
    pub fn get_slots_height(&self) -> usize {
        self.slots.len()
    }

    pub fn verify_proof_of_history_between_slots(
        &self,
        start_slot_index: usize,
        end_slot_index: usize,
    ) -> bool {
        if start_slot_index >= end_slot_index || end_slot_index >= self.get_slots_height() {
            return false;
        }

        let start_slot = &self.slots[start_slot_index];
        let end_slot = &self.slots[end_slot_index];

        let mut events = Vec::with_capacity(self.get_slots_height() - 1);
        for slot in &self.slots[start_slot_index + 1..end_slot_index] {
            let serialized_transactions = bincode::serialize(&slot.transactions).unwrap();
            events.push(serialized_transactions);
        }

        let event_sequence = &events.iter().map(|e| e.as_slice()).collect::<Vec<&[u8]>>();

        self.poh.verify_sequence(
            start_slot.end_poh_hash,
            start_slot.end_poh_count,
            end_slot.slot_hash,
            end_slot.end_poh_count,
            event_sequence,
        )
    }

    // Logger utility to print out the entire ledger.
    pub fn log_ledger(&self) {
        println!("==================== BLOCKCHAIN LEDGER ====================");
        for (index, slot) in self.slots.iter().enumerate() {
            println!("Slot #{} (Slot Number: {})", index, slot.slot_number);
            println!("  Slot Hash: {:?}", hash_to_hex(&slot.slot_hash));
            println!("  End PoH Hash: {:?}", hash_to_hex(&slot.end_poh_hash));
            println!("  End PoH Count: {}", slot.end_poh_count);
            println!("  Transactions:");
            if slot.transactions.is_empty() {
                println!("    No transactions in this slot");
            } else {
                for (tx_index, tx) in slot.transactions.iter().enumerate() {
                    println!("    Transaction #{}:", tx_index);
                    println!("      From: {}", tx.from);
                    println!("      To: {}", tx.to);
                    println!("      Amount: {}", tx.amount);
                    println!(
                        "      Recent Slot Hash: {:?}",
                        hash_to_hex(&tx.recent_block_hash)
                    );
                }
            }
            println!("----------------------------------------------------------");
        }
        println!("==================== END OF LEDGER ==================== \n\n");
    }
}
