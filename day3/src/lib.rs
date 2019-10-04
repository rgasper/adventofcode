#[macro_use] extern crate lazy_static;
pub mod claim {
    //imports
    use std::cmp::{max, min};
    extern crate regex;
    use regex::Regex;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use lazy_static;
    use std::collections::HashSet;
    
    //statics
    static AUTO_INCREMENT: AtomicUsize = AtomicUsize::new(0);
    // structs
    #[derive(Debug, PartialEq, Eq, PartialOrd, Copy, Clone)]
    pub struct Point(i32, i32);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Claim {
        // claim identity is assigned on instantiation using AUTO_INCREMENT
        identity: usize,
        origin: Point,
        width: i32,
        height: i32,
        pub overlaps_with: HashSet<usize>,
    }

    // struct methods
    impl Claim {
        
        pub fn identity(&self) -> usize {
            self.identity
        }

        pub fn area(&self ) -> i32 {
            self.width * self.height
        }

        pub fn overlapping_claim(&self, other: &Claim) -> Option<Claim> {
            // returns a new claim that is the overlap of self and other
            let topleft  = Point( max(self.origin.0, other.origin.0), 
                                  max(self.origin.1, other.origin.1)
                                );
            let botright = Point( min(self.origin.0 + self.width, other.origin.0 + other.width),
                                  min(self.origin.1 + self.height, other.origin.1 + other.height)
                                );
            let overlap = build_claim(topleft.clone(), botright.0-topleft.0, botright.1-topleft.1);
            if overlap.area() > 0 {
                Some(overlap)
            } else {
                None
            }
        }
    }

    impl std::fmt::Display for Claim {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "#{} @ {},{}: {}x{}", self.identity,
                                            self.origin.0,
                                            self.origin.1,
                                            self.width,
                                            self.height)
        }
    }

    //builders
    pub fn build_claim(origin: Point, width: i32, height: i32) -> Claim {
        let identity = AUTO_INCREMENT.fetch_add(1, Ordering::SeqCst);
        let overlaps_with = HashSet::new();
        Claim {
            identity,
            origin,
            width,
            height,
            overlaps_with,
        }
    }

    pub fn build_claim_from_text(text: &str) -> Result<Claim, std::num::ParseIntError> {
        // input string looks like this "#id @ x,y: WxH"
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")
                                    .unwrap();
        }
        let caps: Vec<i32> = RE.captures(text)
                                .unwrap()
                                .iter()
                                .map(|cap| cap.unwrap().as_str().parse::<i32>().unwrap_or(0)) // I expect failure in capture 0 every time
                                .collect();

        let _id = caps[1];
        let x  = caps[2];
        let y  = caps[3];
        let w  = caps[4];
        let h  = caps[5];
        Ok(build_claim(Point(x,y), w, h))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}