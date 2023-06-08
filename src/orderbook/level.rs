// orderbook/level.rs
//  * Author: Aman Kumar <aman@amankrx.com>
//  * Created: Wed Jun 07 2023
//  * Last Modified: Wed Jun 07 2023
//  * Description: Orderbook Level
//  * License: Distributed under the terms of the MIT License

use std::cmp::Ordering;

use crate::orderbook::order::{Order, OrderSide};

pub struct Level {
    pub price: u64,
    pub side: OrderSide,
    pub orders: u64,
    pub total_volume: u64,
}

impl Level {
    pub fn new(price: u64, side: OrderSide) -> Level {
        Level {
            price,
            side,
            orders: 0,
            total_volume: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.orders == 0
    }

    pub fn is_bid(&self) -> bool {
        matches!(self.side, OrderSide::Bid)
    }

    pub fn is_ask(&self) -> bool {
        matches!(self.side, OrderSide::Ask)
    }
}

impl PartialEq for Level {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Level {}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.price.partial_cmp(&other.price)
    }
}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price)
    }
}

pub struct LevelNode {
    pub level: Level,
    pub orders: Vec<Order>,
}
