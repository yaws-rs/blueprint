//! Blueprint traits (Typed)

#[cfg(test)]
mod typed_test;

// temp
use crate::InBuffer;

// As the name implies, used for dummy No-types.
#[derive(Debug)]
pub struct TypedNothingBurger;

/*
/// Use when Left side is not used
pub struct TypedNoLeft;
impl TypedLeft<Tlin, Tlout> for TypedNoLeft {
//    type CustomInType = TypedNothingBurger;
//    type CustomOutType = TypedNothingBurger;

    /// boiler
    fn left_in_blocked(&self) -> bool {
        unreachable!()
    }
    /// boiler
    fn set_left_in_blocked(&mut self, _: bool) -> () {}
    /// boiler
    fn left_lens(&self) -> (usize, usize) {
        unreachable!()
    }
    /// boiler
    fn left_set_lens(&mut self, _: usize, _: usize) -> () {
        unreachable!()
    }    
    /// boiler
    fn left_typed_mut<'d>(&'d mut self) -> (&'d mut Self::CustomInType, &'d mut Self::CustomOutType) {
        unreachable!()
    }
    fn is_ready(&self) -> bool {
        unreachable!()
    }
    fn set_ready(&mut self, _: bool) -> bool {
        unreachable!()
    }
    fn left_want_read(&self) -> bool {
        unreachable!()
    }
    fn set_left_want_read(&mut self, _: bool) -> () {
        unreachable!()
    }
    fn left_want_write(&self) -> bool {
        unreachable!()
    }
    fn set_left_want_write(&mut self, _: bool) -> () {
        unreachable!()
    }
    fn shutdown(&mut self) -> () {
        unreachable!()
    }
}
*/

/// Typed Left side of state machine I/O
pub trait TypedLeft<'d, T: ?Sized> {
    /// Is Left in blocked?
    fn left_in_blocked(&self) -> bool;
    /// Set Left in blocked
    fn set_left_in_blocked(&mut self, _: bool) -> ();
    /// Lengths of Input and Output of Left side
    fn left_lens(&self) -> (usize, usize);
    /// Set the Lengths of Input and Output of Left side
    fn left_set_lens(&mut self, _: usize, _: usize) -> ();
    /// Typed Left Meta
    fn left_typed_meta(&'d mut self) -> &'d mut T;
    /// Mutable Input and Output of Left side Custom In and Out Types
    fn left_typed_mut(&'d mut self) -> (&'d mut T, &'d mut T);
    /// Indicates whether the Left side is ready for Right side
    fn is_ready(&self) -> bool;
    /// Set the layer readiness for Right side input and output
    fn set_ready(&mut self, _: bool) -> bool;
    /// Indicates whether the Left side wants Input
    fn left_want_read(&self) -> bool;
    /// Set the Left side wanting to read
    fn set_left_want_read(&mut self, _: bool) -> ();
    /// Indicates whether the Left side wants Output
    fn left_want_write(&self) -> bool;
    /// Set the Left side wanting to write
    fn set_left_want_write(&mut self, _: bool) -> ();
    /// State machine signals shutdown (e.g. peer signals close)
    fn shutdown(&mut self) -> ();
}

/*
/// Use when Right Typed side is not used
pub struct TypedNoRight;
impl TypedRight for TypedNoRight {
    /// Custom Right Input type
    type CustomInType = TypedNothingBurger;
    /// Custom Right Output type
    type CustomOutType = TypedNothingBurger;

    /// boiler
    fn right_lens(&self) -> (usize, usize) {
        unreachable!()
    }
    /// boiler
    fn typed_right_out(&self) -> &Self::CustomOutType {
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
    fn add_right_out(&mut self, _: &Self::CustomOutType) -> () {
        unreachable!()
    }
    /// boiler
    fn add_right_in(&mut self, _: &Self::CustomInType) -> () {
        unreachable!()
    }
}
*/

/// Typed Right side of state machine I/O
pub trait TypedRight<T: ?Sized> {

    /// Output length of Right side
    fn right_lens(&self) -> (usize, usize);
    /// Indicate processing of Output of Right side
    fn typed_right_out(&self) -> &T;
    /// Indicate whether Right side wants next input block
    fn wants_right_next_in(&self) -> bool;
    /// SM: Indicate Right side to want the next block
    fn set_wants_right_next_in(&mut self, _: bool) -> ();
    /// SM: Called when all Right side Output was consumed
    fn all_sent_right_out(&mut self) -> ();
    /// Add bytes to Right output
    // TODO: Fragmentation & Typification
    fn add_right_out(&mut self, _: &T) -> ();
    /// Add bytes to Right input
    // TODO: Fragmentation & Typification
    fn add_right_in(&mut self, _: &T) -> ();
}

/*
/// Provide Typed Portal between two Orbits Right and Left sides
pub trait TypedPortal {
    /// Position custom type
    type Position;
    /// Typed Trade Right and Left sides
    fn typed_trade<R: TypedRight, L: TypedLeft>(&mut self, _: R, _: L) -> Self::Position;
}
*/

/// Orbit (Typed)
pub trait TypedOrbit<'d, 's: 'd, Tl: ?Sized, Tr: ?Sized> {
    /// Current Position of the Orbit.
    type Position;
    /// Error returned by the Orbit
    type Error;
    /// Advance given instantiated Orbit with Userdata, sides Left and Right.
    fn typed_advance_with<B, L: TypedLeft<'d, Tl>, R: TypedRight<Tr>>(
        &'d mut self,
        _: &'d mut B,
        _: &'d mut L,
        _: &'d mut R,
    ) -> Result<Self::Position, Self::Error>;
}

/// Typed BluePrint
pub trait TypedBluePrint<O: TypedOrbit<Tl, Tr>, Tl: ?Sized, Tr: ?Sized> {
    /// Configuration passed to the instantiated Orbit
    type Config;
    /// Error used by Orbit
    type Error;
    // Typed Left for Orbit
    //type TypedLeft;
    // Typed Right for Orbit
    //type TypedRight;
    /// Instantiate typed Orbit with Defaults
    fn typed_with_defaults() -> Result<O, Self::Error>;
    /// Instantiate typed Orbit with the given Configuration
    fn typed_with_configuration(_: Self::Config) -> Result<O, Self::Error>;    
}
