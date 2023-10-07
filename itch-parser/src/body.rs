// body.rs

use nom::IResult;

/// The message body. It just uses the important variants and their fields.
#[derive(Debug, Clone, PartialEq)]
pub enum Body {
    AddOrder {
        order_id: u64,
        is_bid: bool,
        shares: u32,
        stock: u64,
        price: u32,
    },
    DeleteOrder {
        order_id: u64,
    },
    OrderCancelled {
        order_id: u64,
        shares: u32,
    },
    OrderExecuted {
        order_id: u64,
        shares: u32,
        match_number: u64,
    },
    OrderExecutedWithPrice {
        order_id: u64,
        shares: u32,
        match_number: u64,
        printable: bool,
        price: u32,
    },
    ReplaceOrder {
        old_order_id: u64,
        new_order_id: u64,
        shares: u32,
        price: u32,
    },
    SystemEvent {
        event: EventCode,
    },
    // Enum variant representing a placeholder "Pass" message with no data.
    Pass(()),
}

// Enum representing different event codes for SystemEvent messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventCode {
    StartOfMessages,
    StartOfSystemHours,
    StartOfMarketHours,
    EndOfMarketHours,
    EndOfSystemHours,
    EndOfMessages,
}

// Parse a SystemEvent message from input bytes.
#[inline]
pub fn parse_system_event(input: &[u8]) -> IResult<&[u8], EventCode> {
    let (input, event_char) = nom::character::streaming::anychar(input)?;

    // Match the parsed character to an EventCode variant or return an error if it doesn't match.
    let event = match event_char {
        'O' => EventCode::StartOfMessages,
        'S' => EventCode::StartOfSystemHours,
        'Q' => EventCode::StartOfMarketHours,
        'M' => EventCode::EndOfMarketHours,
        'E' => EventCode::EndOfSystemHours,
        'C' => EventCode::EndOfMessages,
        _ => {
            // If the character doesn't match any known EventCode, return a parsing error.
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
    };

    Ok((input, event))
}
