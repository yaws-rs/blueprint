//! Tick Tock but within Orbit of a Blueprint

use blueprint::{BluePrint, Left, NoRight, Orbit};
use blueprint_tick_tock::TickTock;

#[derive(Default, Debug)]
struct InOut([u8; 4], [u8; 4]);

impl Left for InOut {
    /// Lengths of Input and Output of Left side
    fn left_lens(&self) -> (usize, usize) {
        (4, 0)
    }
    /// Set the Lengths of Input and Output of Left side
    fn left_set_lens(&mut self, _: usize, _: usize) -> () {
        ()
    }
    /// Mutable Input and Output bufs of Left side
    fn left_bufs_mut(&mut self) -> (&mut [u8], &mut [u8]) {
        (&mut self.0, &mut self.1)
    }
}

fn main() {
    let input: [u8; 4] = [84, 73, 67, 75];
    let zero: [u8; 4] = [0, 0, 0, 0];

    let mut user = 0;
    let mut left = InOut(input.clone(), zero);
    let mut right = NoRight;

    let mut orbit = TickTock::with_defaults().unwrap();
    let mut rounds = 0;

    while rounds < 5 {
        let snds_bufs = left.left_bufs_mut();
        println!("Sending {}", core::str::from_utf8(&snds_bufs.0).unwrap());
        drop(snds_bufs);
        orbit
            .advance_with(&mut user, &mut left, &mut right)
            .unwrap();
        let res_bufs = left.left_bufs_mut();
        println!("Received {}", core::str::from_utf8(res_bufs.1).unwrap());
        core::mem::swap(&mut left.0, &mut left.1);
        rounds += 1;
    }
}
