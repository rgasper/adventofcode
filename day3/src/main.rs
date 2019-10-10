use std::fs;
use day3::{claim};
use ndarray::{Array, s};

fn main() {

    let data = fs::read_to_string("input")
                   .expect("Unable to read input file");

    let claims: Vec<claim::Claim> = data.split('\n')
                                        .map(|text| claim::build_claim_from_text(text).unwrap())
                                        .collect();
    let size = (1000,1000);
    let mut cloth = Array::from_elem(size, 0u32);
    
    for claim in claims {
        let xlo = claim.origin.0;
        let ylo = claim.origin.1;
        let xhi = xlo+claim.width;
        let yhi = ylo+claim.height;

        let mut s = cloth.slice_mut(s![xlo..xhi, ylo..yhi]);
        s += 1;
    }

    let shared: u32 = cloth.iter()
                           .filter( |x| x > &&1u32 )
                           .map( |_x| 1 )
                           .sum();
    //println!("{}", cloth);
    println!("{} square inches of fabric are shared", shared);
}