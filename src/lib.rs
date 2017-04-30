#[cfg(test)]

extern crate serde_json;

mod gravity_set {
    
    // T is a 2 or 3-element array of f64 defining
    // the coordinates of the star.
    #[derive(Debug)]
    pub struct Star<T> {
        mass: f64,
        coordinate: T
    }

    impl <T> Star<T> {
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
    
    #[derive(Debug)]
    pub struct GSystem<T> {
        stars: Vec<Star<T>>,
        asize: u32,
        lcorner: T,
        ucorner: T
    }


    impl <T> GSystem<T> {
        pub fn new(stars: Vec<Star<T>>,
                   msize: u32,
                   upper: T,
                   lower: T) -> GSystem<T> {
            GSystem::<T> { stars: stars,
                           asize: 2u32.pow(msize),
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
        let gs = GSystem::<A>::new(vs, 8, [0.,0.,0.], [1.,1.,1.]);
    }
}
