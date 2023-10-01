// utils.rs

pub const INITIAL_ORDER_COUNT: usize = 1 << 20;
pub const MAX_BOOKS: usize = 1 << 14;
pub const MAX_LEVELS: usize = 1 << 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BookId(pub u16);

impl Default for BookId {
    fn default() -> Self {
        Self(0)
    }
}
