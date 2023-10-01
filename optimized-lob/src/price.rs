// price.rs

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
pub struct Price(pub i32);

impl Default for Price {
    fn default() -> Self {
        Self(0)
    }
}

impl Price {
    pub fn is_bid(&self) -> bool {
        self.0 > 0
    }

    pub fn get_absolute_value(&self) -> i32 {
        if self.is_bid() {
            self.0
        } else {
            -self.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_is_bid() {
        let positive_price = Price(100);
        let zero_price = Price(0);
        let negative_price = Price(-100);

        assert!(positive_price.is_bid());
        assert!(!zero_price.is_bid());
        assert!(!negative_price.is_bid());
    }
}
