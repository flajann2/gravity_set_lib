/// Compute module for Gravity Set

use ::{GSystem, Coord};
use std::vec::Vec;
use std::num;
use std::ops::{Index, IndexMut};

pub struct Field {
    plex: u32,
    space: Vec<u16>
}

impl Field {
    pub fn new(plex: u32) -> Field {
        Field {
            plex: plex,
            space: vec![0; 2usize.pow(plex).pow(3u32)]
        }
    }

    ///
    pub fn ijk(&self, i: u32, j: u32, k: u32) -> &u16 {

    }
}

impl Index<Coord> for Field {
    type Output = u32;
    fn index(&self, coord: Coord) -> u16 {
        self.space[i]
    }
}

#[test]
fn field_is_working() {
    let plex = 8u32;
    let side = 2usize.pow(plex);
    let vsize = side.pow(3u32);
    let mut f = Field::new(plex);

    assert_eq!(f.space.len(), vsize);
    f.space[1] = 3;
}
