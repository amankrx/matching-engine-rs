#![allow(unused_imports)]
use optimized_lob::level::LevelId;
use optimized_lob::orderbook_manager::OrderBookManager;
use optimized_lob::quantity::Qty;

// A few helper functions for the tests

#[cfg(test)]
pub(crate) fn get_level_capacity(
    orderbook_manager: &OrderBookManager,
    book_id: usize,
    level_id: u32,
) -> Qty {
    orderbook_manager
        .books
        .get(book_id)
        .unwrap()
        .clone()
        .unwrap()
        .level_pool
        .get(LevelId(level_id))
        .unwrap()
        .size()
}
