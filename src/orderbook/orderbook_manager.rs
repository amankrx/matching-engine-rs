// orderbook_manager.rs

use crate::orderbook::order::Order;
use crate::orderbook::{
    level::{Level, LevelId},
    order::{OidMap, OrderId},
    orderbook::OrderBook,
    pool::Pool,
    price::Price,
    quantity::Qty,
    utils::*,
};
use std::collections::HashMap;

const MAX_BOOKS: usize = 1 << 14;
const MAX_LEVELS: usize = 1 << 20;

struct OrderBookManager {
    books: HashMap<BookId, OrderBook>,
    levels: Pool<Level>,
    oid_map: OidMap,
}

impl OrderBookManager {
    pub fn new() -> Self {
        Self {
            books: HashMap::new(),
            levels: Pool::new_with_capacity(MAX_LEVELS),
            oid_map: OidMap::new(),
        }
    }

    pub fn add_order(&mut self, order_id: OrderId, book_id: BookId, qty: Qty, price: Price) {
        self.oid_map.reserve(order_id);

        let mut order = Order::new(qty, LevelId(0), book_id);

        // Now you can use order as needed
        self.books
            .entry(book_id)
            .or_insert_with(OrderBook::new)
            .add_order(&mut order, price, qty, &mut self.levels);

        self.oid_map.insert(order_id, &order);
    }

    pub fn remove_order(&mut self, order_id: OrderId) {
        if let Some(mut order) = self.oid_map.get_mut(order_id) {
            // Pass a reference to the OrderBook's remove_order function
            self.books
                .get_mut(&order.book_id)
                .unwrap()
                .remove_order(&mut order, &mut self.levels);
        }
        self.oid_map.remove(order_id);
    }

    pub fn cancel_order(&mut self, order_id: OrderId, qty: Qty) {
        let order = self.oid_map.get_mut(order_id);
        if let Some(mut order) = order {
            self.books.get_mut(&order.book_id).unwrap().reduce_order(
                &mut order,
                qty,
                &mut self.levels,
            );
        }
        self.oid_map.update_qty(order_id, qty);
    }

    pub fn execute_order(&mut self, order_id: OrderId, qty: Qty) {
        let order = self.oid_map.get_mut(order_id);
        if let Some(order) = order {
            if order.qty == qty {
                self.remove_order(order_id);
            } else {
                self.cancel_order(order_id, qty);
            }
        }
    }

    pub fn replace_order(
        &mut self,
        order_id: OrderId,
        new_order_id: OrderId,
        new_qty: Qty,
        new_price: Price,
    ) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orderbook::utils::BookId;

    #[test]
    fn test_add_order() {
        let mut orderbook_manager = OrderBookManager::new();

        orderbook_manager.add_order(OrderId(0), BookId(0), Qty(100), Price(600));
        orderbook_manager.add_order(OrderId(1), BookId(0), Qty(50), Price(500));
        orderbook_manager.add_order(OrderId(2), BookId(0), Qty(70), Price(300));
        orderbook_manager.add_order(OrderId(3), BookId(0), Qty(80), Price(400));
        orderbook_manager.add_order(OrderId(4), BookId(0), Qty(90), Price(200));
        orderbook_manager.add_order(OrderId(5), BookId(0), Qty(100), Price(50));
        orderbook_manager.add_order(OrderId(6), BookId(0), Qty(100), Price(100));
        orderbook_manager.add_order(OrderId(7), BookId(1), Qty(50), Price(100));
        orderbook_manager.add_order(OrderId(8), BookId(2), Qty(70), Price(200));
        orderbook_manager.add_order(OrderId(9), BookId(1), Qty(80), Price(300));
        orderbook_manager.add_order(OrderId(10), BookId(1), Qty(90), Price(400));
        orderbook_manager.add_order(OrderId(34), BookId(2), Qty(100), Price(50));
        orderbook_manager.add_order(OrderId(13), BookId(0), Qty(80), Price(-400));
        orderbook_manager.add_order(OrderId(14), BookId(0), Qty(90), Price(-200));
        orderbook_manager.add_order(OrderId(15), BookId(0), Qty(100), Price(-50));
        orderbook_manager.add_order(OrderId(16), BookId(0), Qty(100), Price(-100));
        println!("Test");
    }

    #[test]
    fn test_remove_order() {
        let mut orderbook_manager = OrderBookManager::new();
        orderbook_manager.add_order(OrderId(0), BookId(0), Qty(100), Price(600));
        orderbook_manager.add_order(OrderId(1), BookId(0), Qty(50), Price(500));
        orderbook_manager.add_order(OrderId(2), BookId(0), Qty(70), Price(300));
        orderbook_manager.add_order(OrderId(3), BookId(0), Qty(80), Price(400));
        orderbook_manager.add_order(OrderId(4), BookId(0), Qty(90), Price(200));
        orderbook_manager.add_order(OrderId(5), BookId(0), Qty(100), Price(50));
        orderbook_manager.add_order(OrderId(6), BookId(0), Qty(100), Price(100));
        orderbook_manager.add_order(OrderId(7), BookId(1), Qty(50), Price(100));
        orderbook_manager.add_order(OrderId(8), BookId(2), Qty(70), Price(200));
        orderbook_manager.add_order(OrderId(9), BookId(1), Qty(80), Price(300));

        orderbook_manager.remove_order(OrderId(0));
        orderbook_manager.remove_order(OrderId(1));
        orderbook_manager.remove_order(OrderId(4));
        orderbook_manager.remove_order(OrderId(7));
    }

    #[test]
    fn test_cancel_order() {
        let mut orderbook_manager = OrderBookManager::new();
        orderbook_manager.add_order(OrderId(0), BookId(0), Qty(100), Price(600));
        orderbook_manager.add_order(OrderId(1), BookId(0), Qty(50), Price(500));
        orderbook_manager.add_order(OrderId(2), BookId(0), Qty(70), Price(300));
        orderbook_manager.add_order(OrderId(3), BookId(0), Qty(80), Price(400));
        orderbook_manager.add_order(OrderId(4), BookId(0), Qty(90), Price(200));
        orderbook_manager.add_order(OrderId(5), BookId(0), Qty(100), Price(50));
        orderbook_manager.add_order(OrderId(6), BookId(0), Qty(100), Price(100));
        orderbook_manager.add_order(OrderId(7), BookId(1), Qty(50), Price(100));
        orderbook_manager.add_order(OrderId(8), BookId(2), Qty(70), Price(200));
        orderbook_manager.add_order(OrderId(9), BookId(1), Qty(80), Price(300));

        orderbook_manager.cancel_order(OrderId(0), Qty(50));
        orderbook_manager.cancel_order(OrderId(1), Qty(25));
        orderbook_manager.cancel_order(OrderId(4), Qty(25));
        orderbook_manager.cancel_order(OrderId(7), Qty(25));
        orderbook_manager.cancel_order(OrderId(8), Qty(70));
        print!("Done")
    }

    #[test]
    fn test_million_orders() {
        let mut orderbook_manager = OrderBookManager::new();
        let start = std::time::Instant::now();

        for i in 0..10_000_000 {
            orderbook_manager.add_order(
                OrderId(i),
                BookId(0),
                Qty(100),
                Price((100 * (i % 20)) as i32),
            );
        }

        println!("{:?}", start.elapsed());
        println!(
            "Latency: {:?}",
            start.elapsed().as_nanos() / 10_000_000.0 as u128
        );
    }
}
