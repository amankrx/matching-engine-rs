// orderbook.rs

use crate::{
    level::{Level, LevelId, PriceLevel, SortedLevels},
    order::Order,
    pool::LevelPool,
    price::Price,
    quantity::Qty,
    utils::MAX_LEVELS,
};

pub struct OrderBook {
    pub bids: SortedLevels,
    pub asks: SortedLevels,
    pub level_pool: LevelPool,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: SortedLevels::new(),
            asks: SortedLevels::new(),
            level_pool: LevelPool::new_with_capacity(MAX_LEVELS),
        }
    }

    pub fn add_order(&mut self, order: &mut Order, price: Price, qty: Qty) {
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
                order.set_level_id(LevelId(cur_level.level_idx.0));
                found_insertion_point = true;
                break;
            } else if cur_level.price < price {
                insertion_point += 1;
                break;
            }
        }

        if !found_insertion_point {
            let level_ptr = self.level_pool.alloc();
            order.level_id = LevelId(level_ptr.0);
            let level = Level::new(price, Qty(0));
            self.level_pool.allocated[level_ptr.0 as usize] = level;
            let px = PriceLevel::new(price, level_ptr);
            levels.insert(insertion_point, px);
        }
        self.level_pool.allocated[order.level_id.0 as usize].size += qty;
    }

    pub fn reduce_order(&mut self, order: &mut Order, qty: Qty) {
        self.level_pool.allocated[order.level_id.0 as usize].size -= qty;
    }

    pub fn remove_order(&mut self, order: &mut Order) {
        let order_level_id = order.level_id.0 as usize;

        self.level_pool.allocated[order_level_id].size -= order.qty;

        if self.level_pool.allocated[order_level_id].size.is_empty() {
            let level_price = self.level_pool.allocated[order_level_id].price;
            let levels = if level_price.is_bid() {
                &mut self.bids
            } else {
                &mut self.asks
            };
            levels.remove(level_price);
            self.level_pool.free(LevelId(order.level_id.0));
        }
    }
}
