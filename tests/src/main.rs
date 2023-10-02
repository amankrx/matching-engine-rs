// main.rs
mod test_itch_parser;
mod test_lob;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse command-line arguments
    let test_to_run = if args.len() > 1 && args[1] == "--itch-parser" {
        // If the "--itch-parser" flag is provided, run the itch parser test
        "itch_parser"
    } else {
        // Default to running the test_lob
        "test_lob"
    };

    // Set the file path using an environment variable
    let file_path = env::var("ITCH_DATA").unwrap_or_else(|_| {
        panic!("ITCH_DATA environment variable not set");
    });

    match test_to_run {
        "itch_parser" => test_itch_parser::test_itch_parser(&file_path),
        "test_lob" => test_lob::test_lob(&file_path),
        _ => println!("Invalid test specified"),
    }
}
