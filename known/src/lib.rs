#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

pub use blueprint::{Left, NoLeft, NoRight, Orbit, Right};

use blueprint_ytls::{CryptoConfig, CryptoRng, TlsServerCtxConfig};

/// All the known Orbits enum-dispatch
pub enum Orbits<Config, Crypto, Rng, App> {
    /// Known yTls Server Orbit
    //#[cfg(feature = "ytls-server")]
    YtlsServer(blueprint_ytls::TlsServerOrbit<Config, Crypto, Rng>),
    /// Known Tls Orbit
    #[cfg(feature = "rustls")]
    Rustls(blueprint_rustls::TlsContext),
    /// Known TickTock Orbit
    #[cfg(feature = "tick-tock")]
    TickTock(blueprint_tick_tock::TickTocking),
    /// Known H11Server Orbit
    #[cfg(feature = "h11server")]
    H11Server(blueprint_h11spec::H11Serving<App>),
}

impl<Config, Crypto, Rng, App> core::fmt::Debug for Orbits<Config, Crypto, Rng, App> {
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

use blueprint_h11spec::{
    RequestBodyReceiver, RequestHeaderReceiver, RequestReceiver, Reset, ResponseSender,
};

impl<Config, Crypto, Rng, App> Orbit for Orbits<Config, Crypto, Rng, App>
where
    Config: TlsServerCtxConfig,
    Crypto: CryptoConfig + Clone,
    Rng: CryptoRng,
    App: ResponseSender
        + RequestReceiver
        + RequestHeaderReceiver
        + RequestBodyReceiver
        + Reset
        + Default,
{
    type Position = NoPosition;
    type Error = NoError;
    #[inline]
    fn advance_with<B, L: Left, R: Right>(
        &mut self,
        b: &mut B,
        l: &mut L,
        r: &mut R,
    ) -> Result<Self::Position, Self::Error> {
        _ = match self {
            #[cfg(feature = "ytls-server")]
            Self::YtlsServer(t) => {
                let _ = t.advance_with(b, l, r);
            }
            #[cfg(feature = "rustls")]
            Self::Rustls(t) => {
                let _ = t.advance_with(b, l, r);
            }
            #[cfg(feature = "tick-tock")]
            Self::TickTock(t) => {
                let _ = t.advance_with(b, l, r);
            }
            #[cfg(feature = "h11server")]
            Self::H11Server(t) => {
                let _ = t.advance_with(b, l, r);
            }
        };
        // TODO: harmonize the error & Position
        Ok(NoPosition)
    }
}
