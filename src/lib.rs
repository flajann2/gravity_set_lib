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
use std::ops::{Add, Mul, Div};

mod compute;

/// Defines the dimensionality
const PLY: usize = 3;


/// Represents all the scalar values
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Scalar{ value: f64 }

/// Vector represents all position, acceleration and velocity vectors
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Vector{ value: Vec<f64> }


impl Vector {
    fn zeros() -> Self {
        Vector{value: vec![0.0; PLY]}
    }
}

impl Add for Scalar {
    type Output = Self;

    fn add(self, other: Scalar) -> Self {
        Scalar{value: self.value + other.value}
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Vector) -> Self {
        Vector{ value: other.value.iter().zip(self.value.iter()).map(|(&p, &q)| p + q).collect() }
    }
}

impl Mul<Vector> for Scalar {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector{value: rhs.value.iter().map(|v| self.value * v).collect() }
    }
}

impl Div<Vector> for Scalar {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Vector {
        Vector{value: rhs.value.iter().map(|v| self.value / v).collect() }
    }
}

/// And for all the unsigned integers in our life
type UInt = u32;

/// For the numerical integration
type Position     = Vector;
type Velocity     = Vector;
type Acceleration = Vector;

/// For indexing
type Idx = usize;

/// the coordinates of the star.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Star {
    mass: Scalar,
    position: Position
}

impl Star {
    pub fn new(mass: f64, pos: Position) -> Star {
        Star { mass: Scalar{value: mass},
               position: pos }
    }
}

#[test]
fn star_is_working() {
    let s = Star { mass: Scalar{value: 100.0},
                   position: Position{x: 1.1, y: 2.2, z: 3.3} };
}

/// Total description of the
/// gravitational system.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct GSystem {
    stars: Vec<Star>,
    asize: u32,
    delta: Scalar,
    max_iter: u32,
    lcorner: Position,
    ucorner: Position,
    initial_velocity: Velocity,
    initial_acceleration: Acceleration
}

impl GSystem {
    pub fn new(stars: Vec<Star>,
               msize: UInt,
               delta: f64,
               miter: UInt,
               upper: Position,
               lower: Position) -> GSystem {
        GSystem { stars: stars,
                  asize: 2u32.pow(msize),
                  delta: Scalar{value: delta},
                  max_iter: miter,
                  lcorner: lower,
                  ucorner: upper,
                  initial_velocity: Velocity::zeros(),
                  initial_acceleration: Acceleration::zeros() }
    }

    /// This iterates for a single point
    pub fn iterate(&self, initial: Position) -> u32 {
        0
    }

    /// Calculate the acceleration vector acting on the Free Point Mass
    fn acceleration(&self, fpm: Position) -> Acceleration {
        Vector::zeros()
    }

    /// Calculate the velocity i given velocity i-1 and acceleration
    fn velocity(&self, a: Acceleration, pv: Velocity) -> Velocity {
        Velocity::zeros()
    }

    /// Calculate the position i given position i-1 and velocity.
    fn position(&self, v: Velocity, pp: Position) -> Position {
        pp
    }

    fn center_of_mass(&self) -> Vector {
        let mut total_mass: Scalar = Scalar{value: 0.0};
        let mut saccum: Position = Position::zeros();
        for star in &self.stars {
            total_mass += star.mass;
            saccum += star.pos * star.mass;
        }
        saccum / total_mass
    }
}

#[test]
fn gsystem_is_working() {
    type S = Star;
    let mut vs = Vec::<S>::new();
    vs.push(S::new(1.0, Position{x: 2.0, y: 3.1, z: -1.0}));
    println!("vec: {:?}", vs);
    let gs = GSystem::new(vs,
                          8,
                          0.1,
                          256,
                          Position{x: 0., y: 0., z: 0.},
                          Position{x: 1., y: 1., z: 1.});
    assert_eq!(gs.max_iter, 256);
}

// Run
#[no_mangle]
pub extern fn run_gs(json: &str) -> Result<GSystem, Error> {
    let gs: GSystem = from_str(json)?;
    Ok(gs)
}
