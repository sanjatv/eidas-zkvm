use std::fs;

use chrono::{ NaiveDate, Utc, Datelike };
use methods::JWT_VALIDATOR_ELF;
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
        .prove(env, JWT_VALIDATOR_ELF)
        .with_context(||
            format!("Guest program failed. There is no valid proof of being over 18.")
        )?.receipt;

    Ok(receipt)
}
