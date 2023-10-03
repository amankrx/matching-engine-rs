// orderbook_manager.rs

use crate::{
    level::LevelId,
    order::{OidMap, Order, OrderId},
    orderbook::OrderBook,
    price::Price,
    quantity::Qty,
    utils::BookId,
};
use std::collections::HashMap;

pub struct OrderBookManager {
    pub books: HashMap<BookId, OrderBook>,
    pub oid_map: OidMap,
}

impl Default for OrderBookManager {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderBookManager {
    #[inline]
    pub fn new() -> Self {
        Self {
            books: HashMap::new(),
            oid_map: OidMap::new(),
        }
    }

    #[inline]
    pub fn add_order(
        &mut self,
        order_id: OrderId,
        book_id: BookId,
        qty: Qty,
        price32: u32,
        is_bid: bool,
    ) {
        let price_i32 = if is_bid {
            price32 as i32
        } else {
            -(price32 as i32)
        };

        // Create a Price(i32) from the adjusted price_i32
        let price = Price(price_i32);

        self.oid_map.reserve(order_id);

        let mut order = Order::new(qty, LevelId(0), book_id);

        // Now you can use order as needed
        self.books
            .entry(book_id)
            .or_insert_with(OrderBook::new)
            .add_order(&mut order, price, qty);

        self.oid_map.insert(order_id, &order);
    }

    #[inline]
    pub fn remove_order(&mut self, order_id: OrderId) {
        if let Some(order) = self.oid_map.get_mut(order_id) {
            self.books
                .get_mut(&order.book_id())
                .unwrap()
                .remove_order(order);
        }
        self.oid_map.remove(order_id);
    }

    #[inline]
    pub fn cancel_order(&mut self, order_id: OrderId, qty: Qty) {
        if let Some(order) = self.oid_map.get_mut(order_id) {
            self.books
                .get_mut(&order.book_id())
                .unwrap()
                .reduce_order(order, qty);
            self.oid_map.update_qty(order_id, qty);
        }
    }

    #[inline]
    pub fn execute_order(&mut self, order_id: OrderId, qty: Qty) {
        if let Some(order) = self.oid_map.get_mut(order_id) {
            if order.qty() == qty {
                self.books
                    .get_mut(&order.book_id())
                    .unwrap()
                    .remove_order(order);
                self.oid_map.remove(order_id);
            } else {
                self.books
                    .get_mut(&order.book_id())
                    .unwrap()
                    .reduce_order(order, qty);
                self.oid_map.update_qty(order_id, qty);
            }
        }
    }

    #[inline]
    pub fn replace_order(
        &mut self,
        order_id: OrderId,
        new_order_id: OrderId,
        new_qty: Qty,
        new_price: u32,
    ) {
        let order = self.oid_map.get_mut(order_id);
        let mut is_bid = true;
        let mut book_id = BookId(0);
        if let Some(order) = order {
            let book = self.books.get_mut(&order.book_id()).unwrap();
            is_bid = book
                .level_pool
                .get(order.level_id())
                .unwrap()
                .price()
                .is_bid();
            book_id = order.book_id();
            book.remove_order(order);
            self.oid_map.remove(order_id);
        }
        self.add_order(new_order_id, book_id, new_qty, new_price, is_bid);
    }
}
