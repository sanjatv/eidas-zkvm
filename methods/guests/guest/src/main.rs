// The guest code is never launched as a standalone Rust executable, so we specify #![no_main]
#![no_main]

// The guest code should be as lightweight as possible for performance reasons. So since we’re not using std, we exclude it.
#![no_std]

use risc0_zkvm::guest::env;

// We must make the guest code available for the host to launch, and to do that we must specify which function to call when the host
// starts executing this guest code. This is a macro to indicate the initial guest function to call, which in this case is ‘main’.
risc0_zkvm::guest::entry!(main);

fn main() {
    // Load the first number from the host
    let age: i32 = env::read();

    // Logic circuit
    let is_valid = &(age >= 18);
    // ! after a function name means it is a macro, not a regular function (sikkert fordi Rust er funksjonelt språk)
    assert!(is_valid, "User is underage");

    // write public output to the journal
    // & is reference operator. &age creates a reference (pointer) to the variable age, rather than the value itself
    env::commit(&is_valid);
}
