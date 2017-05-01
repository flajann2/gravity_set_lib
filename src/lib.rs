#[cfg(test)]

extern crate serde_json;
extern crate ndarray;

use ndarray::Array;
use ndarray::Array2;
use ndarray::Array3;
use ndarray::Dim;

mod gravity_set {
    /// 2- and 3-dimensional GS
    pub trait Coord {}

    impl Coord for [f64;2] {}
    impl Coord for [f64;3] {}
    
    /// T is a 2 or 3-element array of f64 defining
    /// the coordinates of the star.
    #[derive(Debug)]
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
        type A = [f64;3];
        let s = Star::<A> { mass: 100.0, coordinate: [1.1, 2.2, 3.3]};
    }

    /// Total description of the
    /// gravitational system.
    #[derive(Debug)]
    pub struct GSystem<T: Coord> {
        stars: Vec<Star<T>>,
        asize: u32,
        delta: f64,
        lcorner: T,
        ucorner: T
    }


    impl <T: Coord> GSystem<T> {
        pub fn new(stars: Vec<Star<T>>,
                   msize: u32,
                   delta: f64,
                   upper: T,
                   lower: T) -> GSystem<T> {
            GSystem::<T> { stars: stars,
                           asize: 2u32.pow(msize),
                           delta: delta,
                           lcorner: lower,
                           ucorner: upper }
        }
    }
    
    #[test]
    fn gsystem_is_working() {
        type A = [f64;3];
        type S = Star<A>;
        let mut vs = Vec::<S>::new();
        vs.push(S::new(1.0, [2.0, 3.1, -1.0]));
        println!("vec: {:?}", vs);
        let gs = GSystem::<A>::new(vs, 8, 0.1, [0.,0.,0.], [1.,1.,1.]);
    }
}
