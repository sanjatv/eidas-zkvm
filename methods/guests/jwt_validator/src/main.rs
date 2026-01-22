#![no_main]
#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

use risc0_zkvm::guest::env;
use base64ct::{ Base64UrlUnpadded, Encoding };
use p256::ecdsa::VerifyingKey;
use p256::ecdsa::Signature;
use p256::ecdsa::signature::Verifier;
use p256::NistP256;
use p256::elliptic_curve::{
    sec1::EncodedPoint,
    generic_array::{ GenericArray, typenum::U32, typenum::U64 },
};
use serde_json::Value;
use serde::{ Deserialize, Serialize };

// tells rust to generate code that can serialize this struct into bytes
#[derive(Deserialize, Serialize)]
pub struct GuestInput {
    pub jwt: String,
}

pub fn main() {
    // read the input
    let input: GuestInput = env::read();

    let jwt = input.jwt.as_str();

    // jwt.split() returns an 'iterator' over the string slices between the dots
    // .collect() takes all those slices and builds a Vec<&str>
    // iterator is not a list. It's a producer of values. Kalle next() for å generere neste element.
    let parts: Vec<&str> = jwt.split('.').collect();

    let header_b64 = parts[0];
    // base64-decode header
    let header_json_bytes = Base64UrlUnpadded::decode_vec(header_b64).expect("invalid b64 header");

    // convert bytes -> str
    let header_json_str = core::str::from_utf8(&header_json_bytes).expect("header htf8 error");

    // parse header JSON
    let header: Value = serde_json::from_str(header_json_str).expect("invalid json");
    let jwk = header["jwk"].clone();

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

    // konverter tilbake til string
    let signing_input = format!("{}.{}", parts[0], parts[1]);
    // hent ut signaturen
    let signature_bytes = Base64UrlUnpadded::decode_vec(parts[2]).unwrap();

    // Signature::from_bytes expects the curve's signature length, e.g. 64 bytes for P-256
    let signature_arr: &GenericArray<u8, U64> = GenericArray::from_slice(&signature_bytes);
    // opprett Signature objekt fra bytes
    // https://github.com/RustCrypto/signatures/blob/master/ecdsa/src/lib.rs#L229
    let signature = Signature::from_bytes(signature_arr).unwrap();

    // burde ikke bruke expect() i guests, bare host
    let is_valid = verifying_key.verify(signing_input.as_bytes(), &signature).is_ok();

    // fn verify(&self, msg: &[u8], signature: &Signature<C>) -> Result<()> {
    //     self.multipart_verify(&[msg], signature)
    // }

    // signing_input er header og payload, altså msg: &[u8] som betyr at vi må konvertere header og payload
    // til bytes array
    // public_key.verify(signing_input.as_bytes(), &signature).is_ok()

    env::commit(&is_valid);
}
