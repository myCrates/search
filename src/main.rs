#![crate_name = "search"]
#![crate_type = "bin"]

//! This is a binary to can make fast search on internet from the terminal.
//! Try to use it and let me know how do you think about it !!!

/// This function alway return 42
fn number() -> u8 {
    42
}

/// This functions tests number
#[test]
fn test_number() {
    assert_eq!(number(), 42);
}

/// This function call the crates to make work the program
fn main() {
    println!("number is {}", number());
}
