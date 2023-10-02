# ITCH Parser
A nom-based Rust parser for parsing NASDAQ ITCH Protocol 5.0 based data. 
I initially intended to use the [itchy-rust](https://github.com/adwhit/itchy-rust) library directly which is in itself a robust way to handle the ITCH data, but much of the libraries that are used in it are outdated, and maybe rejected in future versions. 
For eg: it still uses `v4.x` of nom, when `v7.x` are available now. 
But still though much of the logic 

Note: The parser is incomplete since I just included a few useful operations for my optimized-lob. For a robust parser please use the [itchy-rust](https://github.com/adwhit/itchy-rust) library.

## Usage

- Add the library to your project.
- Import the `itch_parser` struct from `itch-parser`.
- It contains all the functions you need to parse the ITCH data.

## Examples
A simple example to use the stream.
```rust
extern crate itch_parser;

use std::path::Path;

pub fn test_itch_parser(file_path: &str) {
    let path_to_market_data = Path::new(file_path);
    let stream = itch_parser::MessageStream::from_file(path_to_market_data).unwrap();

    let mut messages: u32 = 0;
    for _ in stream {
        messages += 1;
    }
}
```

Using the stream to parse with the message types:
```rust
extern crate itch_parser;

use itch_parser::Body::{
    AddOrder, DeleteOrder, OrderCancelled
};
use itch_parser::MessageStream;
use std::path::Path;
use std::time::Instant;

pub fn test_parser(file_path: &str) {
    let path_to_market_data = Path::new(file_path);
    let stream = MessageStream::from_file(path_to_market_data).unwrap();

    // Counters
    let mut messages = 0;
    let mut add_order_count = 0;
    let mut execute_orders_count = 0;
    let mut cancel_order_count = 0;

    // Process messages
    for msg in stream {
        let unwrapped_msg = msg.unwrap();

        match unwrapped_msg.body {
            AddOrder {
                order_id: _,
                is_bid: _,
                shares: _,
                stock: _,
                price: _,
            } => {
                add_order_count += 1;
            }
            OrderCancelled { order_id: _, shares: _ } => {
                cancel_order_count += 1;
            }
            DeleteOrder { order_id: _ } => {
                delete_order_count += 1;
            }
            _ => {}
        }
        messages += 1;
    } 
}
```
