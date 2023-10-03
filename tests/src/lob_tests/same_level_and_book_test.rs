#![cfg(test)]
mod test {
    use crate::lob_tests::utils::get_level_capacity;
    /// Tests for the same level and book
    /// It tests all the use cases for the same level and book
    /// like add, cancel, remove, and execute orders
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

        assert_eq!(Qty(876), get_level_capacity(&orderbook_manager, 1, 0));

        orderbook_manager.remove_order(OrderId(2));
        assert_eq!(Qty(850), get_level_capacity(&orderbook_manager, 1, 0));

        orderbook_manager.cancel_order(OrderId(0), Qty(100));
        assert_eq!(Qty(750), get_level_capacity(&orderbook_manager, 1, 0));

        orderbook_manager.cancel_order(OrderId(1), Qty(50));
        assert_eq!(Qty(700), get_level_capacity(&orderbook_manager, 1, 0));

        orderbook_manager.add_order(OrderId(3), BookId(1), Qty(50), 500, true);
        orderbook_manager.add_order(OrderId(4), BookId(1), Qty(26), 500, true);
        assert_eq!(Qty(776), get_level_capacity(&orderbook_manager, 1, 0));

        orderbook_manager.remove_order(OrderId(3));
        assert_eq!(Qty(726), get_level_capacity(&orderbook_manager, 1, 0));

        orderbook_manager.remove_order(OrderId(4));
        assert_eq!(Qty(700), get_level_capacity(&orderbook_manager, 1, 0));

        orderbook_manager.remove_order(OrderId(0));
        assert_eq!(Qty(0), get_level_capacity(&orderbook_manager, 1, 0));

        orderbook_manager.add_order(OrderId(5), BookId(1), Qty(50), 500, true);
        orderbook_manager.add_order(OrderId(6), BookId(1), Qty(26), 500, true);
        assert_eq!(Qty(76), get_level_capacity(&orderbook_manager, 1, 0));
        orderbook_manager.execute_order(OrderId(5), Qty(50));
        assert_eq!(Qty(26), get_level_capacity(&orderbook_manager, 1, 0));
        orderbook_manager.execute_order(OrderId(6), Qty(10));
        assert_eq!(Qty(16), get_level_capacity(&orderbook_manager, 1, 0));
    }
}
