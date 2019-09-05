use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    let mut doubles = 0;
    let mut triples = 0;
    let data = fs::read_to_string("input")
        .expect("Unable to read file");
    for box_tag in data.split("\n") {
        let mut counts: HashMap<char,u32> = HashMap::new();
        for letter in box_tag.chars() {
            *counts.entry(letter).or_insert(0) += 1;
        }
        let mut flag2 = false;
        let mut flag3 = false;
        for v in counts.values() {
            match v.cmp(&2) {
                Ordering::Equal => flag2 = true,
                _ => (),
            }
            match v.cmp(&3) {
                Ordering::Equal => flag3 = true,
                _ => (),
            }
        }
        if flag2 {doubles += 1};
        if flag3 {triples += 1};
    }
    println!("Doubles {}, Triples {}, checksum {}", doubles, triples, doubles*triples );
}