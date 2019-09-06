use std::fs;

fn main() {
    let data = fs::read_to_string("input")
        .expect("Unable to read file");
    let data_iter = data.split("\n");
    let ff: i32 = data_iter.map(|numstr| numstr.trim().parse::<i32>())
                           .map(|freq| if let Ok(f) = freq {println!("freq change is {}", f); f} else {0})
                           .sum();
    println!("Final Frequency is {}", ff);
}