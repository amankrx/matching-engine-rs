// price.rs

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug, Default)]
pub struct Price(pub i32);

impl Price {
    /// Returns the value of the price.
    #[inline]
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Returns true if the price is a bid.
    #[inline]
    pub fn is_bid(&self) -> bool {
        self.0 > 0
    }

    /// Returns the absolute value of the price.
    #[inline]
    pub fn absolute(&self) -> i32 {
        self.0.abs()
    }

    /// Convert a u32 to a Price.
    #[inline]
    pub fn from_u32(price: u32, is_bid: bool) -> Self {
        Self(price as i32 * if is_bid { 1 } else { -1 })
    }
}
