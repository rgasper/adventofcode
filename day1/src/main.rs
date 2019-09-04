use std::fs;

fn main() {
    let data = fs::read_to_string("input")
        .expect("Unable to read file");
    let mut frequency = 0;
    for change in data.split("\n") {
        let num_chg: i32 = match change.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            };
        frequency += num_chg;                          
        println!("num change is {}, frequency is now {}", num_chg, frequency);  
    }
}