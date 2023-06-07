// tests/order_test.rs
//  * Author: Aman Kumar <aman@amankrx.com>
//  * Created: Wed Jun 07 2023
//  * Last Modified: Wed Jun 07 2023
//  * Description: The test suite for Order
//  * License: Distributed under the terms of the MIT License

#[cfg(test)]
mod tests {
    use crate::orderbook::order::*;
    use crate::utils::errors::ErrorCode;

    #[test]
    fn test_new_order() {
        let order = Order::new(
            1,
            123,
            OrderSide::Bid,
            OrderType::Market,
            OrderTimeInForce::GTC,
            None,
            None,
            100,
            0,
            100,
            None,
            None,
            None,
            None,
        );

        assert_eq!(order.id, 1);
        assert_eq!(order.symbol_id, 123);
        assert!(order.is_buy());
        assert!(order.is_market());
        assert!(order.is_gtc());
        assert_eq!(order.price, None);
        assert_eq!(order.stop_price, None);
        assert_eq!(order.quantity, 100);
        assert_eq!(order.filled_quantity, 0);
        assert_eq!(order.remaining_quantity, 100);
        assert_eq!(order.max_show_quantity, None);
        assert_eq!(order.slippage, None);
        assert_eq!(order.trailing_distance, None);
        assert_eq!(order.trailing_step, None);
    }

    #[test]
    fn test_market_order() {
        let order = Order::market_order(1, 123, OrderSide::Bid, 100, OrderTimeInForce::GTC, None);

        assert_eq!(order.id, 1);
        assert_eq!(order.symbol_id, 123);
        assert!(order.is_buy());
        assert!(order.is_market());
        assert!(order.is_gtc());
        assert_eq!(order.price, None);
        assert_eq!(order.stop_price, None);
        assert_eq!(order.quantity, 100);
        assert_eq!(order.filled_quantity, 0);
        assert_eq!(order.remaining_quantity, 100);
        assert_eq!(order.max_show_quantity, None);
        assert_eq!(order.slippage, None);
        assert_eq!(order.trailing_distance, None);
        assert_eq!(order.trailing_step, None);
    }

    // TODO: Add more tests for the remaining order types and methods

    #[test]
    fn test_is_buy() {
        let order = Order::new(
            1,
            123,
            OrderSide::Bid,
            OrderType::Market,
            OrderTimeInForce::GTC,
            None,
            None,
            100,
            0,
            100,
            None,
            None,
            None,
            None,
        );

        assert!(order.is_buy());
        assert!(!order.is_sell());
    }

    // TODO: Add more tests for the remaining is_* methods

    #[test]
    fn test_validate_market_order() {
        let order = Order::new(
            1,
            123,
            OrderSide::Bid,
            OrderType::Market,
            OrderTimeInForce::GTC,
            None,
            None,
            100,
            0,
            100,
            None,
            None,
            None,
            None,
        );

        assert_eq!(
            order.validate(),
            Err(ErrorCode::InvalidOrderTimeInForce(
                "Market orders must be IOC or FOK"
            ))
        );
    }

    // TODO: Add more tests for the remaining validate_*_order methods
}
