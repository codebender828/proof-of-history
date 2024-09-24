use blake3::Hasher;
use std::time::{Duration, Instant};

struct ProofOfHistory {
    hasher: Hasher,
    count: u64,
}

impl ProofOfHistory {
    fn new() -> Self {
        ProofOfHistory {
            hasher: Hasher::new(),
            count: 0,
        }
    }

    fn tick(&mut self) {
        self.hasher.update(&self.count.to_be_bytes());
        self.count += 1
    }

    fn record_event(&mut self, event: &str) -> ([u8; 32], u64) {
        self.hasher.update(&self.count.to_be_bytes());
        self.hasher.update(event.as_bytes());
        let hash = self.hasher.finalize();
        self.count += 1;
        (*hash.as_bytes(), self.count - 1)
    }

    fn get_hash(&self) -> [u8; 32] {
        *self.hasher.finalize().as_bytes()
    }
}

fn main() {
    let mut poh = ProofOfHistory::new();
    let start = Instant::now();
    let duration = Duration::from_secs(5);

    while start.elapsed() < duration {
        poh.tick();
    }

    println!("Generated {} ticks in 5 seconds", poh.count);

    let (hash, count) = poh.record_event("Transaction X");
    println!("Recorded event at tick {}", count);
    println!("Event hash: {:?}", hash);
    println!("Current PoH hash: {:?}", poh.get_hash());
}
