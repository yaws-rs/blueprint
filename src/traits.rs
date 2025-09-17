//! Blueprint traits

/// Use when Left side is not used
pub struct NoLeft;
impl Left for NoLeft {
    /// boiler
    fn left_lens(&self) -> (usize, usize) {
        unreachable!()
    }
    /// boiler
    fn left_set_lens(&mut self, _: usize, _: usize) -> () {
        unreachable!()
    }
    /// boiler
    fn left_bufs_mut(&mut self) -> (&mut [u8], &mut [u8]) {
        unreachable!()
    }
}

/// Left side of state machine I/O
pub trait Left {
    /// Lengths of Input and Output of Left side
    fn left_lens(&self) -> (usize, usize);
    /// Set the Lengths of Input and Output of Left side
    fn left_set_lens(&mut self, _: usize, _: usize) -> ();
    /// Mutable Input and Output bufs of Left side
    fn left_bufs_mut(&mut self) -> (&mut [u8], &mut [u8]);
}

/// Use when Right side is not used
pub struct NoRight;
impl Right for NoRight {
    /// boiler
    fn out_len(&self) -> usize {
        unreachable!()
    }
    /// boiler
    fn buf_right_out(&self) -> &[u8] {
        unreachable!()
    }
    /// boiler
    fn wants_right_next_in(&self) -> bool {
        unreachable!()
    }
    /// boiler
    fn set_wants_right_next_in(&mut self, _: bool) -> () {
        unreachable!()
    }
    /// boiler
    fn all_sent_right_out(&mut self) -> () {
        unreachable!()
    }
    /// boiler
    fn add_right_out(&mut self, _: &[u8]) -> () {
        unreachable!()
    }
    /// boiler
    fn add_right_in(&mut self, _: &[u8]) -> () {
        unreachable!()
    }
}

/// Right side of state machine I/O
pub trait Right {
    /// Output length of Right side
    fn out_len(&self) -> usize;
    /// Indicate processing of Output of Right side
    fn buf_right_out(&self) -> &[u8];
    /// Indicate whether Right side wants next input block
    fn wants_right_next_in(&self) -> bool;
    /// SM: Indicate Right side to want the next block
    fn set_wants_right_next_in(&mut self, _: bool) -> ();
    /// SM: Called when all Right side Output was consumed
    fn all_sent_right_out(&mut self) -> ();
    /// Add bytes to Right output
    // TODO: Fragmentation & Typification
    fn add_right_out(&mut self, _: &[u8]) -> ();
    /// Add bytes to Right input
    // TODO: Fragmentation & Typification
    fn add_right_in(&mut self, _: &[u8]) -> ();
}

/// Provide Portal between two Orbits Right and Left sides
pub trait Portal {
    /// Position custom type
    type Position;
    /// Trade Right and Left sides
    fn trade<R: Right, L: Left>(&mut self, _: R, _: L) -> Self::Position;
}

/// ..
pub trait Orbit {
    /// .
    type Position;
    /// .
    type Error;
    /// Userdata, sides Left and Right
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
