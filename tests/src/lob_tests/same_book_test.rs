#![cfg(test)]
mod test {
    /// Tests for the same book and multiple levels
    /// It tests all the use cases for the same level and book
    /// like add, cancel, remove, and execute orders.
    use optimized_lob::level::LevelId;
    use optimized_lob::order::OrderId;
    use optimized_lob::orderbook_manager::OrderBookManager;
    use optimized_lob::quantity::Qty;
    use optimized_lob::utils::BookId;

    #[test]
    fn test_for_same_book_with_multiple_levels() {
        let mut orderbook_manager = OrderBookManager::new();

        orderbook_manager.add_order(OrderId(0), BookId(1), Qty(800), 500, true);
        orderbook_manager.add_order(OrderId(1), BookId(1), Qty(50), 600, true);
        orderbook_manager.add_order(OrderId(2), BookId(1), Qty(26), 600, true);

        assert_eq!(
            Qty(800),
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
            Qty(50),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(1))
                .unwrap()
                .size()
        );

        orderbook_manager.cancel_order(OrderId(0), Qty(100));
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

        orderbook_manager.remove_order(OrderId(1));
        assert_eq!(
            Qty(0),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(1))
                .unwrap()
                .size()
        );

        orderbook_manager.add_order(OrderId(3), BookId(1), Qty(50), 800, true);
        orderbook_manager.add_order(OrderId(4), BookId(1), Qty(26), 600, true);
        assert_eq!(
            Qty(50),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(1))
                .unwrap()
                .size()
        );
        assert_eq!(
            Qty(26),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(2))
                .unwrap()
                .size()
        );
        orderbook_manager.remove_order(OrderId(3));
        assert_eq!(
            Qty(0),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(1))
                .unwrap()
                .size()
        );
        orderbook_manager.remove_order(OrderId(4));
        assert_eq!(
            Qty(0),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(2))
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
        orderbook_manager.add_order(OrderId(5), BookId(1), Qty(50), 1500, true);
        orderbook_manager.add_order(OrderId(6), BookId(1), Qty(26), 500, true);
        assert_eq!(
            Qty(50),
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
            Qty(26),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(2))
                .unwrap()
                .size()
        );
        assert_eq!(
            Qty(0),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(1))
                .unwrap()
                .size()
        );
        orderbook_manager.add_order(OrderId(6), BookId(1), Qty(86), 1400, true);
        assert_eq!(
            Qty(86),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(1))
                .unwrap()
                .size()
        );
        orderbook_manager.add_order(OrderId(6), BookId(1), Qty(96), 1300, true);
        assert_eq!(
            Qty(96),
            orderbook_manager
                .books
                .get(&BookId(1))
                .unwrap()
                .level_pool
                .get(LevelId(3))
                .unwrap()
                .size()
        );
    }
}
