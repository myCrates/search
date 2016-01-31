#![crate_name = "search"]
#![crate_type = "bin"]
#![doc(html_root_url = "https://mycrates.github.io/search/")]

//! # Search binary
//!
//! This is a binary to can make fast search on internet from the terminal.
//! Try to use it and let me know how do you think about it !!!

/// Call main library
//extern crate search;
/// Load main library functions
//use search::*;

/// Import clap library
#[macro_use]
extern crate clap;

/// # Main function
///
/// Parse aguments with clap and call search library
fn main() {
    use clap::App;
    // The YAML file is found relative to the current file, similar to how modules are found
    let yml = load_yaml!("cli/english.yml");
    let matches = App::from_yaml(yml).get_matches();

    // Because the example 17_yaml.yml is rather large we'll just look a single arg so you can
    // see that it works...
    if let Some(mode) = matches.value_of("mode") {
        match mode {
            "fast" => println!("We're really going now!"),
            "slow" => println!("Awwww, too slow :("),
            _      => unreachable!()
        }
    } else {
        println!("--mode <MODE> wasn't used...");
    }
}
