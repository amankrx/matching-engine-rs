// utils.rs

use arrayvec::ArrayString;
use nom::{IResult, Needed};
use super::errors::*;

/// Stack-allocated string of size 4 bytes
pub type ArrayString4 = ArrayString<4>;

/// Stack-allocated string of size 8 bytes
pub type ArrayString8 = ArrayString<8>;

#[inline]
pub fn char_to_bool(input: u8) -> Result<bool> {
    if input == b'Y' {
        Ok(true)
    } else if input == b'N' {
        Ok(false)
    } else {
        Err(Error::from("Invalid input"))
    }
}

#[inline]
pub fn be_u48(i: &[u8]) -> IResult<&[u8], u64> {
    if i.len() < 6 {
        Err(nom::Err::Incomplete(Needed::Size(std::num::NonZeroUsize::new(6).unwrap())))
    } else {
        let res = ((i[0] as u64) << 40)
            + ((i[1] as u64) << 32)
            + ((i[2] as u64) << 24)
            + ((i[3] as u64) << 16)
            + ((i[4] as u64) << 8)
            + i[5] as u64;
        Ok((&i[6..], res))
    }
}

