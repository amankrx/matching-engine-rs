// body.rs

use arrayvec::ArrayString;
use nom::{
    bytes::streaming::take,
    character::streaming::char,
    combinator::{cond, map_res},
    number::streaming::{be_u32, be_u64},
    sequence::tuple,
    IResult,
    Parser
};
use std::str::FromStr;
use super::price::Price4;
use super::utils::{ArrayString4, ArrayString8};

/// The message body. Refer to the protocol spec for interpretation.
#[derive(Debug, Clone, PartialEq)]
pub enum Body {
    AddOrder(AddOrder),
    DeleteOrder {
        reference: u64,
    },
    OrderCancelled {
        reference: u64,
        cancelled: u32,
    },
    OrderExecuted {
        reference: u64,
        executed: u32,
        match_number: u64,
    },
    OrderExecutedWithPrice {
        reference: u64,
        executed: u32,
        match_number: u64,
        printable: bool,
        price: Price4,
    },
    ReplaceOrder(ReplaceOrder),
    SystemEvent {
        event: EventCode,
    },
    Pass(()),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddOrder {
    pub reference: u64,
    pub side: Side,
    pub shares: u32,
    pub stock: ArrayString8,
    pub price: Price4,
    pub m_pid: Option<ArrayString4>,
}

#[inline]
pub fn parse_add_order(input: &[u8], attribution: bool) -> IResult<&[u8], AddOrder> {
    let mut parser = tuple((
        be_u64,
        char('B').map(|_| Side::Buy).or(char('S').map(|_| Side::Sell)),
        be_u32,
        map_res(take(8usize), |bytes| {
            let s = std::str::from_utf8(bytes).unwrap_or("");
            ArrayString::from_str(s)
        }),
        be_u32,
        // Conditionally parse m_pid if attribution is true
        cond(attribution, map_res(take(4usize), |bytes| {
            let s = std::str::from_utf8(bytes).unwrap_or("");
            ArrayString::from_str(s)
        })),
    ));
    let (input, (reference, side, shares, stock, price, m_pid)) = parser(input)?;

    // If attribution is true and m_pid is None, return an error
    if attribution && m_pid.is_none() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let price = Price4::from(price);

    Ok((input, AddOrder { reference, side, shares, stock, price, m_pid }))
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
        _ => return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag))),
    };

    Ok((input, event))
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReplaceOrder {
    pub old_reference: u64,
    pub new_reference: u64,
    pub shares: u32,
    pub price: Price4,
}

#[inline]
pub fn parse_replace_order(input: &[u8]) -> IResult<&[u8], ReplaceOrder> {
    let mut parser = tuple((
        be_u64,
        be_u64,
        be_u32,
        be_u32,
    ));
    let (input, (old_reference, new_reference, shares, price)) = parser(input)?;
    Ok((input, ReplaceOrder {
        old_reference,
        new_reference,
        shares,
        price: price.into()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn hex_to_bytes(bytes: &[u8]) -> Vec<u8> {
        fn h2b(h: u8) -> Option<u8> {
            match h {
                v @ b'0'..=b'9' => Some(v - b'0'),
                v @ b'a'..=b'f' => Some(v - b'a' + 10),
                b' ' | b'\n' => None,
                _ => panic!("Invalid hex: {}", h as char),
            }
        }
        bytes
            .iter()
            .filter_map(|b| h2b(*b))
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|slice| (slice[0] << 4) + slice[1])
            .collect()
    }

    #[test]
    fn test_parse_add_order_incomplete() {
        // Input with less than number of bytes, which is not enough for parsing AddOrder
        let code = b"00 00 00 00 00 00 05 84 42 00 00 00 64 5a 58 5a 5a 54 20 20 20 00 00 27";
        let bytes = hex_to_bytes(&code[..]);
        let result = parse_add_order(&bytes[..], false);

        // Expecting an Err::Incomplete result
        assert!(matches!(result, Err(nom::Err::Incomplete(_))));
    }

    #[test]
    fn test_parse_add_order_complete() {
        // Input with enough bytes for parsing AddOrder
        let code = b"00 00 00 00 00 00 05 84 42 00 00 00 64 5a 58 5a 5a 54 20 20 20 00 00 27 10";
        let bytes = hex_to_bytes(&code[..]);
        let result = parse_add_order(&bytes[..], false);

        // Expecting an Ok result indicating successful parsing
        assert!(result.is_ok());

        let (remaining, _) = result.unwrap();
        assert_eq!(remaining.len(), 0);
    }
}
