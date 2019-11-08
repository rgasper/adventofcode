//! Minigrep
//! a limited text search tool
use std::error::Error;
use std::fs;
use std::env;

/// Contains configuration arguments
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    /// build a new Config
    pub fn new(mut args : std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no query string found in args")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("no query string found in args")
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Ok( Config{query, filename, case_sensitive} )
    }
}

/// case-sensitive search for query in lines of contents
/// # Examples
/// ```
/// use minigrep::search;
/// let a = "hello
/// world
/// HEAT
/// hotdog
/// hey";
/// let b = vec!["hello","hey"];
/// assert_eq!(search("he",a), b);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
            .filter(|line| line.contains(query))
            .collect()
}

/// case-insensitive search for query in lines of contents
/// # Examples
/// ```
/// use minigrep::search_case_insensitive;
/// let a = "hello
/// world
/// HEAT
/// hotdog
/// hey";
/// let b = vec!["hello", "HEAT", "hey"];
/// assert_eq!(search_case_insensitive("he",a), b);
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lcase_query = query.to_lowercase();
    contents.lines()
            .filter(|line| line.to_lowercase().contains(&lcase_query))
            .collect()
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for res_line in results {
        println!("{}", res_line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}