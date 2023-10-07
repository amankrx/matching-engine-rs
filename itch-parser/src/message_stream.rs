// message_stream.rs

use super::{
    errors::*,
    message::{parse_message, Message},
};
use std::{fs::File, io::Read, path::Path};

const BUF_SIZE: usize = 64 * 1024;

/// Represents an iterable stream of ITCH protocol messages.
pub struct MessageStream<R> {
    reader: R,
    buffer: Box<[u8; BUF_SIZE]>,
    buf_start: usize,
    buf_end: usize,
    bytes_read: usize,
    read_calls: u32,
    message_ct: u32, // Total messages read so far
    in_error_state: bool,
}

impl MessageStream<File> {
    /// Creates a new `MessageStream` from a file at the specified path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<MessageStream<File>> {
        let reader = File::open(path)?;
        Ok(MessageStream::from_reader(reader))
    }
}

impl<R: Read> MessageStream<R> {
    /// Creates a new `MessageStream` from any type that implements the `Read` trait.
    #[inline]
    pub fn from_reader(reader: R) -> MessageStream<R> {
        MessageStream::new(reader)
    }

    /// Initializes a new `MessageStream` with default values.
    #[inline]
    fn new(reader: R) -> MessageStream<R> {
        MessageStream {
            reader,
            buffer: Box::new([0; BUF_SIZE]),
            buf_start: 0,
            buf_end: 0,
            bytes_read: 0,
            read_calls: 0,
            message_ct: 0,
            in_error_state: false,
        }
    }

    /// Fetches more bytes from the reader and updates the buffer.
    #[inline]
    fn fetch_more_bytes(&mut self) -> Result<usize> {
        self.read_calls += 1;
        if self.buf_end == BUF_SIZE {
            // Safety Checks
            assert!(self.buf_start > BUF_SIZE / 2);
            assert!(BUF_SIZE - self.buf_start < 100);

            {
                let (left, right) = self.buffer.split_at_mut(self.buf_start);
                left[..right.len()].copy_from_slice(right);
                self.buf_start = 0;
                self.buf_end = right.len();
            }
        }
        Ok(self.reader.read(&mut self.buffer[self.buf_end..])?)
    }
}

impl<R: Read> Iterator for MessageStream<R> {
    type Item = Result<Message>;

    #[inline]
    fn next(&mut self) -> Option<Result<Message>> {
        {
            let buf = &self.buffer[self.buf_start..self.buf_end];
            match parse_message(buf) {
                Ok((rest, msg)) => {
                    self.buf_start = self.buf_end - rest.len();
                    self.message_ct += 1;
                    self.in_error_state = false;
                    return Some(Ok(msg));
                }
                Err(nom::Err::Error(_e)) | Err(nom::Err::Failure(_e)) => {
                    return if self.in_error_state {
                        None
                    } else {
                        self.in_error_state = true;
                        Some(Err(format!(
                            "Parse failed: {:?}, buffer context",
                            &self.buffer[self.buf_start..self.buf_start + 20]
                        )
                        .into()))
                    }
                }
                Err(nom::Err::Incomplete(_)) => {
                    // Fall through to below... necessary to appease borrow checker.
                }
            }
        }
        match self.fetch_more_bytes() {
            Ok(0) => {
                // If we get EOF, return None
                if self.buf_start == self.buf_end {
                    return None;
                }
                if self.in_error_state {
                    None
                } else {
                    self.in_error_state = true;
                    Some(Err("Unexpected EOF".into()))
                }
            }
            Ok(ct) => {
                self.buf_end += ct;
                self.bytes_read += ct;
                self.next()
            }
            Err(e) => {
                if self.in_error_state {
                    None
                } else {
                    self.in_error_state = true;
                    Some(Err(e))
                }
            }
        }
    }
}
