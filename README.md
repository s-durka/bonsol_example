## Bash

We need solana test validator running with Bonsol deployed.

(in Bonsol, `bin/validator.sh`)
Deploy the Solana program needs to be deployed

in `program/`
```bash
cargo build-sbf
solana address -k ./target/deploy/
# get the Pubkey with:
solana address -k ./target/deploy/bonsol_example-keypair.json
# copy and paste the key to client/main.rs first line, then deploy:
solana program deploy ./target/deploy/bonsol_example.so
```

in `client`:
```bash
cargo run
```