use risc0_zkvm::guest::env;
// use jwt_compact::alg::Es256;
// use jwt_compact::prelude::*;
use base64ct::{ Base64UrlUnpadded, Encoding };
use p256::ecdsa::VerifyingKey;
use p256::ecdsa::Signature;
use p256::ecdsa::signature::Verifier;
use p256::NistP256;
use p256::elliptic_curve::{
    sec1::EncodedPoint,
    generic_array::{ GenericArray, typenum::U32, typenum::U64 },
};

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

    let jwt: &str =
        "eyJraWQiOiJReHBOaDQzUzdyZnFtSlZoRG03UzJZT3l2SDBiYjRKdE41UVJ5SkhQcThnIiwidHlwIjoiSldUIiwiYWxnIjoiRVMyNTYiLCJqd2siOnsia3R5IjoiRUMiLCJjcnYiOiJQLTI1NiIsIngiOiJlNzFIYVpWelZGaTF5cGx1eHF6dmRyZ2ZXeVowTlN3RllFXzB2cHBVdDZJIiwieSI6IllZdlk2MlMwd09TeGVqZlllVExCNl8zWTlKMVhyMlBuWjk0VlVWSnNyUWMifX0.eyJpc3MiOiJodHRwczovL3Byb3RvdHlwZS1sb21pbm8taXNzdWVyLmF6dXJld2Vic2l0ZXMubmV0L29wZW5pZC9kcmFmdF8xNCIsInN1YiI6ImRpZDprZXk6ejJkbXpEODFjZ1B4OFZraTdKYnV1TW1GWXJXUGdZb3l0eWtVWjNleXFodDFqOUtibjlTSGhTVWNiZjM4dVVweXAzQmltejlqTDZOUU5EOFZCTGg0a0NaaHRrbnVLUzRiQU1EU1duUFFHZ1lKS25GamN5QVJSTG45cDU3SzNZdjVRVnBzaVVtemV1aE5MRkJRTjNWWXhKSFAyQjJRdlE5UnVVRDJHRVZIQWZxcWRSSG5mRSIsInZjIjp7InR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJwaWQiXSwiY3JlZGVudGlhbFN1YmplY3QiOnsiZ2l2ZW5fbmFtZSI6IkFsbHNsYWdzIiwiZmFtaWx5X25hbWUiOiJMZWtlcGxhc3MiLCJiaXJ0aF9kYXRlIjoiMTk5Mi0xMC0xOSIsImJpcnRoX3BsYWNlIjoiU3ZlcmlnZSIsIm5hdGlvbmFsaXR5IjoiU0UiLCJwZXJzb25hbF9hZG1pbmlzdHJhdGl2ZV9udW1iZXIiOiI5MjEwMTktOTMyMCIsImlkIjoiZGlkOmtleTp6MmRtekQ4MWNnUHg4VmtpN0pidXVNbUZZcldQZ1lveXR5a1VaM2V5cWh0MWo5S2JuOVNIaFNVY2JmMzh1VXB5cDNCaW16OWpMNk5RTkQ4VkJMaDRrQ1podGtudUtTNGJBTURTV25QUUdnWUpLbkZqY3lBUlJMbjlwNTdLM1l2NVFWcHNpVW16ZXVoTkxGQlFOM1ZZeEpIUDJCMlF2UTlSdVVEMkdFVkhBZnFxZFJIbmZFIn0sImlkIjoidXJuOnV1aWQ6YzU0Mzk2M2MtZjVhZC00ZDlkLWEzZmMtNGEzYzc3YTllNWY0IiwiaXNzdWVyIjp7ImlkIjoiZGlkOndlYjpwcm90b3R5cGUtbG9taW5vLWlzc3Vlci5henVyZXdlYnNpdGVzLm5ldDpmNzAwZTllMCIsIm5hbWUiOiJTa2F0dGV2ZXJrZXQifSwidmFsaWRGcm9tIjoiMjAyNS0xMS0wN1QxNDozNToyOS42NTgxNjk1NDhaIiwidmFsaWRVbnRpbCI6IjIwMjYtMDItMDdUMTQ6MzU6MjkuNjU4MjMyNjQ4WiIsImNyZWRlbnRpYWxTdGF0dXMiOnsiaWQiOiJodHRwczovL3Byb3RvdHlwZS1sb21pbm8taXNzdWVyLmF6dXJld2Vic2l0ZXMubmV0L2NyZWRlbnRpYWxzL3N0YXR1cy9jMzMzNzU3NC1iYzViLTQwYTYtOThmMi1mYTYyMzczZDBjNjMiLCJ0eXBlIjoiS2FudGVnYVN0YXR1c0xpc3RFbnRyeSIsInN0YXR1c1B1cnBvc2UiOiJyZXZvY2F0aW9uIiwic3RhdHVzTGlzdEluZGV4IjoiYzMzMzc1NzQtYmM1Yi00MGE2LTk4ZjItZmE2MjM3M2QwYzYzIiwic3RhdHVzTGlzdENyZWRlbnRpYWwiOiJodHRwczovL3Byb3RvdHlwZS1sb21pbm8taXNzdWVyLmF6dXJld2Vic2l0ZXMubmV0L2NyZWRlbnRpYWxzL3N0YXR1cy9jMzMzNzU3NC1iYzViLTQwYTYtOThmMi1mYTYyMzczZDBjNjMifSwiY3JlZGVudGlhbEJyYW5kaW5nIjp7ImJhY2tncm91bmRDb2xvciI6IiNGRUNDMDIiLCJsb2dvVXJsIjoiaHR0cHM6Ly9kZXNpZ24uYmV2aXNzdHVkaW8ubm8vYXBpL3B1YmxpYy9sb2dvL2J5SWQvMzcwZThmNTUtOTM0Yy00NzA4LTg1MGEtOWViYzBiZjk3ZTg1LmpwZyJ9LCJuYW1lIjoiRGlnaXRhbHQgSWRlbnRpdGV0c2tvcnQgKHNlKSIsImRlc2NyaXB0aW9uIjoiU3ZlbnNrIG1lZGJvcmdhcmUifX0.3lX3pXwYgQdh9rlXomiWXB4YiDAolrKrj9MN6QY_4pIkA2Jl4mR9CRqnDhRZvw6cSW15LLfn-6ARDe5xXRJArw";

    // --- Parse the JWK manually ---
    let jwk: serde_json::Value = serde_json::from_str(jwk_json).unwrap();

    let x_64 = jwk["x"].as_str().unwrap();
    let y_64 = jwk["y"].as_str().unwrap();

    let x_bytes = Base64UrlUnpadded::decode_vec(x_64).unwrap();
    let y_bytes = Base64UrlUnpadded::decode_vec(y_64).unwrap();

    // Ensure exactly 32 bytes (P-256 coordinate length)
    assert_eq!(x_bytes.len(), 32);
    assert_eq!(y_bytes.len(), 32);

    // Convert Vec<u8> to GenericArray<u8, U32>
    let x_arr: &GenericArray<u8, U32> = GenericArray::from_slice(&x_bytes);
    let y_arr: &GenericArray<u8, U32> = GenericArray::from_slice(&y_bytes);

    // Build SEC1 encoded point from x,y (les mer om SEC1 i masternotat Elliptic curve cryptocraphy)
    // This constructs an elliptic-curve point (the public key) from X and Y
    // false: do not compress the point
    // https://github.com/RustCrypto/formats/blob/cd86385e384b0d637c2e19fed0c812e5c0e10ee1/sec1/src/point.rs
    let point = EncodedPoint::<NistP256>::from_affine_coordinates(&x_arr, &y_arr, false);

    // Create verifying key
    // https://github.com/RustCrypto/signatures/blob/master/ecdsa/src/verifying.rs
    let verifying_key = VerifyingKey::from_encoded_point(&point).unwrap();

    // ---------------------------------------------
    // Dropper jwt_compact
    // Example JWT verification (you can pass it in later)
    // let jwt = env::read::<String>();
    // let claims = jwt_compact::Claims::decode_and_verify::<Es256>(&jwt, &verifying_key).unwrap();
    // env::commit(&claims);
    // ---------------------------------------------

    // jwt.split() returns an 'iterator' over the string slices between the dots
    // .collect() takes all those slices and builds a Vec<&str>
    // iterator is not a list. It's a producer of values. Kalle next() for å generere neste element.
    let parts: Vec<&str> = jwt.split('.').collect();

    // konverter tilbake til string
    let signing_input = format!("{}.{}", parts[0], parts[1]);
    // hent ut signaturen
    let signature_bytes = Base64UrlUnpadded::decode_vec(parts[2]).unwrap();

    // Signature::from_bytes expects the curve's signature length, e.g. 64 bytes for P-256
    let signature_arr: &GenericArray<u8, U64> = GenericArray::from_slice(&signature_bytes);
    // opprett Signature objekt fra bytes
    // https://github.com/RustCrypto/signatures/blob/master/ecdsa/src/lib.rs#L229
    let signature = Signature::from_bytes(signature_arr).unwrap();

    verifying_key
        .verify(signing_input.as_bytes(), &signature)
        .expect("ECDSA signature verification failed");

    // fn verify(&self, msg: &[u8], signature: &Signature<C>) -> Result<()> {
    //     self.multipart_verify(&[msg], signature)
    // }

    // signing_input er header og payload, altså msg: &[u8] som betyr at vi må konvertere header og payload
    // til bytes array
    // public_key.verify(signing_input.as_bytes(), &signature).is_ok()

    println!("POINT {}", point);
    println!("VERIFYING KEY {:?}", verifying_key);
    println!("SIGNATURE {:?}", signature);
    // TODO: do something with the input

    // write public output to the journal

    env::commit(&input);
}
