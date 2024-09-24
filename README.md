# Light Proof Of History Implementation

Proof Of History at it's core is a verifiable and computable means, by which the order of the passage if time can be crytographically proven, independent of using System Time.

This repo is simple exercise that implements a very light blockchain that uses proof of history to verify the order of transactions in it's ledger.

This repo does not implement consensus, or transaction execution. It only implements a very light ledger with a verifiable occurance of events.

## Known bugs:

- Hashing result does not match at:
  ```rs
  let verification_result =
        ledger.verify_proof_of_history_between_slots(0, ledger.get_slots_height() - 1);
  ```
  This the result returned here should be `true` but it currently returns false.

# Install dependencies

```bash
cargo install
```

# Run

```bash
cargo run
```

`
