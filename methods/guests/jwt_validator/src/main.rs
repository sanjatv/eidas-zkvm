use risc0_zkvm::guest::env;
use jwt_compact::alg::Es256;
use jwt_compact::prelude::*;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    let jwk_json: &str =
        r#"
        {
            "kty": "EC",
            "crv": "P-256",
            "x": "e71HaZVzVFi1ypluxqzvdrgfWyZ0NSwFYE_0vppUt6I",
            "y": "YYvY62S0wOSxejfYeTLB6_3Y9J1Xr2PnZ94VUVJsrQc"
        }
    "#;

    let key = Es256.verifying_key_from_jwk(jwk_json)?;
    // TODO: do something with the input

    // write public output to the journal

    env::commit(&input);
}
