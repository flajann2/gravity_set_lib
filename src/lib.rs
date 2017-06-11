#[cfg(test)]

extern crate serde;
extern crate serde_json;
extern crate ndarray;

#[macro_use]
extern crate serde_derive;

use serde_json::{from_str, Error};
use std::thread;
use std::result::Result;
use ndarray::{Array, Array2, Array3, Dim};

mod compute;


/// Mainly for the fixed coordinates of the Stars
pub struct Coord {
    x: f64,
    y: f64,
    z: f64
}

impl Coord {
    fn zeros() -> Coord {
        Coord {x: 0.0, y: 0.0, z: 0.0}
    }
}

/// For the numerical integration
type Position     = Coord;
type Velocity     = Coord;
type Acceleration = Coord;

/// the coordinates of the star.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Star {
    mass: f64,
    coordinate: Coord
}

impl Star {
    pub fn new(mass: f64, coord: Coord) -> Star {
        Star { mass: mass,
               coordinate: coord }
    }
}

#[test]
fn star_is_working() {
    let s = Star { mass: 100.0,
                   coordinate: Coord.new(1.1, 2.2, 3.3) };
}

/// Total description of the
/// gravitational system.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct GSystem {
    stars: Vec<Star>,
    asize: u32,
    delta: f64,
    max_iter: u32,
    lcorner: Coord,
    ucorner: Coord
}

impl GSystem {
    pub fn new(stars: Vec<Star>,
               msize: u32,
               delta: f64,
               miter: u32,
               upper: Coord,
               lower: Coord) -> GSystem {
        GSystem { stars: stars,
                  asize: 2u32.pow(msize),
                  delta: delta,
                  max_iter: miter,
                  lcorner: lower,
                  ucorner: upper }
    }
    
    /// This iterates for a single point
    pub fn iterate(initial: Position) -> u32 {
        0
    }

    fn center_of_mass(&self) -> Coord {
        let mut total_mass: f64 = 0.0;
        let mut saccum: Coord = Coord.zeros();
        for star in &self.stars {
            total_mass += star.mass;
        }
        saccum
    }
}

#[test]
fn gsystem_is_working() {
    type S = Star;
    let mut vs = Vec::<S>::new();
    vs.push(S::new(1.0, [2.0, 3.1, -1.0]));
    println!("vec: {:?}", vs);
    let gs = GSystem::new(vs,
                          8,
                          0.1,
                          256,
                          [0.,0.,0.],
                          [1.,1.,1.]);
    assert_eq!(gs.max_iter, 256);
}

// Run
#[no_mangle]
pub extern fn run_gs(json: &str) -> Result<GSystem, Error> {
    let gs: GSystem = from_str(json)?;
    Ok(gs)
}
