// orderbook_manager.rs

use crate::{
    level::LevelId,
    order::{OidMap, OrderId, Order},
    orderbook::OrderBook,
    price::Price,
    quantity::Qty,
    utils::BookId,
};
use std::collections::HashMap;

pub struct OrderBookManager {
    books: HashMap<BookId, OrderBook>,
    oid_map: OidMap,
}

impl OrderBookManager {
    pub fn new() -> Self {
        Self {
            books: HashMap::new(),
            oid_map: OidMap::new(),
        }
    }

    pub fn add_order(&mut self, order_id: OrderId, book_id: BookId, qty: Qty, price32: u32, is_bid: bool) {
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

    pub fn remove_order(&mut self, order_id: OrderId) {
        if let Some(mut order) = self.oid_map.get_mut(order_id) {
            self.books
                .get_mut(&order.book_id)
                .unwrap()
                .remove_order(&mut order);
        }
        self.oid_map.remove(order_id);
    }

    pub fn cancel_order(&mut self, order_id: OrderId, qty: Qty) {
        if let Some(mut order) = self.oid_map.get_mut(order_id) {
            if order.qty == qty {
                self.books
                    .get_mut(&order.book_id)
                    .unwrap()
                    .remove_order(&mut order);
                self.oid_map.remove(order_id);
            } else {
                self.books
                    .get_mut(&order.book_id)
                    .unwrap()
                    .reduce_order(&mut order, qty);
                self.oid_map.update_qty(order_id, qty);
            }
        }
    }

    pub fn execute_order(&mut self, order_id: OrderId, qty: Qty) {
        if let Some(mut order) = self.oid_map.get_mut(order_id) {
            if order.qty == qty {
                self.books
                    .get_mut(&order.book_id)
                    .unwrap()
                    .remove_order(&mut order);
                self.oid_map.remove(order_id);
            } else {
                self.books
                    .get_mut(&order.book_id)
                    .unwrap()
                    .reduce_order(&mut order, qty);
                self.oid_map.update_qty(order_id, qty);
            }
        }
    }

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
        if let Some(mut order) = order {
            let book = self.books.get_mut(&order.book_id).unwrap();
            is_bid = book.level_pool.allocated[order.level_id.0 as usize].price.is_bid();
            book_id = order.book_id;
            book.remove_order(&mut order);
            self.oid_map.remove(order_id);
        }
        self.add_order(new_order_id, book_id, new_qty, new_price, is_bid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_order() {
        let mut orderbook_manager = OrderBookManager::new();

        orderbook_manager.add_order(OrderId(0), BookId(0), Qty(100), 600, true);
        orderbook_manager.add_order(OrderId(1), BookId(0), Qty(50), 500, true);
        orderbook_manager.add_order(OrderId(2), BookId(0), Qty(70), 300, true);
        orderbook_manager.add_order(OrderId(3), BookId(0), Qty(80), 400, true);
        orderbook_manager.add_order(OrderId(4), BookId(0), Qty(90), 200, true);
        orderbook_manager.add_order(OrderId(5), BookId(0), Qty(100), 50, true);
        orderbook_manager.add_order(OrderId(6), BookId(0), Qty(100), 100, true);
        orderbook_manager.add_order(OrderId(7), BookId(1), Qty(50), 100, true);
        orderbook_manager.add_order(OrderId(8), BookId(2), Qty(70), 200, true);
        orderbook_manager.add_order(OrderId(9), BookId(1), Qty(80), 300, true);
        orderbook_manager.add_order(OrderId(10), BookId(1), Qty(90), 400, true);
        orderbook_manager.add_order(OrderId(34), BookId(2), Qty(100), 50, true);
        orderbook_manager.add_order(OrderId(13), BookId(0), Qty(80), 400, false);
        orderbook_manager.add_order(OrderId(14), BookId(0), Qty(90), 200, false);
        orderbook_manager.add_order(OrderId(15), BookId(0), Qty(100), 50, false);
        orderbook_manager.add_order(OrderId(16), BookId(0), Qty(100), 100, false);
        println!("Test");
    }


    #[test]
    fn test_remove_order() {
        let mut orderbook_manager = OrderBookManager::new();

        orderbook_manager.add_order(OrderId(0), BookId(0), Qty(100), 600, true);
        orderbook_manager.add_order(OrderId(1), BookId(0), Qty(50), 500, true);
        orderbook_manager.add_order(OrderId(2), BookId(0), Qty(70), 300, true);
        orderbook_manager.add_order(OrderId(3), BookId(0), Qty(80), 400, true);
        orderbook_manager.add_order(OrderId(4), BookId(0), Qty(90), 200, true);
        orderbook_manager.add_order(OrderId(5), BookId(0), Qty(100), 50, true);
        orderbook_manager.add_order(OrderId(6), BookId(0), Qty(100), 100, true);
        orderbook_manager.add_order(OrderId(7), BookId(1), Qty(50), 100, true);
        orderbook_manager.add_order(OrderId(8), BookId(2), Qty(70), 200, true);
        orderbook_manager.add_order(OrderId(9), BookId(1), Qty(80), 300, true);

        orderbook_manager.remove_order(OrderId(0));
        orderbook_manager.remove_order(OrderId(1));
        orderbook_manager.remove_order(OrderId(4));
        orderbook_manager.remove_order(OrderId(7));

        orderbook_manager.add_order(OrderId(10), BookId(0), Qty(100), 600, true);
        orderbook_manager.add_order(OrderId(0), BookId(0), Qty(100), 600, true);
    }

    #[test]
    fn test_random_remove_order() {
        let mut orderbook_manager = OrderBookManager::new();

        orderbook_manager.add_order(OrderId(0), BookId(0), Qty(100), 600, true);
        orderbook_manager.add_order(OrderId(1), BookId(0), Qty(50), 600, true);
        orderbook_manager.remove_order(OrderId(0));
        orderbook_manager.remove_order(OrderId(1));
        orderbook_manager.add_order(OrderId(2), BookId(0), Qty(70), 300, true);
        orderbook_manager.add_order(OrderId(3), BookId(0), Qty(80), 400, true);
        orderbook_manager.add_order(OrderId(4), BookId(0), Qty(90), 200, true);
        orderbook_manager.remove_order(OrderId(3));
        orderbook_manager.add_order(OrderId(5), BookId(0), Qty(100), 50, true);
        orderbook_manager.add_order(OrderId(6), BookId(0), Qty(100), 100, true);
        orderbook_manager.add_order(OrderId(7), BookId(1), Qty(50), 100, true);
        orderbook_manager.add_order(OrderId(8), BookId(2), Qty(70), 200, true);
        orderbook_manager.add_order(OrderId(9), BookId(1), Qty(80), 300, true);

        orderbook_manager.remove_order(OrderId(4));
        orderbook_manager.remove_order(OrderId(7));

        orderbook_manager.add_order(OrderId(10), BookId(0), Qty(100), 600, true);
        orderbook_manager.add_order(OrderId(0), BookId(0), Qty(100), 600, true);
    }


    #[test]
    fn test_cancel_order() {
        let mut orderbook_manager = OrderBookManager::new();

        orderbook_manager.add_order(OrderId(0), BookId(0), Qty(100), 600, true);
        orderbook_manager.add_order(OrderId(1), BookId(0), Qty(50), 500, true);
        orderbook_manager.add_order(OrderId(2), BookId(0), Qty(70), 300, true);
        orderbook_manager.add_order(OrderId(3), BookId(0), Qty(80), 400, true);
        orderbook_manager.add_order(OrderId(4), BookId(0), Qty(90), 200, true);
        orderbook_manager.add_order(OrderId(5), BookId(0), Qty(100), 50, true);
        orderbook_manager.add_order(OrderId(6), BookId(0), Qty(100), 100, true);
        orderbook_manager.add_order(OrderId(7), BookId(1), Qty(50), 100, true);
        orderbook_manager.add_order(OrderId(8), BookId(2), Qty(70), 200, true);
        orderbook_manager.add_order(OrderId(9), BookId(1), Qty(80), 300, true);

        orderbook_manager.cancel_order(OrderId(0), Qty(50));
        orderbook_manager.cancel_order(OrderId(1), Qty(25));
        orderbook_manager.cancel_order(OrderId(4), Qty(25));
        orderbook_manager.cancel_order(OrderId(7), Qty(25));
        orderbook_manager.cancel_order(OrderId(8), Qty(70));

        print!("Done")
    }

    #[test]
    fn test_for_same_book_with_multiple_levels() {
        let mut orderbook_manager = OrderBookManager::new();

        orderbook_manager.add_order(OrderId(0), BookId(1), Qty(800), 500, true);
        orderbook_manager.add_order(OrderId(1), BookId(1), Qty(50), 600, true);
        orderbook_manager.add_order(OrderId(2), BookId(1), Qty(26), 600, true);

        assert_eq!(Qty(800), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[0].size);

        orderbook_manager.remove_order(OrderId(2));
        assert_eq!(Qty(50), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[1].size);

        orderbook_manager.cancel_order(OrderId(0), Qty(100));
        assert_eq!(Qty(700), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[0].size);

        orderbook_manager.cancel_order(OrderId(1), Qty(50));
        assert_eq!(Qty(0), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[1].size);

        orderbook_manager.add_order(OrderId(3), BookId(1), Qty(50), 800, true);
        orderbook_manager.add_order(OrderId(4), BookId(1), Qty(26), 600, true);
        assert_eq!(Qty(50), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[1].size);
        assert_eq!(Qty(26), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[2].size);
        orderbook_manager.remove_order(OrderId(3));
        assert_eq!(Qty(0), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[1].size);
        orderbook_manager.remove_order(OrderId(4));
        assert_eq!(Qty(0), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[2].size);
        orderbook_manager.remove_order(OrderId(0));
        assert_eq!(Qty(0), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[0].size);
        orderbook_manager.add_order(OrderId(5), BookId(1), Qty(50), 1500, true);
        orderbook_manager.add_order(OrderId(6), BookId(1), Qty(26), 500, true);
        assert_eq!(Qty(50), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[0].size);
        assert_eq!(Qty(26), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[2].size);
        assert_eq!(Qty(0), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[1].size);
        orderbook_manager.add_order(OrderId(6), BookId(1), Qty(86), 1400, true);
        assert_eq!(Qty(86), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[1].size);
        orderbook_manager.add_order(OrderId(6), BookId(1), Qty(96), 1300, true);
        assert_eq!(Qty(96), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[3].size);
    }

    #[test]
    fn test_replace_order() {
        let mut orderbook_manager = OrderBookManager::new();

        orderbook_manager.add_order(OrderId(0), BookId(1), Qty(800), 500, true);
        orderbook_manager.add_order(OrderId(1), BookId(1), Qty(50), 500, true);
        orderbook_manager.add_order(OrderId(2), BookId(1), Qty(26), 500, true);

        assert_eq!(Qty(876), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[0].size);

        orderbook_manager.replace_order(OrderId(2), OrderId(3), Qty(50), 400);
        assert_eq!(Qty(50), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[1].size);

        orderbook_manager.add_order(OrderId(4), BookId(1), Qty(26), 500, false);

        assert_eq!(Qty(850), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[0].size);

        orderbook_manager.replace_order(OrderId(4), OrderId(5), Qty(50), 400);
        assert_eq!(Qty(50), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[2].size);
    }

    #[test]
    fn test_for_same_book_and_level() {
        let mut orderbook_manager = OrderBookManager::new();

        orderbook_manager.add_order(OrderId(0), BookId(1), Qty(800), 500, true);
        orderbook_manager.add_order(OrderId(1), BookId(1), Qty(50), 500, true);
        orderbook_manager.add_order(OrderId(2), BookId(1), Qty(26), 500, true);

        assert_eq!(Qty(876), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[0].size);

        orderbook_manager.remove_order(OrderId(2));
        assert_eq!(Qty(850), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[0].size);

        orderbook_manager.cancel_order(OrderId(0), Qty(100));
        assert_eq!(Qty(750), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[0].size);

        orderbook_manager.cancel_order(OrderId(1), Qty(50));
        assert_eq!(Qty(700), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[0].size);

        orderbook_manager.add_order(OrderId(3), BookId(1), Qty(50), 500, true);
        orderbook_manager.add_order(OrderId(4), BookId(1), Qty(26), 500, true);
        assert_eq!(Qty(776), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[0].size);

        orderbook_manager.remove_order(OrderId(3));
        assert_eq!(Qty(726), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[0].size);

        orderbook_manager.remove_order(OrderId(4));
        assert_eq!(Qty(700), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[0].size);

        orderbook_manager.remove_order(OrderId(0));
        assert_eq!(Qty(0), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[0].size);

        orderbook_manager.add_order(OrderId(5), BookId(1), Qty(50), 500, true);
        orderbook_manager.add_order(OrderId(6), BookId(1), Qty(26), 500, true);
        assert_eq!(Qty(76), orderbook_manager.books.get(&BookId(1)).unwrap().level_pool.allocated[0].size);
    }
}

#[test]
fn test_million_orders() {
    let mut orderbook_manager = OrderBookManager::new();
    let start = std::time::Instant::now();

    for i in 0..10_000_000 {
        orderbook_manager.add_order(
            OrderId(i),
            BookId((i % 50) as u16),
            Qty(100),
            100 * (i % 20),
            i % 2 == 0,
        );
    }

    println!("{:?}", start.elapsed());
    println!(
        "Latency: {:?}",
        start.elapsed().as_nanos() / 10_000_000.0 as u128
    );
}