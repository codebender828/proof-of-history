use blake3::Hasher;

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
        self.hasher.update(&self.count.to_be_bytes());
        let hash = self.hasher.finalize();
        self.count += 1;
        (*hash.as_bytes(), self.count - 1)
    }

    pub fn record_event(&mut self, event: &[u8]) -> ([u8; 32], u64) {
        self.hasher.update(&self.count.to_be_bytes());
        self.hasher.update(event);
        let hash = self.hasher.finalize();
        self.count += 1;
        (*hash.as_bytes(), self.count - 1)
    }

    pub fn get_hash(&self) -> [u8; 32] {
        *self.hasher.finalize().as_bytes()
    }
}
