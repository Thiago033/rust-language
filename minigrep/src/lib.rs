use std::{
    fs, 
    error::Error,
    env
};

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config { query, file_name, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_name)?;

    let results: Vec<&str> = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    println!("Result: ");

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Test";
        let contents = 
        "\
            Test: this is a test
            safe, fast, productive.
            trust theree.
        ";

        assert_eq!(vec!["Test: this is a test"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "tEsT";
        let contents = 
        "\
            Test: this is a test
            safe, fast, productive.
            trust theree.
        ";

        assert_eq!(vec!["Test: this is a test"], search_case_insensitive(query, contents));
    }
}
