// order.rs

use crate::{
    level::LevelId,
    quantity::Qty,
    utils::{BookId, INITIAL_ORDER_COUNT},
};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OrderId(pub u32);

#[derive(Default, Clone)]
pub struct Order {
    level_id: LevelId,
    book_id: BookId,
    qty: Qty,
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
    #[inline]
    pub fn new(qty: Qty, level_id: LevelId, book_id: BookId) -> Self {
        Self {
            level_id,
            book_id,
            qty,
        }
    }

    #[inline]
    pub fn replace(&mut self, order: Order) {
        self.level_id = order.level_id;
        self.book_id = order.book_id;
        self.qty = order.qty;
    }

    #[inline]
    pub fn qty(&self) -> Qty {
        self.qty
    }

    #[inline]
    pub fn book_id(&self) -> BookId {
        self.book_id
    }

    #[inline]
    pub fn level_id(&self) -> LevelId {
        self.level_id
    }

    #[inline]
    pub fn set_qty(&mut self, qty: Qty) {
        self.qty = qty;
    }

    #[inline]
    pub fn set_book_id(&mut self, book_id: BookId) {
        self.book_id = book_id;
    }

    #[inline]
    pub fn set_level_id(&mut self, level_id: LevelId) {
        self.level_id = level_id;
    }
}

pub struct OidMap {
    data: Vec<Option<Order>>,
}

impl Default for OidMap {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl OidMap {
    #[inline]
    pub fn new() -> Self {
        OidMap {
            data: vec![None; INITIAL_ORDER_COUNT], // Use a fixed-size array
        }
    }

    #[inline]
    pub fn reserve(&mut self, oid: OrderId) {
        let idx = oid.0 as usize;
        if idx >= self.data.len() {
            self.data.resize(idx + 1, None);
        }
    }

    #[inline]
    pub fn insert(&mut self, oid: OrderId, value: &Order) {
        let idx = oid.0 as usize;
        if idx >= self.data.len() {
            self.data.resize(idx + 1, None);
        }
        self.data[idx] = Some(value.clone()); // Clone only when necessary
    }

    #[inline]
    pub fn remove(&mut self, oid: OrderId) {
        let idx = oid.0 as usize;
        if idx < self.data.len() {
            self.data[idx] = None;
        }
    }

    #[inline]
    pub fn update_qty(&mut self, oid: OrderId, qty: Qty) {
        let idx = oid.0 as usize;
        if idx < self.data.len() {
            if let Some(order) = &mut self.data[idx] {
                order.qty -= qty;
            }
        }
    }

    #[inline]
    pub fn get(&self, oid: OrderId) -> Option<&Order> {
        let idx = oid.0 as usize;
        self.data.get(idx)?.as_ref()
    }

    #[inline]
    pub fn get_mut(&mut self, oid: OrderId) -> Option<&mut Order> {
        let idx = oid.0 as usize;
        self.data.get_mut(idx)?.as_mut()
    }
}
