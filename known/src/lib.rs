#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

pub use blueprint::{Orbit, Left, Right, NoLeft, NoRight};

pub enum Orbits {
    #[cfg(feature = "tls")]
    Tls(blueprint_tls::TlsContext),
    #[cfg(feature = "tick-tock")]
    TickTock(blueprint_tick_tock::TickTocking),
    #[cfg(feature = "h11server")]
    H11Server(blueprint_h11spec::H11Serving),
}

impl core::fmt::Debug for Orbits {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "Orbits::Something")
    }
}

#[derive(Debug)]
pub struct NoPosition;

// TODO: harmonise the error & Position
#[derive(Debug)]
pub struct NoError;

impl Orbit for Orbits {
    type Position = NoPosition;
    type Error = NoError;
    #[inline]
    fn advance_with<B, L: Left, R: Right>(&mut self,b: &mut B,l: &mut L,r: &mut R) -> Result<Self::Position, Self::Error> {
        _ = match self {
            #[cfg(feature = "tls")]
            Self::Tls(t) => { t.advance_with(b, l, r); },
            #[cfg(feature = "tick-tock")]
            Self::TickTock(t) => { t.advance_with(b, l, r); },
            #[cfg(feature = "h11server")]
            Self::H11Server(t) => { t.advance_with(b, l, r); },
        };
        // TODO: harmonize the error & Position
        Ok(NoPosition)
    } 
}
