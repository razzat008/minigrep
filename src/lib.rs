use std::{env, error::Error, fs, process};

pub struct Config {
    pub query: String,     // string to look for
    pub file_path: String, // the file path
}

impl Config {
   pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    // println!("With text:\n{contents}");
    Ok(())
}

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "ast";
        let contents = "\
            Rust:
            safe, fast, productive";
        assert_eq!(vec!["safe","fast","productive"],search(query,contents) );
    }


}
