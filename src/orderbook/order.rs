// order.rs

use crate::orderbook::level::LevelId;
use crate::orderbook::quantity::Qty;
use crate::orderbook::utils::BookId;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u32);

impl Default for OrderId {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Default, Clone)]
pub struct Order {
    pub level_id: LevelId,
    pub book_id: BookId,
    pub qty: Qty,
}

impl Debug for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Order")
            .field("level_id", &self.level_id)
            .field("book_id", &self.book_id)
            .field("qty", &self.qty)
            .finish()
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.level_id == other.level_id && self.book_id == other.book_id && self.qty == other.qty
    }
}

impl AsRef<Order> for Order {
    fn as_ref(&self) -> &Order {
        self
    }
}

impl Order {
    pub fn new(qty: Qty, level_id: LevelId, book_id: BookId) -> Self {
        Self {
            level_id,
            book_id,
            qty,
        }
    }

    pub fn replace(&mut self, order: Order) {
        self.level_id = order.level_id;
        self.book_id = order.book_id;
        self.qty = order.qty;
    }

    pub fn set_level_id(&mut self, level_id: LevelId) {
        self.level_id = level_id;
    }
}

pub struct OidMap {
    data: Vec<Option<Order>>,
}

impl OidMap {
    pub fn new() -> Self {
        OidMap {
            data: vec![None; 1 << 20], // Use a fixed-size array
        }
    }

    pub fn reserve(&mut self, oid: OrderId) {
        let idx = oid.0 as usize;
        if idx >= self.data.len() {
            self.data.resize(idx + 1, None);
        }
    }

    pub fn insert(&mut self, oid: OrderId, value: &Order) {
        let idx = oid.0 as usize;
        if idx >= self.data.len() {
            self.data.resize(idx + 1, None);
        }
        self.data[idx] = Some(value.clone()); // Clone only when necessary
    }

    pub fn remove(&mut self, oid: OrderId) {
        let idx = oid.0 as usize;
        if idx < self.data.len() {
            self.data[idx] = None;
        }
    }

    pub fn update_qty(&mut self, oid: OrderId, qty: Qty) {
        let idx = oid.0 as usize;
        if idx < self.data.len() {
            if let Some(order) = &mut self.data[idx] {
                if order.qty == qty {
                    self.data[idx] = None;
                } else {
                    order.qty -= qty;
                }
            }
        }
    }

    pub fn get(&self, oid: OrderId) -> Option<&Order> {
        let idx = oid.0 as usize;
        self.data.get(idx)?.as_ref()
    }

    pub fn get_mut(&mut self, oid: OrderId) -> Option<&mut Order> {
        let idx = oid.0 as usize;
        self.data.get_mut(idx)?.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_creation() {
        let qty = Qty(100);
        let level_id = LevelId(1);
        let book_id = BookId(42);

        let order = Order::new(qty, level_id, book_id);

        assert_eq!(order.qty, qty);
        assert_eq!(order.level_id, level_id);
        assert_eq!(order.book_id, book_id);
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

        assert_eq!(order.qty, qty2);
        assert_eq!(order.level_id, level_id2);
        assert_eq!(order.book_id, book_id2);
    }

    #[test]
    fn test_order_set_level_id() {
        let qty = Qty(100);
        let level_id1 = LevelId(1);
        let level_id2 = LevelId(2);
        let book_id = BookId(42);

        let mut order = Order::new(qty, level_id1, book_id);

        order.set_level_id(level_id2);

        assert_eq!(order.level_id, level_id2);
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
