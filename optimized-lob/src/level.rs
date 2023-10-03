// level.rs

use crate::{price::Price, quantity::Qty};
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LevelId(pub u32);

impl LevelId {
    #[inline]
    pub fn value(&self) -> u32 {
        self.0
    }
}

#[derive(Debug)]
pub struct Level {
    price: Price,
    size: Qty,
}

impl Default for Level {
    #[inline]
    fn default() -> Self {
        Self {
            price: Price(0),
            size: Qty(0),
        }
    }
}

impl Level {
    #[inline]
    pub fn new(price: Price, size: Qty) -> Self {
        Self { price, size }
    }

    #[inline]
    pub fn price(&self) -> Price {
        self.price
    }

    #[inline]
    pub fn size(&self) -> Qty {
        self.size
    }

    #[inline]
    pub fn set_price(&mut self, price: Price) {
        self.price = price
    }

    #[inline]
    pub fn set_size(&mut self, size: Qty) {
        self.size = size
    }

    #[inline]
    pub fn incr(&mut self, size: Qty) {
        self.size += size
    }

    #[inline]
    pub fn decr(&mut self, size: Qty) {
        self.size -= size
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct PriceLevel {
    price: Price,
    level_idx: LevelId,
}

impl Default for PriceLevel {
    #[inline]
    fn default() -> Self {
        Self {
            price: Price(0),
            level_idx: LevelId(0),
        }
    }
}

impl Debug for PriceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PriceLevel")
            .field("price", &self.price)
            .field("level_idx", &self.level_idx)
            .finish()
    }
}

impl PriceLevel {
    #[inline]
    pub(crate) fn new(price: Price, level_idx: LevelId) -> Self {
        Self { price, level_idx }
    }

    #[inline]
    pub fn price(&self) -> Price {
        self.price
    }

    #[inline]
    pub fn level_id(&self) -> LevelId {
        self.level_idx
    }
}

impl Ord for PriceLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.value().cmp(&other.price.value())
    }
}

impl PartialOrd for PriceLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
pub struct SortedLevels(Vec<PriceLevel>);

impl SortedLevels {
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn get_mut(&mut self, idx: usize) -> &mut PriceLevel {
        &mut self.0[idx]
    }

    #[inline]
    pub fn insert(&mut self, idx: usize, px: PriceLevel) {
        self.0.insert(idx, px);
    }

    #[inline]
    pub fn remove(&mut self, price: Price) {
        for px in self.0.iter_mut().rev() {
            if px.price == price {
                self.0.retain(|x| x.price != price);
                break;
            }
        }
    }
}
