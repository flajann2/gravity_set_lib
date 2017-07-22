/// Compute module for Gravity Set

use ::{GSystem, Coord};
use std::vec::Vec;
use std::num;

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
}

#[test]
fn field_is_working() {
    let plex = 8u32;
    let vsize = 2usize.pow(plex).pow(3u32);
    let f = Field::new(plex);
    assert_eq!(f.space.len(), vsize);
}
