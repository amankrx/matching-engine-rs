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

impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderBook {
    #[inline]
    pub fn new() -> Self {
        Self {
            bids: SortedLevels::new(),
            asks: SortedLevels::new(),
            level_pool: LevelPool::new_with_capacity(MAX_LEVELS),
        }
    }

    #[inline]
    pub fn add_order(&mut self, order: &mut Order, price: Price, qty: Qty) {
        let levels = if price.is_bid() {
            &mut self.bids
        } else {
            &mut self.asks
        };

        let mut insertion_point = levels.len();
        let mut found_insertion_point = false;

        while insertion_point > 0 {
            insertion_point -= 1;
            let cur_level = levels.get_mut(insertion_point);

            match cur_level.price().cmp(&price) {
                std::cmp::Ordering::Equal => {
                    order.set_level_id(LevelId(cur_level.level_id().value()));
                    found_insertion_point = true;
                    break;
                }
                std::cmp::Ordering::Less => {
                    insertion_point += 1;
                    break;
                }
                _ => {}
            }
        }

        if !found_insertion_point {
            let level_ptr = self.level_pool.alloc();
            order.set_level_id(level_ptr);
            let level = Level::new(price, Qty(0));
            self.level_pool.set_level(level_ptr, level);
            let px = PriceLevel::new(price, level_ptr);
            levels.insert(insertion_point, px);
        }
        self.level_pool.get_mut(order.level_id()).unwrap().incr(qty);
    }

    #[inline]
    pub fn reduce_order(&mut self, order: &mut Order, qty: Qty) {
        self.level_pool
            .get_mut(LevelId(order.level_id().value()))
            .unwrap()
            .decr(qty);
    }

    #[inline]
    pub fn remove_order(&mut self, order: &mut Order) {
        let lvl = self.level_pool.get_mut(order.level_id()).unwrap();
        lvl.decr(order.qty());

        if lvl.size().is_empty() {
            let level_price = lvl.price();
            let levels = if level_price.is_bid() {
                &mut self.bids
            } else {
                &mut self.asks
            };
            levels.remove(level_price);
            self.level_pool.free(LevelId(order.level_id().value()));
        }
    }
}
