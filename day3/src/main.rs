use std::fs;
#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

fn main() {
    let data = fs::read_to_string("easy_input")
                   .expect("Unable to read input file");
    let claims: Vec<Claim> = data.split("\n")
                                 .map(|text| build_claim_from_text(text).unwrap())
                                 .collect();
    for c in claims {
        println!{"{}", c}
    }
}

#[derive(Debug)]
struct Point(i32, i32);

#[derive(Debug)]
struct Claim {
    identity: i32,
    origin: Point,
    width: i32,
    height: i32,
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

fn build_claim(identity: i32, origin: Point, width: i32, height: i32) -> Claim {
    Claim {
        identity,
        origin,
        width,
        height,
    }
}

fn build_claim_from_text(text: &str) -> Result<Claim, std::num::ParseIntError> {
    // input string looks like this "#id @ x,y: WxH"
    lazy_static! {
       static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")
                                .unwrap();
    }

    let caps = RE.captures(text).unwrap();
    let id = caps.get(1).unwrap().as_str().parse::<i32>()?;
    let x = caps.get(2).unwrap().as_str().parse::<i32>()?;
    let y = caps.get(3).unwrap().as_str().parse::<i32>()?;
    let w = caps.get(4).unwrap().as_str().parse::<i32>()?;
    let h = caps.get(5).unwrap().as_str().parse::<i32>()?;
    
    Ok(build_claim(id, Point(x,y), w, h))
}
