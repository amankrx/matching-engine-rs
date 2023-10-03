#![cfg(test)]
mod test {
    /// Tests for the same level and book
    /// It tests all the use cases for the same level and book
    /// like add, cancel, remove, and execute orders.
    use optimized_lob::level::LevelId;
    use optimized_lob::order::OrderId;
    use optimized_lob::orderbook_manager::OrderBookManager;
    use optimized_lob::quantity::Qty;
    use optimized_lob::utils::BookId;

    #[test]
    fn test_for_same_book_and_level() {
        let mut orderbook_manager = OrderBookManager::new();

        orderbook_manager.add_order(OrderId(0), BookId(1), Qty(800), 500, true);
        orderbook_manager.add_order(OrderId(1), BookId(1), Qty(50), 500, true);
        orderbook_manager.add_order(OrderId(2), BookId(1), Qty(26), 500, true);

        assert_eq!(
            Qty(876),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(0))
                .unwrap()
                .size()
        );

        orderbook_manager.remove_order(OrderId(2));
        assert_eq!(
            Qty(850),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(0))
                .unwrap()
                .size()
        );

        orderbook_manager.cancel_order(OrderId(0), Qty(100));
        assert_eq!(
            Qty(750),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(0))
                .unwrap()
                .size()
        );

        orderbook_manager.cancel_order(OrderId(1), Qty(50));
        assert_eq!(
            Qty(700),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(0))
                .unwrap()
                .size()
        );

        orderbook_manager.add_order(OrderId(3), BookId(1), Qty(50), 500, true);
        orderbook_manager.add_order(OrderId(4), BookId(1), Qty(26), 500, true);
        assert_eq!(
            Qty(776),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(0))
                .unwrap()
                .size()
        );

        orderbook_manager.remove_order(OrderId(3));
        assert_eq!(
            Qty(726),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(0))
                .unwrap()
                .size()
        );

        orderbook_manager.remove_order(OrderId(4));
        assert_eq!(
            Qty(700),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(0))
                .unwrap()
                .size()
        );

        orderbook_manager.remove_order(OrderId(0));
        assert_eq!(
            Qty(0),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(0))
                .unwrap()
                .size()
        );

        orderbook_manager.add_order(OrderId(5), BookId(1), Qty(50), 500, true);
        orderbook_manager.add_order(OrderId(6), BookId(1), Qty(26), 500, true);
        assert_eq!(
            Qty(76),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(0))
                .unwrap()
                .size()
        );
        orderbook_manager.execute_order(OrderId(5), Qty(50));
        assert_eq!(
            Qty(26),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(0))
                .unwrap()
                .size()
        );
        orderbook_manager.execute_order(OrderId(6), Qty(10));
        assert_eq!(
            Qty(16),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(0))
                .unwrap()
                .size()
        );
        assert_eq!(
            1,
            orderbook_manager.books.get(&BookId(1)).unwrap().bids.len()
        )
    }
}
