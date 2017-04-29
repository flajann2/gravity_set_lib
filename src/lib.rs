#[cfg(test)]
//#[macro_use(s)]

//extern crate ndarray;
//use ndarray::Array;
//use ndarray::Dimension;
//use ndarray::Axis;

mod gravity_set {
    
    // COOR is a 2 or 3-element array of f64 defining
    // the coordinates of the star.
    struct Star<COOR> {
        mass: f64,
        coordinate: COOR
    }
    
    #[test]
    fn star_is_working() {
        let s: Star<[f64; 3]> = Star { mass: 100.0, coordinate: [1.1, 2.2, 3.3]};
    }
    
    struct GSystem<COOR> {
        //stars: Vec<Star>,
        msize: u16,
        lcorner: COOR,
        ucorner: COOR
    }

    #[test]
    fn gsystem_is_working() {
        let gs: GSystem<[f64; 3]> = GSystem { msize: 8,
                                              lcorner: [0.,0.,0.],
                                              ucorner: [1.,1.,1.]};
    }
}
