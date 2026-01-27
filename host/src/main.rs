use std::fs;
use anyhow::Result;
use host::{ b64_encode_receipt, create_zkp_age_over_18, verify_age_over_18 };
use risc0_zkvm::Receipt;

// Double colons (::) are called path separators. Used to access items (functions, types, constants etc.)
// inside modules enums or crates.
mod api; // This makes host/src/api.rs available

// fn main() -> Result<()> {
//     // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
//     tracing_subscriber
//         ::fmt()
//         .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
//         .init();

//     let filepath_jwt_encoded: &str = "PIDVCencoded";
//     let jwt = fs::read_to_string(filepath_jwt_encoded).expect("Not able to read file");

//     let receipt: Receipt = create_zkp_age_over_18(jwt.as_str())?;
//     println!("receipt: {}", b64_encode_receipt(receipt));

//     // use '?' to forward errors to the caller instead of handling errors here. Vet ikke om dette er bedre enn å ha 'match'
//     // let (is_valid, _) = verify_age_over_18(receipt)?;

//     // println!("journal: {is_valid}");

//     // The receipt was verified at the end of proving, but the below code is an
//     // example of how someone else could verify this receipt.
//     //receipt.verify(AGE_OVER_18_ID).unwrap();

//     Ok(())
// }

// ------- HTTP entry point -------

// Må ha med #[tokio::main] fordi fordi start_server() er async
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Start the HTTP server (this never returns until you stop it)
    api::start_server().await;
    Ok(())
}
