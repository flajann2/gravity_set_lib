/// Compute module for Gravity Set
extern crate num_traits;

use ::{Star, GSystem, Position, Velocity, Acceleration, Idx};
use std::vec::Vec;
//use std::num;
use std::ops::{Index, IndexMut};
use self::num_traits::pow;

struct IJK {
    i: Idx,
    j: Idx,
    k: Idx
}

impl IJK {
    pub fn new() -> IJK{
        IJK{ i: 0, j: 0, k: 0 }
    }
}

pub struct Field<'a> {
    plex: Idx,
    space: Vec<u16>,
    gs: &'a GSystem
}

impl<'a> Field<'a> {
    pub fn new(plex: Idx, gs: &GSystem) -> Field {
        Field {
            plex: plex,
            space: vec![0; pow(pow(2, plex), 3)],
            gs: gs
        }
    }

    /// All indecies extend from 0 to plex - 1
    /// i is the least significant, k the most significant.
    pub fn ijk(&self, ind: &IJK) -> &u16 {
        &self.space[ind.i + (ind.j * self.plex) + (ind.k * self.plex * self.plex)]
    }
}

impl<'a> Index<Position> for Field<'a> {
    type Output = u16;
    fn index(&self, position: Position) -> &u16 {
        let ijk = IJK {
            i: 0,
            j: 0,
            k: 0
        };

        self.ijk(&ijk)
    }
}

#[test]
fn field_is_working() {
    type S = Star;
    let mut vs = Vec::<S>::new();
    vs.push(S::new(1.0, Position{x: 2.0, y: 3.1, z: -1.0}));
    let mut gs = GSystem::new(vs,
                              8,
                              0.1,
                              256,
                              Position{x: 0., y: 0., z: 0.},
                              Position{x: 1., y: 1., z: 1.});
    let plex = 8;
    let side = pow(2, plex);
    let vsize = pow(side, 3);
    let mut f = Field::new(plex, &gs);

    assert_eq!(f.space.len(), vsize);
    f.space[1] = 3;
}
