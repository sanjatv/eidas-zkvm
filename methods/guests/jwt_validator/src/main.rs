use risc0_zkvm::guest::env;
// use jwt_compact::alg::Es256;
// use jwt_compact::prelude::*;
use base64ct::{ Base64UrlUnpadded, Encoding };
use p256::ecdsa::VerifyingKey;
use p256::NistP256;
use p256::elliptic_curve::{ sec1::EncodedPoint, generic_array::{ GenericArray, typenum::U32 } };

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

    // Build SEC1 encoded point from x,y
    // This constructs an elliptic-curve point (the public key) from X and Y
    // false: do not compress the point
    // https://paritytech.github.io/try-runtime-cli/sec1/point/struct.EncodedPoint.html#tymethod.from_affine_coordinates
    let point = EncodedPoint::<NistP256>::from_affine_coordinates(&x_arr, &y_arr, false);

    // Create verifying key
    // https://docs.rs/p256/latest/p256/ecdsa/type.VerifyingKey.html#method.from_encoded_point
    // let verifying_key = VerifyingKey::from_encoded_point(&point).unwrap();
    // let alg = Es256;

    // Example JWT verification (you can pass it in later)
    // let jwt = env::read::<String>();
    // let claims = jwt_compact::Claims::decode_and_verify::<Es256>(&jwt, &verifying_key).unwrap();
    // env::commit(&claims);

    println!("POINT {}", point);
    // TODO: do something with the input

    // write public output to the journal

    env::commit(&input);
}
