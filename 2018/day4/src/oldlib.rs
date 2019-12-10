#[macro_use] extern crate lazy_static;

// NOTE(rgasper) unused
pub fn try_i32_to_usize(i:i32) -> Option<usize> {
    if   (i < std::usize::MIN as i32) | (i > std::usize::MAX as i32) {
        None
    } else {
        Some(i as usize)
    }
}

pub mod claim {
    //imports
    extern crate regex;
    use regex::Regex;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use lazy_static;
    
    //statics
    static AUTO_INCREMENT: AtomicUsize = AtomicUsize::new(1);
    // structs
    #[derive(Debug, PartialEq, Eq, PartialOrd, Copy, Clone)]
    pub struct Point(pub usize, pub usize);

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Claim {
        // distinct identity is assigned at build time in build_claim
        pub identity: usize,
        pub origin: Point,
        pub width: usize,
        pub height: usize,
    }

    // struct methods
    impl Claim {
        
        pub fn identity(&self) -> usize {
            self.identity
        }

        pub fn area(&self ) -> usize {
            self.width * self.height
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
    pub fn build_claim(origin: Point, width: usize, height: usize) -> Claim {
        let identity = AUTO_INCREMENT.fetch_add(1, Ordering::SeqCst);
        Claim {
            identity,
            origin,
            width,
            height,
        }
    }

    //NOTE(rgasper) use a custom error instead of this innacurate error once I know how to make a new error
    pub fn build_claim_from_text(text: &str) -> Result<Claim, std::num::ParseIntError> {
        // input string looks like this "#id @ x,y: WxH"
        // only compile the regex a single time
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")
                                    .unwrap();
        }
        let caps: Vec<usize> = RE.captures(text)
                                .unwrap()
                                .iter()
                                .map(|cap| cap.unwrap().as_str().parse::<usize>().unwrap_or(0)) // capture zero is the entire string, so it will fail and turn into 0
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