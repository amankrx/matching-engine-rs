#![cfg(test)]
mod test {
    use crate::lob_tests::utils::get_level_capacity;
    /// Tests for the same book and multiple levels
    /// It tests all the use cases for the same level and book
    /// like add, cancel, remove, and execute orders.
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

        assert_eq!(Qty(800), get_level_capacity(&orderbook_manager, 1, 0));

        orderbook_manager.remove_order(OrderId(2));
        assert_eq!(Qty(50), get_level_capacity(&orderbook_manager, 1, 1));

        orderbook_manager.cancel_order(OrderId(0), Qty(100));
        assert_eq!(Qty(700), get_level_capacity(&orderbook_manager, 1, 0));

        orderbook_manager.remove_order(OrderId(1));
        assert_eq!(Qty(0), get_level_capacity(&orderbook_manager, 1, 1));

        orderbook_manager.add_order(OrderId(3), BookId(1), Qty(50), 800, true);
        orderbook_manager.add_order(OrderId(4), BookId(1), Qty(26), 600, true);
        assert_eq!(Qty(50), get_level_capacity(&orderbook_manager, 1, 1));
        assert_eq!(Qty(26), get_level_capacity(&orderbook_manager, 1, 2));
        orderbook_manager.remove_order(OrderId(3));
        assert_eq!(Qty(0), get_level_capacity(&orderbook_manager, 1, 1));
        orderbook_manager.remove_order(OrderId(4));
        assert_eq!(Qty(0), get_level_capacity(&orderbook_manager, 1, 2));
        orderbook_manager.remove_order(OrderId(0));
        assert_eq!(Qty(0), get_level_capacity(&orderbook_manager, 1, 0));
        orderbook_manager.add_order(OrderId(5), BookId(1), Qty(50), 1500, true);
        orderbook_manager.add_order(OrderId(6), BookId(1), Qty(26), 500, true);
        assert_eq!(Qty(50), get_level_capacity(&orderbook_manager, 1, 0));
        assert_eq!(Qty(26), get_level_capacity(&orderbook_manager, 1, 2));
        assert_eq!(Qty(0), get_level_capacity(&orderbook_manager, 1, 1));
        orderbook_manager.add_order(OrderId(6), BookId(1), Qty(86), 1400, true);
        assert_eq!(Qty(86), get_level_capacity(&orderbook_manager, 1, 1));
        orderbook_manager.add_order(OrderId(6), BookId(1), Qty(96), 1300, true);
        assert_eq!(Qty(96), get_level_capacity(&orderbook_manager, 1, 3));
    }
}
