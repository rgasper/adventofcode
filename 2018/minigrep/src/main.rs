use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
    let conf = Config::new(env::args()).unwrap_or_else(|err| {
                eprintln!("Error parsing arguments: {}", err);
                process::exit(1);
        });
    println!("looking for pattern \"{}\" in file \"{}\"", conf.query, conf.filename);
    
    if let Err(e) = run(conf) {
        eprintln!("Application Error {}", e);
        process::exit(1);
    };
}
