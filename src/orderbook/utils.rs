// utils.rs

pub const MAX_ORDER_IDS: usize = 100_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ptr(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BookId(pub u32);

impl Default for BookId {
    fn default() -> Self {
        Self(0)
    }
}
