// orderbook/order.rs
//  * Author: Aman Kumar <aman@amankrx.com>
//  * Created: Wed Jun 07 2023
//  * Last Modified: Wed Jun 07 2023
//  * Description: Orderbook Order
//  * License: Distributed under the terms of the MIT License

use crate::utils::errors::ErrorCode;
/// # OrderSide
/// ## Description
/// OrderSide is an enum that represents the side of an order.
/// ## Variants
/// * Bid - Buy side of the order represents the highest price that a buyer is willing to pay for a security.
/// * Ask - Sell side of the order represents the lowest price that a seller is willing to accept for a security.
#[derive(Debug)]
pub enum OrderSide {
    Bid,
    Ask,
}

/// # OrderType
/// ## Description
/// OrderType is an enum that represents the type of an order.
/// ## Variants
/// * Market - A market order is a buy or sell order to be executed immediately at the current market prices.
/// * Limit - A limit order is an order to buy or sell a stock at a specific price or better.
/// * Stop - A stop order, also referred to as a stop-loss order, is an order to buy or sell a stock once the price of the stock reaches a specified price, known as the stop price.
/// * StopLimit - A stop-limit order is an order to buy or sell a stock that combines the features of a stop order and a limit order.
/// * TrailingStop - A trailing stop is a stop order that can be set at a defined percentage or dollar amount away from a security's current market price.
/// * TrailingStopLimit - A trailing stop limit order is designed to allow an investor to specify a limit on the maximum possible loss, without setting a limit on the maximum possible gain.
#[derive(Debug)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

/// # Order
/// ## Description
/// Order is a struct that represents an order.
#[derive(Debug)]
pub struct Order {
    pub id: u64,
    pub symbol_id: u32,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub price: Option<u64>,
    pub stop_price: Option<u64>,
    pub quantity: u64,
    pub filled_quantity: u64,
    pub remaining_quantity: u64,
}

impl Order {
    pub fn new(
        id: u64,
        symbol_id: u32,
        side: OrderSide,
        order_type: OrderType,
        price: Option<u64>,
        stop_price: Option<u64>,
        quantity: u64,
        filled_quantity: u64,
        remaining_quantity: u64,
    ) -> Order {
        Order {
            id,
            symbol_id,
            side,
            order_type,
            price,
            stop_price,
            quantity,
            filled_quantity,
            remaining_quantity,
        }
    }

    pub fn market_order(id: u64, symbol_id: u32, side: OrderSide, quantity: u64) -> Order {
        Order {
            id,
            symbol_id,
            side,
            order_type: OrderType::Market,
            price: None,
            stop_price: None,
            quantity,
            filled_quantity: 0,
            remaining_quantity: quantity,
        }
    }

    pub fn limit_order(
        id: u64,
        symbol_id: u32,
        side: OrderSide,
        quantity: u64,
        price: u64,
    ) -> Order {
        Order {
            id,
            symbol_id,
            side,
            order_type: OrderType::Limit,
            price: Some(price),
            stop_price: None,
            quantity,
            filled_quantity: 0,
            remaining_quantity: quantity,
        }
    }

    pub fn stop_order(
        id: u64,
        symbol_id: u32,
        side: OrderSide,
        quantity: u64,
        stop_price: u64,
    ) -> Order {
        Order {
            id,
            symbol_id,
            side,
            order_type: OrderType::Stop,
            price: None,
            stop_price: Some(stop_price),
            quantity,
            filled_quantity: 0,
            remaining_quantity: quantity,
        }
    }

    pub fn stop_limit_order(
        id: u64,
        symbol_id: u32,
        side: OrderSide,
        quantity: u64,
        price: u64,
        stop_price: u64,
    ) -> Order {
        Order {
            id,
            symbol_id,
            side,
            order_type: OrderType::StopLimit,
            price: Some(price),
            stop_price: Some(stop_price),
            quantity,
            filled_quantity: 0,
            remaining_quantity: quantity,
        }
    }

    pub fn is_buy(&self) -> bool {
        matches!(self.side, OrderSide::Bid)
    }

    pub fn is_sell(&self) -> bool {
        matches!(self.side, OrderSide::Ask)
    }

    pub fn is_market(&self) -> bool {
        matches!(self.order_type, OrderType::Market)
    }

    pub fn is_limit(&self) -> bool {
        matches!(self.order_type, OrderType::Limit)
    }

    pub fn is_stop(&self) -> bool {
        matches!(self.order_type, OrderType::Stop)
    }

    pub fn is_stop_limit(&self) -> bool {
        matches!(self.order_type, OrderType::StopLimit)
    }

    pub fn is_filled(&self) -> bool {
        self.remaining_quantity == 0
    }

    pub fn is_partially_filled(&self) -> bool {
        self.remaining_quantity > 0 && self.filled_quantity > 0
    }

    pub fn is_active(&self) -> bool {
        self.remaining_quantity > 0
    }

    fn validate_market_order(&self) -> Result<(), ErrorCode> {
        if self.price.is_some() {
            return Err(ErrorCode::InvalidOrderParameters(
                "Market orders must not have a price",
            ));
        }

        if self.stop_price.is_some() {
            return Err(ErrorCode::InvalidOrderParameters(
                "Market orders must not have a stop price",
            ));
        }

        Ok(())
    }

    fn validate_limit_order(&self) -> Result<(), ErrorCode> {
        if self.price.is_none() {
            return Err(ErrorCode::InvalidOrderParameters(
                "Limit orders must have a price",
            ));
        }
        if self.stop_price.is_some() {
            return Err(ErrorCode::InvalidOrderParameters(
                "Limit orders must not have a stop price",
            ));
        }

        Ok(())
    }

    fn validate_stop_order(&self) -> Result<(), ErrorCode> {
        if self.price.is_some() {
            return Err(ErrorCode::InvalidOrderParameters(
                "Stop orders must not have a price",
            ));
        }
        if self.stop_price.is_none() {
            return Err(ErrorCode::InvalidOrderParameters(
                "Stop orders must have a stop price",
            ));
        }

        Ok(())
    }

    fn validate_stop_limit_order(&self) -> Result<(), ErrorCode> {
        if self.price.is_none() {
            return Err(ErrorCode::InvalidOrderParameters(
                "Stop Limit orders must have a price",
            ));
        }
        if self.stop_price.is_none() {
            return Err(ErrorCode::InvalidOrderParameters(
                "Stop Limit orders must have a stop price",
            ));
        }

        Ok(())
    }

    pub fn validate(&self) -> Result<(), ErrorCode> {
        // Validate Order ID
        if self.id == 0 {
            return Err(ErrorCode::InvalidOrderId("Order ID must be greater than 0"));
        }

        // Validate Order Quantity
        if self.quantity < self.remaining_quantity {
            return Err(ErrorCode::InvalidOrderQuantity(
                "Order quantity must be greater than or equal to remaining quantity",
            ));
        }
        if self.quantity == 0 {
            return Err(ErrorCode::InvalidOrderQuantity(
                "Order quantity must be greater than 0",
            ));
        }
        if self.remaining_quantity == 0 {
            return Err(ErrorCode::InvalidOrderQuantity(
                "Order remaining quantity must be greater than 0",
            ));
        }

        // Validate Market Order
        if self.is_market() {
            return self.validate_market_order();
        }

        // Validate Limit Order
        if self.is_limit() {
            return self.validate_limit_order();
        }

        // Validate Stop Order
        if self.is_stop() {
            return self.validate_stop_order();
        }

        // Validate Stop Limit Order
        if self.is_stop_limit() {
            return self.validate_stop_limit_order();
        }

        Ok(())
    }
}
