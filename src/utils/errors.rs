// utils/errors.rs
//  * Author: Aman Kumar <aman@amankrx.com>
//  * Created: Wed Jun 07 2023
//  * Last Modified: Wed Jun 07 2023
//  * Description: The possible errors that can occur.
//  * License: Distributed under the terms of the MIT License

#[derive(Debug, Eq, PartialEq)]
pub enum ErrorCode {
    DuplicateSymbol(&'static str),
    DuplicateOrder(&'static str),
    DuplicateOrderBook(&'static str),
    SymbolNotFound(&'static str),
    OrderNotFound(&'static str),
    OrderBookNotFound(&'static str),
    InvalidOrderType(&'static str),
    InvalidOrderSide(&'static str),
    InvalidOrderPrice(&'static str),
    InvalidOrderQuantity(&'static str),
    InvalidOrderId(&'static str),
    InvalidOrderBookId(&'static str),
    InvalidSymbolId(&'static str),
    InvalidOrderParameters(&'static str),
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::DuplicateSymbol(msg) => write!(f, "Duplicate Symbol Error: {}", msg),
            ErrorCode::DuplicateOrder(msg) => write!(f, "Duplicate Order Error: {}", msg),
            ErrorCode::DuplicateOrderBook(msg) => write!(f, "Duplicate OrderBook Error: {}", msg),
            ErrorCode::SymbolNotFound(msg) => write!(f, "Symbol Not Found Error: {}", msg),
            ErrorCode::OrderNotFound(msg) => write!(f, "Order Not Found Error: {}", msg),
            ErrorCode::OrderBookNotFound(msg) => write!(f, "OrderBook Not Found Error: {}", msg),
            ErrorCode::InvalidOrderType(msg) => write!(f, "Invalid Order Type Error: {}", msg),
            ErrorCode::InvalidOrderSide(msg) => write!(f, "Invalid Order Side Error: {}", msg),
            ErrorCode::InvalidOrderPrice(msg) => write!(f, "Invalid Order Price Error: {}", msg),
            ErrorCode::InvalidOrderQuantity(msg) => {
                write!(f, "Invalid Order Quantity Error: {}", msg)
            }
            ErrorCode::InvalidOrderId(msg) => write!(f, "Invalid Order Id Error: {}", msg),
            ErrorCode::InvalidOrderBookId(msg) => write!(f, "Invalid OrderBook Id Error: {}", msg),
            ErrorCode::InvalidSymbolId(msg) => write!(f, "Invalid Symbol Id Error: {}", msg),
            ErrorCode::InvalidOrderParameters(msg) => {
                write!(f, "Invalid Order Parameters Error: {}", msg)
            }
        }
    }
}
