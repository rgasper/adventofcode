use std::fs;
use std::collections::HashSet;

fn main() {
    let mut frequencies: HashSet<i32> = HashSet::new();
    let mut frequency: i32 = 0;
    frequencies.insert(frequency);
    let mut flag: bool = false; // using a flag is ugly but it does work
    loop {
        let data = fs::read_to_string("input")
            .expect("Unable to read file");
        for change in data.split("\n") {
            let num_chg: i32 = match change.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
                };
            frequency += num_chg;      
            println!("frequency is now {}", frequency);
            if frequencies.contains(&frequency) {
                println!("First frequency encountered twice is {}", frequency);
                flag = true;
                break;
            } else {
                frequencies.insert(frequency);
            }
        }
        if flag {
            break;
        }
    }
}