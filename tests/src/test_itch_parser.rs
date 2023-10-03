extern crate itch_parser;

use std::path::Path;
use std::time::Instant;

pub fn test_itch_parser(file_path: &str) {
    let path_to_market_data = Path::new(file_path);
    let stream = itch_parser::MessageStream::from_file(path_to_market_data).unwrap();

    let mut messages: u32 = 0;

    println!("------------------------------------");
    println!("ITCH Parser Processing...\n");

    let start = Instant::now();

    for _ in stream {
        messages += 1;
    }

    let duration = Instant::now() - start;
    let speed = messages as f64 / duration.as_secs_f64();

    println!("Success...\n");
    println!("ITCH Parsing Statistics:");
    println!("Total Messages: {}", messages);
    println!("Total Time: {:.3} seconds", duration.as_secs_f64());
    println!("Speed: {} msg/second", speed as u32);
    println!("Latency: {} ns", duration.as_nanos() / messages as u128);
    println!("------------------------------------");
}
