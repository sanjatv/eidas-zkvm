# eIDAS zkVM

A RISC Zero zkVM application for generating zero-knowledge proofs (ZKPs) from eIDAS 2.0-compliant verifiable credentials.

## Guests

This application contains multiple guest programs that the zkVM can run and generate ZKPs for.

#### JWT Validator

The JWT validator guest is a guest program that validates the signature on the signed JSON web token (JWT) to ensure its integrity.

# Run

`cargo run --release`
or with dev mode explicitly disabled:
`RISC0_DEV_MODE=0 cargo run --release`

## Dev mode

Run with dev mode enabled (faster proving/debugging). WARNING: Proving in dev mode does not generate a valid receipt. Receipts generated from this process are invalid and should never be used in production.
`RISC0_DEV_MODE=1 cargo run --release`

If you want to run dev mode without api run
`RISC0_DEV_MODE=1 cargo run --release -- dev`

## HTTP API mode

In HTTP mode, the host runs as a web server so external clients can call it. To run the API you have to run
`RISC0_DEV_MODE=1 cargo run --release`

To test the API endpoint with curl you can run this in your terminal:

```
curl -X POST http://localhost:3030/verify \
-H "Content-Type: application/json" \
-d "{\"jwt\":\"$(cat PIDVCencoded)\"}"
```

This command sends the encoded JWT from the `PIDVCencoded` file to the `/verify` endpoint, where the server validates the JWT’s signature inside the zkVM.
