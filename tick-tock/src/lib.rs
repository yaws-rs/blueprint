#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

use blueprint::BluePrint;
use blueprint::Orbit;
use blueprint::{Left, Right, InBuffer};

/// Position in Tick Tocking
pub struct Position;

/// Error in Tick Tocking
#[derive(Debug)]
pub enum Error {
    /// Left Input exhausted with the current position
    ExhaustedLeftInput,
    /// Invalid Input as position
    InvalidInput,
}

/// boiler
pub struct TickTock;
/// boiler
#[derive(Default)]
pub struct TickTocking {
    initiated: bool,
}

impl Orbit for TickTocking {
    type Position = Position;
    type Error = Error;
    fn advance_with<B, L: Left, R: Right>(
        &mut self,
        _u: &mut B,
        l: &mut L,
        _r: &mut R,
    ) -> Result<Self::Position, Self::Error> {
        let (left_in_len, left_out_len) = l.left_lens();
        let (left_inputs, left_out_b) = l.left_bufs_mut();

        let left_in_b = match left_inputs {
            InBuffer::Single(buf) => buf,
            InBuffer::Double(buf1, buf2) => {
                todo!()
            },
        };
        
        if !self.initiated && left_in_len == 0 {
            left_out_b[0..4].copy_from_slice("TICK".as_bytes());
            l.left_set_lens(0, 4);
            self.initiated = true;
            return Ok(Position);
        }

        let inputs = &mut left_in_b.chunks_exact_mut(4);
        let mut in_discard = 0;
        let mut out_added = 0;
        let mut cur_pos = 0;

        while let Some(ref mut input) = &mut inputs.next() {
            let out = match core::str::from_utf8(&input).as_ref() {
                Ok(&"TICK") => "TOCK",
                Ok(&"TOCK") => "TICK",
                _ => return Err(Error::InvalidInput),
            };

            input.copy_from_slice(&[0u8; 4]);
            left_out_b[cur_pos..cur_pos + 4].copy_from_slice(out.as_bytes());
            in_discard += 4;
            out_added += 4;

            cur_pos += 4;
        }
        let new_in_len = left_in_len - in_discard;
        let new_out_len = left_out_len + out_added;
        l.left_set_lens(new_in_len, new_out_len);
        Ok(Position)
    }
}

/// boiler
pub struct Config;

impl BluePrint<TickTocking> for TickTock {
    type Config = Config;
    type Error = Error;

    fn with_defaults() -> Result<TickTocking, Self::Error> {
        Ok(TickTocking::default())
    }
    fn with_configuration(_: Self::Config) -> Result<TickTocking, Self::Error> {
        todo!()
    }
}
