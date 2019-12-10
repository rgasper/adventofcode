use std::fs;
use edit_distance::edit_distance;

fn main() {
    let mut seen: Vec<String> = Vec::new();
    let data = fs::read_to_string("input")
        .expect("Unable to read file");
    // This is O(N**2), is there a way to do this better?
    for box_tag in data.split("\n") {
        for seen_tag in &seen {
            if edit_distance(seen_tag, box_tag) == 1 {
                println!("Found two strings that differ by only one character: {} {}",seen_tag, box_tag);
                // this loop is also O(N**2), but N is small and will only happen once, so is not SO bad.
                let mut accumulator = String::new();
                for char_b in box_tag.chars() {
                    for char_s in seen_tag.chars() {
                        if char_b == char_s { accumulator.push(char_b); break }
                    }
                }
                println!("string of shared chars {}", accumulator)
            }
        }
        &seen.push(box_tag.to_string());
    }
}