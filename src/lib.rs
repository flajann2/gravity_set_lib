#[cfg(test)]

extern crate serde_json;

mod gravity_set {
    
    // T is a 2 or 3-element array of f64 defining
    // the coordinates of the star.
    struct Star<T> {
        mass: f64,
        coordinate: T
    }
    
    #[test]
    fn star_is_working() {
        let s: Star<[f64; 3]> = Star { mass: 100.0, coordinate: [1.1, 2.2, 3.3]};
    }
    
    struct GSystem<T> {
        stars: Vec<Star<T>>,
        asize: u32,
        lcorner: T,
        ucorner: T
    }


    impl <T> GSystem<T> {
        fn new(stars: Vec<Star<T>>,
               msize: u16,
               upper: T,
               lower: T) -> GSystem<T> {
            GSystem::<T> { stars: stars, asize: 2 ** msize, lcorner: lower, ucorner: upper }
        }
    }

    #[test]
    fn gsystem_is_working() {
        let vs: vec![Star<[f64;3]>];
        let gs: GSystem<[f64;3]> = GSystem::<[f64;3]>.new(stars: vs,
                                                          msize: 8,
                                                          lcorner: [0.,0.,0.],
                                                          ucorner: [1.,1.,1.]);
    }
}
