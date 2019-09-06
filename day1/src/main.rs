use std::fs;
use std::collections::HashSet;

fn main() {
    let mut frequencies: HashSet<i32> = HashSet::new();
    let mut frequency: i32 = 0;
    frequencies.insert(frequency);
    let data = fs::read_to_string("input")
        .expect("Unable to read file");
    let data_iter = data.split_whitespace();
    data_iter.map(|numstr| numstr.trim().parse());
}