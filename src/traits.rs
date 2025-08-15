//! Blueprint traits

/// ..
pub trait Orbit {
    /// .
    type Position;
    /// .
    type Error;
    /// .
    fn advance_with<B>(&mut self, _: &mut B, _: &mut [u8]) -> Result<Self::Position, Self::Error>;
}

/// .
pub trait BluePrint<O: Orbit> {
    /// .
    type Config;
    /// .
    type Error;    
    /// .
    fn with_defaults() -> Result<O, Self::Error>;
    /// .
    fn with_configuration(_: Self::Config) -> Result<O, Self::Error>;
}
