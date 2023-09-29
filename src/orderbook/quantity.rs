//quantity.rs

use std::ops::{AddAssign, SubAssign};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct Qty(pub i32);

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

mod tests {
    use super::Qty;

    #[test]
    fn test_qty_add_sub_is_zero() {
        let qty1 = Qty(10);
        let qty2 = Qty(5);
        let qty3 = Qty(0);

        assert_eq!(qty1.add(qty2.clone()), Qty(15));
        assert_eq!(qty1.sub(qty2), Qty(5));
        assert!(qty3.is_empty());
        assert!(!qty1.is_empty());
    }
}
