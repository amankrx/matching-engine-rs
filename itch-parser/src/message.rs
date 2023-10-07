// message.rs

use super::body::{parse_system_event, Body};
use super::utils::{be_u48, char_to_bool};
use nom::{
    bytes::streaming::take,
    character::streaming::char,
    combinator::map_res,
    number::streaming::{be_u16, be_u32, be_u64, be_u8},
    sequence::tuple,
    IResult, Parser,
};

/// The Message struct. Contains the parsed values of a message.
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    /// Message Type (tag)
    pub tag: u8,
    /// Integer identifying the underlying instrument updated daily (stock_locate)
    pub stock_locate: u16,
    /// NASDAQ internal tracking number (tracking_number)
    pub tracking_number: u16,
    /// Nanoseconds since midnight (timestamp)
    pub timestamp: u64,
    /// Body of one of the supported message types
    pub body: Body,
}

/// Parses a complete message from input bytes.
///
/// # Arguments
/// `input` - Input bytes
///
/// # Returns
/// Returns a `Result` containing the parsed `Message` or an error.
#[inline]
pub fn parse_message(input: &[u8]) -> IResult<&[u8], Message> {
    // Parse the first 16 bits as an unsigned 16-bit integer and discard it.
    let (input, _) = be_u16(input)?;

    // Parse the next 8 bits as an unsigned 8-bit integer, representing the message tag.
    let (input, tag) = be_u8(input)?;

    // Parse the next 96 bits as a tuple containing stock_locate, tracking_number, timestamp,
    // and the message body.
    let (input, (stock_locate, tracking_number, timestamp, body)) =
        tuple((be_u16, be_u16, be_u48, |input| parse_body(input, tag)))(input)?;

    // Create and return a Message struct with the parsed values.
    Ok((
        input,
        Message {
            tag,
            stock_locate,
            tracking_number,
            timestamp,
            body,
        },
    ))
}

/// Function to parse the body of a message based on its tag.
#[inline]
fn parse_body(input: &[u8], tag: u8) -> IResult<&[u8], Body> {
    match tag {
        // Handles the `Add Order` message.
        b'A' => {
            let (input, (order_id, is_bid, shares, stock, price)) = tuple((
                be_u64,
                char('B').map(|_| true).or(char('S').map(|_| false)),
                be_u32,
                be_u64,
                be_u32,
            ))(input)?;
            Ok((
                input,
                Body::AddOrder {
                    order_id,
                    is_bid,
                    shares,
                    stock,
                    price,
                },
            ))
        }
        // Handles the `Broken Trade` message.
        b'B' => {
            let (input, _) = take(8usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        // Handles the `Order Executed with Price` message.
        b'C' => {
            let (input, (order_id, shares, match_number, printable, price)) =
                tuple((be_u64, be_u32, be_u64, map_res(be_u8, char_to_bool), be_u32))(input)?;
            Ok((
                input,
                Body::OrderExecutedWithPrice {
                    order_id,
                    shares,
                    match_number,
                    printable,
                    price,
                },
            ))
        }
        // Handles the `Order Delete` message.
        b'D' => {
            let (input, order_id) = be_u64(input)?;
            Ok((input, Body::DeleteOrder { order_id }))
        }
        // Handles the `Order Executed` message.
        b'E' => {
            let (input, (order_id, shares, match_number)) = tuple((be_u64, be_u32, be_u64))(input)?;
            Ok((
                input,
                Body::OrderExecuted {
                    order_id,
                    shares,
                    match_number,
                },
            ))
        }
        // Handles the `Add Order with MPID Attribution` message.
        b'F' => {
            let (input, (order_id, is_bid, shares, stock, price, _m_pid)) = tuple((
                be_u64,
                char('B').map(|_| true).or(char('S').map(|_| false)),
                be_u32,
                be_u64,
                be_u32,
                be_u32,
            ))(input)?;
            Ok((
                input,
                Body::AddOrder {
                    order_id,
                    is_bid,
                    shares,
                    stock,
                    price,
                },
            ))
        }
        // Handles the `Stock Trading Action` message.
        b'H' => {
            let (input, _) = take(14usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        // Handles the `Net Order Imbalance Indicator` message.
        b'I' => {
            let (input, _) = take(39usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        // Handles the `LULD Auction Collar` message.
        b'J' => {
            let (input, _) = take(24usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        // Handles the `Quoting Period Update` message.
        b'K' => {
            let (input, _) = take(17usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        // Handles the `Market Participant Position` message.
        b'L' => {
            let (input, _) = take(15usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        // Handles the `Retail Price Improvement Indicator` message.
        b'N' => {
            let (input, _) = take(9usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        // Handles the `Non-Cross Trade` message.
        b'P' => {
            let (input, _) = take(33usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        // Handles the `Cross Trade` message.
        b'Q' => {
            let (input, _) = take(29usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        // Handles the `Stock Directory` message.
        b'R' => {
            let (input, _) = take(28usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        // Handles the `System Event` message.
        b'S' => {
            let (input, event_code) = parse_system_event(input)?;
            Ok((input, Body::SystemEvent { event: event_code }))
        }
        // Handles the `Order Replace` message.
        b'U' => {
            let (input, (old_order_id, new_order_id, shares, price)) =
                tuple((be_u64, be_u64, be_u32, be_u32))(input)?;
            Ok((
                input,
                Body::ReplaceOrder {
                    old_order_id,
                    new_order_id,
                    shares,
                    price,
                },
            ))
        }
        // Handles the `MWCB Decline Level` message.
        b'V' => {
            let (input, _) = take(24usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        // Handles the `MWCB Status` message.
        b'W' => {
            let (input, _) = take(1usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        // Handles the `Order Cancel` message.
        b'X' => {
            let (input, (order_id, shares)) = tuple((be_u64, be_u32))(input)?;
            Ok((input, Body::OrderCancelled { order_id, shares }))
        }
        // Handles the `Reg SHO Short Sale Price Test Restricted Indicator` message.
        b'Y' => {
            let (input, _) = take(9usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        // Return an error if the tag doesn't match any known message type.
        _ => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
}
