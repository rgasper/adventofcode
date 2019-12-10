#[macro_use] extern crate lazy_static;

// NOTE(rgasper) unused
pub fn try_i32_to_usize(i:i32) -> Option<usize> {
    if   (i < std::usize::MIN as i32) | (i > std::usize::MAX as i32) {
        None
    } else {
        Some(i as usize)
    }
}

pub mod schedule {
    //imports
    extern crate regex;
    extern crate datetime;
    use datetime;
    use regex::Regex;
    use lazy_static;
    
    //NOTE(rgasper) use a custom error instead of this innacurate error once I know how to make a new error
    pub fn build_time_from_text(text: &str) -> Result<TimePiece, std::num::ParseIntError> {
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