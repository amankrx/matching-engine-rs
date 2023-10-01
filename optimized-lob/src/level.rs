// level.rs

use crate::{price::Price, quantity::Qty};
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LevelId(pub u32);

impl Default for LevelId {
    fn default() -> Self {
        Self(0)
    }
}

pub struct Level {
    pub price: Price,
    pub size: Qty,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            price: Price(0),
            size: Qty(0),
        }
    }
}

impl Level {
    pub fn new(price: Price, size: Qty) -> Self {
        Self { price, size }
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct PriceLevel {
    pub price: Price,
    pub level_idx: LevelId,
}

impl Default for PriceLevel {
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
    pub(crate) fn new(price: Price, level_idx: LevelId) -> Self {
        Self { price, level_idx }
    }
}

impl Ord for PriceLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.0.cmp(&other.price.0)
    }
}

impl PartialOrd for PriceLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
pub struct SortedLevels(pub Vec<PriceLevel>);

impl SortedLevels {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn insert(&mut self, idx: usize, px: PriceLevel) {
        self.0.insert(idx, px);
    }

    pub fn remove(&mut self, price: Price) {
        self.0.retain(|px| px.price != price);
    }
}
