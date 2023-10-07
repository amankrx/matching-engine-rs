// orderbook_manager.rs

use crate::{
    level::LevelId,
    order::{OidMap, Order, OrderId},
    orderbook::OrderBook,
    price::Price,
    quantity::Qty,
    utils::{BookId, MAX_BOOKS},
};

/// Manages multiple order books and orders.
pub struct OrderBookManager {
    pub books: Vec<Option<OrderBook>>, // A mapping of book IDs to order books.
    pub oid_map: OidMap,               // A mapping of order IDs to order objects.
}

impl Default for OrderBookManager {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderBookManager {
    /// Creates a new OrderBookManager with empty books and an OidMap.
    #[inline]
    pub fn new() -> Self {
        Self {
            books: vec![None; MAX_BOOKS],
            oid_map: OidMap::new(),
        }
    }

    /// Adds a new order to the order book based on the provided parameters.
    /// ## Arguments:
    /// - `order_id`: The order ID for the order. Represented as unique reference number.
    /// - `book_id`: The identifier for the book where the order will be placed. Represents as stock locate.
    /// - `qty`: The quantity of the order. Represented as shares in the orderbook.
    /// - `price32`: The price of the order as a 32-bit unsigned integer. Return the Price(4) in the orderbook.
    /// - `is_bid`: A flag indicating whether the order is a bid (true) or ask (false). Return the Buy/Sell Indicator as boolean.
    ///
    /// ## Example:
    /// ```
    /// let mut orderbook_manager = OrderBookManager::new();
    ///
    /// orderbook_manager.add_order(
    ///     OrderId(0), // Order ID
    ///     BookId(0), // Book ID
    ///     Qty(100), // Quantity
    ///     600, // Price
    ///     true, // Is Bid
    /// );
    /// ```
    #[inline]
    pub fn add_order(
        &mut self,
        order_id: OrderId,
        book_id: BookId,
        qty: Qty,
        price32: u32,
        is_bid: bool,
    ) {
        let price_i32 = if is_bid {
            price32 as i32
        } else {
            -(price32 as i32)
        };

        // Create a Price(i32) from the adjusted price_i32.
        let price = Price(price_i32);

        self.oid_map.reserve(order_id);

        let mut order = Order::new(qty, LevelId(0), book_id);

        // Check if the book for the given book_id exists; if not, create it.
        if self.books[book_id.value() as usize].is_none() {
            self.books[book_id.value() as usize] = Some(OrderBook::new());
        }
        if let Some(orderbook) = self.books.get_mut(book_id.value() as usize).unwrap() {
            orderbook.add_order(&mut order, price, qty);
        }
        self.oid_map.insert(order_id, &order);
    }

    /// Removes an order from the order book based on its order ID.
    /// ## Arguments:
    /// - `order_id`: The order ID for the order. Represented as unique reference number.
    /// ## Example:
    /// ```
    /// let mut orderbook_manager = OrderBookManager::new();
    ///
    /// orderbook.remove_order(OrderId(0));
    /// ```
    #[inline]
    pub fn remove_order(&mut self, order_id: OrderId) {
        if let Some(order) = self.oid_map.get_mut(order_id) {
            if let Some(orderbook) = self
                .books
                .get_mut(order.book_id().value() as usize)
                .unwrap()
            {
                orderbook.remove_order(order);
            }
        }
        self.oid_map.remove(order_id);
    }

    /// Cancels an order by reducing its quantity in the order book.
    /// ## Arguments:
    /// - `order_id`: The order ID for the order. Represented as unique reference number.
    /// - `qty`: The quantity of the order to be cancelled. Represented as shares in the orderbook.
    /// ## Example:
    /// ```
    /// let mut orderbook_manager = OrderBookManager::new();
    ///
    /// orderbook.cancel_order(OrderId(0), Qty(100));
    /// ```
    #[inline]
    pub fn cancel_order(&mut self, order_id: OrderId, qty: Qty) {
        if let Some(order) = self.oid_map.get_mut(order_id) {
            if let Some(orderbook) = self
                .books
                .get_mut(order.book_id().value() as usize)
                .unwrap()
            {
                orderbook.reduce_order(order, qty);
            }
        }
        self.oid_map.update_qty(order_id, qty);
    }

    /// Executes an order by either removing it completely or reducing its quantity.
    /// ## Arguments:
    /// - `order_id`: The order ID for the order. Represented as unique reference number.
    /// - `qty`: The quantity of the order to be executed. Represented as shares in the orderbook.
    /// ## Example:
    /// ```
    /// let mut orderbook_manager = OrderBookManager::new();
    ///
    /// orderbook.execute_order(OrderId(0), Qty(100));
    /// ```
    #[inline]
    pub fn execute_order(&mut self, order_id: OrderId, qty: Qty) {
        if let Some(order) = self.oid_map.get_mut(order_id) {
            if order.qty() == qty {
                if let Some(orderbook) = self
                    .books
                    .get_mut(order.book_id().value() as usize)
                    .unwrap()
                {
                    orderbook.remove_order(order);
                }
                self.oid_map.remove(order_id);
            } else {
                if let Some(orderbook) = self
                    .books
                    .get_mut(order.book_id().value() as usize)
                    .unwrap()
                {
                    orderbook.reduce_order(order, qty);
                }
                self.oid_map.update_qty(order_id, qty);
            }
        }
    }

    /// Replaces an existing order with a new order based on order IDs and new parameters.
    /// ## Arguments:
    /// - `order_id`: The order ID for the order to be replaced. Represented as Original unique reference number.
    /// - `new_order_id`: The new order ID for the order that has to be replaced. Represented as the new unique reference number.
    /// - `new_qty`: The quantity of the new order. Represented as shares in the orderbook.
    /// - `new_price`: The price of the new order as a 32-bit unsigned integer. Return the Price(4) in the orderbook.
    ///
    /// ## Example:
    /// ```
    /// let mut orderbook_manager = OrderBookManager::new();
    ///
    /// orderbook_manager.replace_order(
    ///     OrderId(0), // Old Order ID
    ///     OrderId(0), // New Order ID
    ///     Qty(200), // Quantity
    ///     500, // Price
    /// );
    /// ```
    #[inline]
    pub fn replace_order(
        &mut self,
        order_id: OrderId,
        new_order_id: OrderId,
        new_qty: Qty,
        new_price: u32,
    ) {
        let order = self.oid_map.get_mut(order_id);
        let mut is_bid = true;
        let mut book_id = BookId(0);
        if let Some(order) = order {
            if let Some(book) = self
                .books
                .get_mut(order.book_id().value() as usize)
                .unwrap()
            {
                is_bid = book
                    .level_pool
                    .get(order.level_id())
                    .unwrap()
                    .price()
                    .is_bid();
                book_id = order.book_id();
                book.remove_order(order);
            }
            self.oid_map.remove(order_id);
        }
        self.add_order(new_order_id, book_id, new_qty, new_price, is_bid);
    }
}
