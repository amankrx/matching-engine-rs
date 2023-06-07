// orderbook/symbol.rs
//  * Author: Aman Kumar <aman@amankrx.com>
//  * Created: Wed Jun 07 2023
//  * Last Modified: Wed Jun 07 2023
//  * Description: A struct that represents a symbol.
//  * License: Distributed under the terms of the MIT License

/// # Symbol
/// ## Description
/// Symbol is a struct that represents a symbol.
/// ## Fields
/// * **id** - A unique identifier for the symbol.
/// * **name** - A name for the symbol.
pub struct Symbol {
    pub id: u32,
    pub name: [char; 8],
}

impl Symbol {
    /// # new
    /// ## Description
    /// Creates a new symbol.
    /// ## Parameters
    /// * **id** - A unique identifier for the symbol.
    /// * **name** - A name for the symbol.
    /// ## Returns
    /// * **Symbol** - A new symbol.
    pub fn new(id: u32, name: [char; 8]) -> Symbol {
        Symbol { id, name }
    }
}
