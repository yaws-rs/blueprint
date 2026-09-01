#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

pub use blueprint::{Orbit, Left, Right, NoLeft, NoRight};

/// All the known Orbits enum-dispatch
pub enum Orbits {
    /// Known yTls Server Orbit
    #[cfg(feature = "ytls-server")]
    YtlsServer(blueprint_ytls::TlsServerOrbit),
    /// Known Tls Orbit
    #[cfg(feature = "rustls")]
    Rustls(blueprint_rustls::TlsContext),
    /// Known TickTock Orbit
    #[cfg(feature = "tick-tock")]
    TickTock(blueprint_tick_tock::TickTocking),
    /// Known H11Server Orbit
    #[cfg(feature = "h11server")]
    H11Server(blueprint_h11spec::H11Serving),
}

impl core::fmt::Debug for Orbits {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "Orbits::Something")
    }
}

/// No harmonized position currently used
#[derive(Debug)]
pub struct NoPosition;

// TODO: harmonise the error & Position
/// No harmonized error currently used
#[derive(Debug)]
pub struct NoError;

#[cfg(feature = "ytls-server")]
use blueprint_ytls::{TlsServerCtxConfig, CryptoConfig, CryptoRng};

/*
#[cfg(not(feature = "ytls-server"))]
mod dummy {
    struct TlsServerCtxConfig;
    struct CryptoConfig;
    struct CryptoRng;
}
#[cfg(not(feature = "ytls-server"))]
use dummy::*; */

impl Orbit for Orbits {
    type Position = NoPosition;
    type Error = NoError;
    #[inline]
    fn advance_with<B, L: Left, R: Right>(&mut self,b: &mut B,l: &mut L,r: &mut R) -> Result<Self::Position, Self::Error> {
        _ = match self {
            #[cfg(feature = "ytls-server")]
            Self::YtlsServer(t) => { t.advance_with(b, l, r); },
            #[cfg(feature = "rustls")]
            Self::Rustls(t) => { t.advance_with(b, l, r); },
            #[cfg(feature = "tick-tock")]
            Self::TickTock(t) => { t.advance_with(b, l, r); },
            #[cfg(feature = "h11server")]
            Self::H11Server(t) => { t.advance_with(b, l, r); },
            _ => unreachable!(),
        };
        // TODO: harmonize the error & Position
        Ok(NoPosition)
    } 
}
