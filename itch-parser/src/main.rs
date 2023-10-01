// main.rs

use itch_parser;
use std::env::args;
use std::path::Path;
use std::time::Instant;

fn main() {
    let args: Vec<String> = args().collect();
    let path_to_market_data = Path::new(&args[1]);
    let stream = itch_parser::MessageStream::from_file(path_to_market_data).unwrap();

    let mut messages = 0;

    let start = Instant::now();

    for _ in stream {
        messages += 1;
    }

    let duration = Instant::now() - start;
    let speed = messages / duration.as_secs();

    println!("------------------------------------");
    println!("ITCH Message Processing\n");
    println!("Total Messages: {}", messages);
    println!("Total Time: {:.3} seconds", duration.as_secs_f64());
    println!("Speed: {} messages per second", speed);
    println!("Latency: {} ns", duration.as_nanos() / messages as u128);
    println!("------------------------------------");
}