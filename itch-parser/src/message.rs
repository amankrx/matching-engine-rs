// message.rs

use super::body::{Body, parse_system_event, parse_add_order, parse_replace_order};
use super::utils::{be_u48, char_to_bool};
use nom::{
    bytes::streaming::take,
    combinator::map_res,
    number::streaming::{be_u16, be_u32, be_u64, be_u8},
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    /// Message Type
    pub tag: u8,
    /// Integer identifying the underlying instrument updated daily
    pub stock_locate: u16,
    /// NASDAQ internal tracking number
    pub tracking_number: u16,
    /// Nanoseconds since midnight
    pub timestamp: u64,
    /// Body of one of the supported message types
    pub body: Body,
}

#[inline]
pub fn parse_message(input: &[u8]) -> IResult<&[u8], Message> {
    let (input, _) = be_u16(input)?;
    let (input, tag) = be_u8(input)?;
    let (input, (stock_locate, tracking_number, timestamp, body)) = tuple((
        be_u16,
        be_u16,
        be_u48,
        |input| parse_body(input, tag),
    ))(input)?;

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

#[inline]
fn parse_body(input: &[u8], tag: u8) -> IResult<&[u8], Body> {
    match tag {
        b'A' => {
            let (input, order) = parse_add_order(input, false)?;
            Ok((input, Body::AddOrder(order)))
        }
        b'B' => {
            let (input, _) = take(8usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        b'C' => {
            let (input, (reference, executed, match_number, printable, price)) = tuple((
                be_u64,
                be_u32,
                be_u64,
                map_res(be_u8, char_to_bool),
                be_u32,
            ))(input)?;
            Ok((
                input,
                Body::OrderExecutedWithPrice {
                    reference,
                    executed,
                    match_number,
                    printable,
                    price: price.into(),
                },
            ))
        }
        b'D' => {
            let (input, reference) = be_u64(input)?;
            Ok((input, Body::DeleteOrder { reference }))
        }
        b'E' => {
            let (input, (reference, executed, match_number)) = tuple((be_u64, be_u32, be_u64))(input)?;
            Ok((input, Body::OrderExecuted { reference, executed, match_number }))
        }
        b'F' => {
            let (input, order) = parse_add_order(input, true)?;
            Ok((input, Body::AddOrder(order)))
        }
        b'H' => {
            let (input, _) = take(14usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        b'I' => {
            let (input, _) = take(39usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        b'J' => {
            let (input, _) = take(24usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        b'K' => {
            let (input, _) = take(17usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        b'L' => {
            let (input, _) = take(15usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        b'N' => {
            let (input, _) = take(9usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        b'P' => {
            let (input, _) = take(33usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        b'Q' => {
            let (input, _) = take(29usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        b'R' => {
            let (input, _) = take(28usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        b'S' => {
            let (input, event_code) = parse_system_event(input)?;
            Ok((input, Body::SystemEvent { event: event_code }))
        }
        b'U' => {
            let (input, order) = parse_replace_order(input)?;
            Ok((input, Body::ReplaceOrder(order)))
        }
        b'V' => {
            let (input, _) = take(24usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        b'W' => {
            let (input, _) = take(1usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        b'X' => {
            let (input, (reference, cancelled)) = tuple((be_u64, be_u32))(input)?;
            Ok((input, Body::OrderCancelled { reference, cancelled }))
        }
        b'Y' => {
            let (input, _) = take(9usize)(input)?;
            Ok((input, Body::Pass(())))
        }
        _ => Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag))),
    }
}