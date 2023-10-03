# optimized-lob

This library contains an optimized implementation of a Limit Order Book (LOB). It keeps the aggregate quantities at each level. 

Note: Will add the comments describing the project later on.

## Usage

- Add the library to your project.
- Import the `optimized_lob` struct from `optimized-lob`.
- The `orderbook_manager` struct contains all the functions you need to manage the LOB. You can perform add, cancel, execute, delete, and replace operations on the LOB.
- There are individual wrapper structs as well that you might need to store price, quantity, order ID, book ID, etc.

## Examples
A simple example demonstrating the use of library to add, delete, and replace orders.
```rust
extern crate optimized_lob;
use optimized_lob::{
    order::OrderId, orderbook_manager::OrderBookManager, quantity::Qty, utils::BookId,
};

fn test_lob() {
    let mut orderbook_manager = OrderBookManager::new();

    // Add an order
    orderbook_manager.add_order(
        OrderId(0), // Order ID
        BookId(0), // Book ID
        Qty(100), // Quantity
        600, // Price
        true, // Is Bid
    );
    
    // Replace an order
    orderbook_manager.replace_order(
        OrderId(0), // Order ID to replace
        OrderId(1), // New Order ID
        Qty(50), // New Quantity
        500, // New Price
    );
    
    // Remove an order
    orderbook_manager.remove_order(OrderId(1));
}
```

See Also: 
- [CppTrader](https://github.com/chronoxor/CppTrader) matching engine implementation
- A [StackOverflow answer](https://quant.stackexchange.com/questions/3783/what-is-an-efficient-data-structure-to-model-order-book/32482#32482) along with his implementation of an [optimized LOB](https://github.com/charles-cooper/itch-order-book/). I have specifically followed the much of the design idea from his implementation.
- This [blog post](https://web.archive.org/web/20110219163448/http://howtohft.wordpress.com/2011/02/15/how-to-build-a-fast-limit-order-book/) gives a good idea for the low-level design of the orderbook.
