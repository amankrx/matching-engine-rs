extern crate itch_parser;
extern crate optimized_lob;

use std::env::args;
use std::path::Path;
use std::time::Instant;
use itch_parser::Body::{AddOrder, OrderCancelled, OrderExecuted, OrderExecutedWithPrice};
use itch_parser::{MessageStream, Side};
use optimized_lob::{
    orderbook_manager::OrderBookManager,
    order::OrderId,
    quantity::Qty,
    utils::BookId
};
use optimized_lob::price::Price;


pub fn test_lob() {
    let args: Vec<String> = args().collect();
    let path_to_market_data = Path::new(&args[1]);
    let stream = MessageStream::from_file(path_to_market_data).unwrap();

    let mut messages = 0;
    let mut add_order_count = 0;
    let mut execute_orders_count = 0;
    let mut cancel_order_count = 0;

    let start = Instant::now();
    let mut orderbook = OrderBookManager::new();

    for msg in stream {
        let unwrapped_msg = msg.unwrap();
        let stock_locate = unwrapped_msg.stock_locate;

        match unwrapped_msg.body {
            AddOrder(order) => {
                let order_id: Option<u32> = order.reference.try_into().ok();

                // Convert price_u32 to i32 based on the side
                let price = match order.side {
                    Side::Buy => order.price.raw() as i32,
                    Side::Sell => -(order.price.raw() as i32),
                };

                match order_id {
                    Some(id) => {
                        orderbook.add_order(OrderId(id), BookId(stock_locate), Qty(order.shares), Price(price));
                    }
                    None => {
                        // Conversion failed due to overflow, handle the error here
                        println!("Failed to convert Order ID u32 due to overflow");
                        break;
                    }
                }
                add_order_count += 1;
            },
            OrderExecuted {
                reference: oid,
                executed: shares,
                match_number: _,
            } => {
                let order_id: Option<u32> = oid.try_into().ok();
                match order_id {
                    Some(id) => {
                        orderbook.execute_order(OrderId(id), Qty(shares));
                    }
                    None => {
                        // Conversion failed due to overflow, handle the error here
                        println!("Failed to convert Order ID u32 due to overflow");
                        break;
                    }
                }
                execute_orders_count += 1;
            },
            OrderExecutedWithPrice {
                reference: oid,
                executed: shares,
                match_number: _,
                printable: _,
                price: _,
            } => {
                let order_id: Option<u32> = oid.try_into().ok();
                match order_id {
                    Some(id) => {
                        orderbook.execute_order(OrderId(id), Qty(shares));
                    }
                    None => {
                        // Conversion failed due to overflow, handle the error here
                        println!("Failed to convert Order ID u32 due to overflow");
                        break;
                    }
                }
                execute_orders_count += 1;
            },
            OrderCancelled {
                reference: oid,
                cancelled: shares,
            } => {
                let order_id: Option<u32> = oid.try_into().ok();
                match order_id {
                    Some(id) => {
                        orderbook.cancel_order(OrderId(id), Qty(shares));
                    }
                    None => {
                        // Conversion failed due to overflow, handle the error here
                        println!("Failed to convert Order ID u32 due to overflow");
                        break;
                    }
                }
                cancel_order_count += 1;
            },
            _ => {}
        }

        messages += 1;
    }


    let duration = Instant::now() - start;
    let speed = messages / duration.as_secs();

    println!("------------------------------------");
    println!("ITCH Message Processing\n");
    println!("Total Messages: {}", messages);
    println!("ITCH Latency: {} ns", duration.as_nanos() / messages as u128);
    println!("Total Time: {:.3} seconds", duration.as_secs_f64());
    println!("Speed: {} msg/second", speed);
    println!("Total Add Orders: {}", add_order_count);
    println!("Total Execute Orders: {}", execute_orders_count);
    println!("Total Cancel Orders: {}", cancel_order_count);
    println!("------------------------------------");
}