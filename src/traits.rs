//! Blueprint traits

/// Left side of state machine I/O
pub trait Left {
    /// Input of Left side
    fn bufs(&mut self) -> (&mut [u8], &mut [u8]);
}

/// Right side of state machine I/O
pub trait Right {}

/// ..
pub trait Orbit {
    /// .
    type Position;
    /// .
    type Error;
    /// Userdata, Left In, Left Out, Right In, Right Out
    //    fn advance_with<B>(&mut self, _: &mut B, _: &mut [u8], _: &mut [u8]) -> Result<Self::Position, Self::Error>;
    fn advance_with<B, L: Left, R: Right>(
        &mut self,
        _: &mut B,
        _: &mut L,
        _: &mut R,
    ) -> Result<Self::Position, Self::Error>;
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
