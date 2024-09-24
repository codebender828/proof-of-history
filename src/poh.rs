use blake3::Hasher;

use crate::ledger::hash_to_hex;

pub struct ProofOfHistory {
    hasher: Hasher,
    count: u64,
}

impl ProofOfHistory {
    pub fn new() -> Self {
        ProofOfHistory {
            hasher: Hasher::new(),
            count: 0,
        }
    }

    pub fn tick(&mut self) -> ([u8; 32], u64) {
        self.hasher.update(&self.count.to_le_bytes());
        let mut hash = [0u8; 32];
        self.hasher.finalize_xof().fill(&mut hash);
        self.count += 1;
        (hash, self.count - 1)
    }

    pub fn record_event(&mut self, event: &[u8]) -> ([u8; 32], u64) {
        self.hasher.update(&self.count.to_le_bytes());
        self.hasher.update(event);
        let mut hash = [0u8; 32];
        self.hasher.finalize_xof().fill(&mut hash);
        self.count += 1;
        (hash, self.count - 1)
    }

    /// Verifies the proof of sequence of tick events
    pub fn verify_sequence(
        &self,
        start_hash: [u8; 32],
        start_count: u64,
        end_hash: [u8; 32],
        end_count: u64,
        events: &[&[u8]],
    ) -> bool {
        let mut hasher = Hasher::new();
        hasher.update(&start_hash);
        let mut current_count = start_count;

        for &event in events {
            hasher.update(&current_count.to_le_bytes());
            hasher.update(event);
            current_count += 1
        }

        while current_count < end_count {
            hasher.update(&current_count.to_le_bytes());
            current_count += 1;
        }

        let mut final_hash = [0u8; 32];
        hasher.finalize_xof().fill(&mut final_hash);

        println!("current_hash {:?}", hash_to_hex(&final_hash));
        println!("end_hash {:?}", hash_to_hex(&end_hash));

        final_hash == end_hash && current_count == end_count
    }
}
