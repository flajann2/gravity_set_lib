#[cfg(test)]

extern crate serde;
extern crate serde_json;
extern crate ndarray;

#[macro_use]
extern crate serde_derive;

pub mod gravity_set {
    use serde_json::from_str;
    use serde_json::Error;
    use std::thread;
    use std::result::Result;
    use ndarray::Array;
    use ndarray::Array2;
    use ndarray::Array3;
    use ndarray::Dim;

    /// 2D and 3D coords
    type TwoD   = [f64;2];
    type ThreeD = [f64;3];

    /// 2- and 3-dimensional GS
    pub trait Coord {}

    impl Coord for [f64;2] {}
    impl Coord for [f64;3] {}
    
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
        let s = Star::<ThreeD> { mass: 100.0, coordinate: [1.1, 2.2, 3.3]};
    }

    /// Total description of the
    /// gravitational system.
    #[derive(Debug)]
    #[derive(Serialize, Deserialize)]
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

    type GSystem2 = GSystem<TwoD>;
    type GSystem3 = GSystem<ThreeD>;
    
    #[test]
    fn gsystem_is_working() {
        type S = Star<ThreeD>;
        let mut vs = Vec::<S>::new();
        vs.push(S::new(1.0, [2.0, 3.1, -1.0]));
        println!("vec: {:?}", vs);
        let gs = GSystem::<A>::new(vs, 8, 0.1, [0.,0.,0.], [1.,1.,1.]);
        assert!(gs);
    }

    /// Run 2D
    #[no_mangle]
    pub extern fn run_gs2(json: &str) -> Result<GSystem2, Error> {
        let gs: GSystem2 = from_str(json)?;
        Ok(gs)
    }   

    /// Run 3D
    #[no_mangle]
    pub extern fn run_gs3(json: &str) -> Result<GSystem3, Error> {
        let gs: GSystem3 = from_str(json)?;
        Ok(gs)
    }   
}
