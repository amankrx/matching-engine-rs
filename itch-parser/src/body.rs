// body.rs

use nom::IResult;

/// The message body. Refer to the protocol spec for interpretation.
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
    Pass(()),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventCode {
    StartOfMessages,
    StartOfSystemHours,
    StartOfMarketHours,
    EndOfMarketHours,
    EndOfSystemHours,
    EndOfMessages,
}

#[inline]
pub fn parse_system_event(input: &[u8]) -> IResult<&[u8], EventCode> {
    let (input, event_char) = nom::character::streaming::anychar(input)?;

    let event = match event_char {
        'O' => EventCode::StartOfMessages,
        'S' => EventCode::StartOfSystemHours,
        'Q' => EventCode::StartOfMarketHours,
        'M' => EventCode::EndOfMarketHours,
        'E' => EventCode::EndOfSystemHours,
        'C' => EventCode::EndOfMessages,
        _ => {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    };

    Ok((input, event))
}
