//quantity.rs

use std::ops::{AddAssign, SubAssign};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct Qty(pub u32);

impl Default for Qty {
    fn default() -> Self {
        Self(0)
    }
}

impl AddAssign for Qty {
    fn add_assign(&mut self, other: Qty) {
        self.0 += other.0;
    }
}

impl SubAssign for Qty {
    fn sub_assign(&mut self, other: Qty) {
        self.0 -= other.0;
    }
}

impl Qty {
    pub fn add(&self, other: Qty) -> Qty {
        Qty(self.0 + other.0)
    }

    pub fn sub(&self, other: Qty) -> Qty {
        Qty(self.0 - other.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}
