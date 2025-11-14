# eIDAS zkVM

A RISC Zero zkVM application for generating zero-knowledge proofs (ZKPs) from eIDAS 2.0-compliant verifiable credentials.

## Guests

This application contains multiple guest programs that the zkVM can run and generate ZKPs for.

#### JWT Validator

The JWT validator guest is a guest program that validates the signature on the signed JSON web token (JWT) to ensure its integrity.

# Run
```cargo run --release```
or
```RISC0_DEV_MODE=0 cargo run --release```

## Dev mode
```RISC0_DEV_MODE=1 cargo run --release```