# eIDAS zkVM

A RISC Zero zkVM application for generating zero-knowledge proofs (ZKPs) from eIDAS 2.0-compliant verifiable credentials.

## Guests

This application contains multiple guest programs that the zkVM can run and generate ZKPs for.

#### JWT Validator

The JWT validator guest is a guest program that validates the signature on the signed JSON web token (JWT) to ensure its integrity.

## Running

There are two independent options when running the host:

**`RISC0_DEV_MODE`** controls how the zkVM generates proofs. In normal mode (`RISC0_DEV_MODE=0`, the default), the zkVM produces real cryptographic proofs. This is slow but generates valid receipts suitable for production. In dev mode (`RISC0_DEV_MODE=1`), proving is skipped and replaced with a fast mock, which is useful during development but produces invalid receipts that must never be used in production.

**`serve`** controls whether the host starts an HTTP API server. Without it, the host runs the proof pipeline directly and exits. With `-- serve`, it starts a web server on `localhost:3030` so external clients can submit verification requests.

### Options

Run with real proofs, no server:
```
cargo run --release
```

Run with real proofs, with API server:
```
cargo run --release -- serve
```

Run with dev mode (fast, no valid proofs), no server:
```
RISC0_DEV_MODE=1 cargo run --release
```

Run with dev mode, with API server:
```
RISC0_DEV_MODE=1 cargo run --release -- serve
```

### Testing the API

To test the API endpoint with curl you can run this in your terminal:

```
curl -X POST http://localhost:3030/verify \
-H "Content-Type: application/json" \
-d "{\"jwt\":\"$(cat PIDVCencoded)\"}"
```

This command sends the encoded JWT from the `PIDVCencoded` file to the `/verify` endpoint, where the server validates the JWT’s signature inside the zkVM.
