use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();
    let query = &args[1];    
    let filename = &args[2];
    println!("looking for pattern \"{}\" in file \"{}\"", query, filename);

    let contents = fs::read_to_string(filename)
                    .expect("cannot read file");
    println!("contents of file:\n {}", contents)
}
