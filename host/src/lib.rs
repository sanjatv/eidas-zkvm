use std::fs;

use chrono::{ NaiveDate, Utc, Datelike };
use methods::{ JWT_VALIDATOR_ELF, JWT_VALIDATOR_ID, AGE_OVER_18_ELF };
use risc0_zkvm::{ ExecutorEnv, Receipt, default_prover };
use serde_json::Value;
use anyhow::{ Context, Result };

pub fn from_file_to_json(filepath: &str) -> Value {
    println!("In file {}!", filepath);

    let contents = fs::read_to_string(filepath).expect("Not able to read file.");
    let json: Value = serde_json::from_str(&contents).expect("Invalid JSON structure.");
    json
}

fn extract_birthdate_from_jwt(json: &Value) -> NaiveDate {
    // Navigate to birth_date field
    let birthdate_str = json["vc"]["credentialSubject"]["birth_date"]
        .as_str()
        .ok_or("Missing birthdate");

    NaiveDate::parse_from_str(birthdate_str.unwrap_or("unknown"), "%Y-%m-%d").expect(
        "Invalid date string."
    )
}

pub fn get_age_from_jwt(filepath: &str) -> i32 {
    let json: Value = from_file_to_json(filepath);
    let birthdate = extract_birthdate_from_jwt(&json);
    let today = Utc::now().date_naive();
    let age =
        today.year() -
        birthdate.year() -
        (if (today.month(), today.day()) < (birthdate.month(), birthdate.day()) { 1 } else { 0 });
    age
}

pub fn create_zkp_age_over_18(age: &i32) -> Result<Receipt> {
    let env = ExecutorEnv::builder().write(&age).unwrap().build().unwrap();
    // Obtain the default prover.
    let prover = default_prover();

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    // let prove_info = prover.prove(env, AGE_OVER_18_ELF).unwrap();

    let receipt: Receipt = prover
        .prove(env, AGE_OVER_18_ELF)
        .with_context(||
            format!("Guest program failed. There is no valid proof of being over 18.")
        )?.receipt;

    Ok(receipt)
}

pub fn verify_jwt_signature(jwt: &str) -> Result<(bool, Receipt)> {
    let receipt: Receipt = create_zkp_jwt_valid(jwt)?;

    let is_valid: bool = receipt.journal
        .decode()
        .expect("Failed to deserialixe journal input as bool");

    receipt.verify(JWT_VALIDATOR_ID).expect("Proof verification failed");

    Ok((is_valid, receipt))
}

pub fn create_zkp_jwt_valid(jwt: &str) -> Result<Receipt> {
    // An executor environment describes the configurations for the zkVM
    // including program inputs.
    // A default ExecutorEnv can be created like so:
    // `let env = ExecutorEnv::builder().build().unwrap();`
    // However, this `env` does not have any inputs.
    //
    // To add guest input to the executor environment, use
    // ExecutorEnvBuilder::write().
    // To access this method, you'll need to use ExecutorEnv::builder(), which
    // creates an ExecutorEnvBuilder. When you're done adding input, call
    // ExecutorEnvBuilder::build().
    let env = ExecutorEnv::builder().write(&jwt).unwrap().build().unwrap();
    let prover = default_prover();

    let receipt: Receipt = prover
        .prove(env, JWT_VALIDATOR_ELF)
        .with_context(||
            format!("Guest program failed. There is no valid proof of the signature being valid.")
        )?.receipt;

    Ok(receipt)
}
