#[cfg(test)]
mod tests {
    use optimized_lob::level::LevelId;
    use optimized_lob::order::{OidMap, Order, OrderId};
    use optimized_lob::quantity::Qty;
    use optimized_lob::utils::BookId;

    #[test]
    fn test_order_creation() {
        let qty = Qty(100);
        let level_id = LevelId(1);
        let book_id = BookId(42);

        let order = Order::new(qty, level_id, book_id);

        assert_eq!(order.qty(), qty);
        assert_eq!(order.level_id(), level_id);
        assert_eq!(order.book_id(), book_id);
    }

    #[test]
    fn test_order_replace() {
        let qty1 = Qty(100);
        let qty2 = Qty(200);
        let level_id1 = LevelId(1);
        let level_id2 = LevelId(2);
        let book_id1 = BookId(42);
        let book_id2 = BookId(43);

        let mut order = Order::new(qty1, level_id1, book_id1);
        let new_order = Order::new(qty2, level_id2, book_id2);

        order.replace(new_order);

        assert_eq!(order.qty(), qty2);
        assert_eq!(order.level_id(), level_id2);
        assert_eq!(order.book_id(), book_id2);
    }

    #[test]
    fn test_order_set_level_id() {
        let qty = Qty(100);
        let level_id1 = LevelId(1);
        let level_id2 = LevelId(2);
        let book_id = BookId(42);

        let mut order = Order::new(qty, level_id1, book_id);

        order.set_level_id(level_id2);

        assert_eq!(order.level_id(), level_id2);
    }

    #[test]
    fn test_oid_map_insert_and_get() {
        let mut oid_map = OidMap::new();
        let oid = OrderId(0);
        let qty = Qty(100);
        let level_id = LevelId(1);
        let book_id = BookId(42);
        let order = Order::new(qty, level_id, book_id);

        oid_map.insert(oid, &order);
        let retrieved_order = oid_map.get(oid);

        assert_eq!(retrieved_order, Some(&order));
    }

    #[test]
    fn test_oid_map_reserve() {
        let mut oid_map = OidMap::new();
        let oid = OrderId(1000);
        let qty = Qty(100);
        let level_id = LevelId(1);
        let book_id = BookId(42);
        let order = Order::new(qty, level_id, book_id);

        oid_map.reserve(oid);
        oid_map.insert(oid, &order);
        let retrieved_order = oid_map.get(oid);

        assert_eq!(retrieved_order, Some(&order));
    }

    #[test]
    fn test_oid_map_remove() {
        let mut oid_map = OidMap::new();
        let oid = OrderId(0);
        let qty = Qty(100);
        let level_id = LevelId(1);
        let book_id = BookId(42);
        let order = Order::new(qty, level_id, book_id);

        oid_map.insert(oid, &order);
        oid_map.remove(oid);
        let retrieved_order = oid_map.get(oid);

        assert_eq!(retrieved_order, None);
    }
}
