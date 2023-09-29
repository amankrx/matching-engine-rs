// orderbook.rs

use crate::orderbook::level::PriceLevel;
use crate::orderbook::quantity::Qty;
use crate::orderbook::{
    level::{Level, LevelId, SortedLevels},
    order::Order,
    pool::Pool,
    price::Price,
    utils::*,
};

pub struct OrderBook {
    pub bids: SortedLevels,
    pub asks: SortedLevels,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: SortedLevels::new(),
            asks: SortedLevels::new(),
        }
    }

    pub fn add_order(
        &mut self,
        order: &mut Order,
        price: Price,
        qty: Qty,
        level_pool: &mut Pool<Level>,
    ) {
        let levels = if price.is_bid() {
            &mut self.bids
        } else {
            &mut self.asks
        };

        let mut insertion_point = levels.0.len();
        let mut found_insertion_point = false;

        while insertion_point > 0 {
            insertion_point -= 1;
            let cur_level = &mut levels.0[insertion_point];

            if cur_level.price == price {
                order.set_level_id(LevelId(insertion_point as u32));
                found_insertion_point = true;
                break;
            } else if cur_level.price < price {
                insertion_point += 1;
                break;
            }
        }

        if !found_insertion_point {
            let level_ptr = level_pool.alloc();
            order.level_id = LevelId(level_ptr.0);
            let level = Level::new(price, Qty(0));
            level_pool.allocated[level_ptr.0 as usize] = level;
            let px = PriceLevel::new(price, level_ptr);
            levels.insert(insertion_point, px);
        }
        level_pool.allocated[order.level_id.0 as usize].size += qty;
    }

    pub fn reduce_order(&mut self, order: &mut Order, qty: Qty, level_pool: &mut Pool<Level>) {
        level_pool.allocated[order.level_id.0 as usize].size -= qty;
    }

    pub fn remove_order(&mut self, order: &mut Order, level_pool: &mut Pool<Level>) {
        let order_level_id = order.level_id.0 as usize;

        level_pool.allocated[order_level_id].size -= order.qty;

        if level_pool.allocated[order_level_id].size.is_empty() {
            let level_price = level_pool.allocated[order_level_id].price;
            let levels = if level_price.is_bid() {
                &mut self.bids
            } else {
                &mut self.asks
            };
            levels.remove(level_price);
            level_pool.free(Ptr(order.level_id.0));
        }
    }
}
