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

/// 2D and 3D coords
type TwoD   = [f64;2];
type ThreeD = [f64;3];

/// 2- and 3-dimensional GS
pub trait Coord {}
impl Coord for TwoD {}
impl Coord for ThreeD {}

/// T is a 2 or 3-element array of f64 defining
/// the coordinates of the star.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Star<T: Coord> {
    mass: f64,
    coordinate: T
}

impl <T: Coord> Star<T> {
    pub fn new(mass: f64, coord: T) -> Star<T> {
        Star::<T> { mass: mass,
                    coordinate: coord }
    }
}

#[test]
fn star_is_working() {
    let s = Star::<ThreeD> { mass: 100.0,
                             coordinate: [1.1, 2.2, 3.3]};
}

/// Total description of the
/// gravitational system.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct GSystem<T: Coord> {
    stars: Vec<Star<T>>,
    asize: u32,
    delta: f64,
    max_iter: u32,
    lcorner: T,
    ucorner: T
}

impl <T: Coord> GSystem<T> {
    pub fn new(stars: Vec<Star<T>>,
               msize: u32,
               delta: f64,
               miter: u32,
               upper: T,
               lower: T) -> GSystem<T> {
        GSystem::<T> { stars: stars,
                       asize: 2u32.pow(msize),
                       delta: delta,
                       max_iter: miter,
                       lcorner: lower,
                       ucorner: upper }
    }
    
    /// This iterates for a single point
    pub fn iterate(initial: T) -> u32 {
        0
    }

    fn center_of_mass(&self) -> T {
        let mut total_mass: f64 = 0.0;
        let mut saccum: T;
        match saccum {
            TwoD => [0.0, 0.0],
            ThreeD => [0.0, 0.0, 0.0],
        }
        for star in &self.stars {
            total_mass += star.mass;
            
        }
        saccum
    }
}

type GSystem2 = GSystem<TwoD>;
type GSystem3 = GSystem<ThreeD>;
 
#[test]
fn gsystem_is_working() {
    type S = Star<ThreeD>;
    let mut vs = Vec::<S>::new();
    vs.push(S::new(1.0, [2.0, 3.1, -1.0]));
    println!("vec: {:?}", vs);
    let gs = GSystem3::new(vs,
                           8,
                           0.1,
                           256,
                           [0.,0.,0.],
                           [1.,1.,1.]);
    assert_eq!(gs.max_iter, 256);
}

// Run 2D
#[no_mangle]
pub extern fn run_gs2(json: &str) -> Result<GSystem2, Error> {
    let gs: GSystem2 = from_str(json)?;
    Ok(gs)
}

// Run 3D
#[no_mangle]
pub extern fn run_gs3(json: &str) -> Result<GSystem3, Error> {
    let gs: GSystem3 = from_str(json)?;
    Ok(gs)
}   
