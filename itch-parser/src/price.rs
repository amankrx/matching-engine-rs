// price.rs

/// Representing a price to four decimal places
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Price4(u32);

impl Price4 {
    pub fn raw(self) -> u32 {
        self.0
    }
}

impl From<u32> for Price4 {
    #[inline]
    fn from(v: u32) -> Price4 {
        Price4(v)
    }
}

/// Representing a price to eight decimal places
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Price8(u64);

impl Price8 {
    pub fn raw(self) -> u64 {
        self.0
    }
}

impl From<u64> for Price8 {
    fn from(v: u64) -> Price8 {
        Price8(v)
    }
}