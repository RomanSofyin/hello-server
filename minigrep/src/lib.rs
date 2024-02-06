use std::error::Error;
use std::fs;

pub struct Config {
    // ToDo: `query` and `file_path` needs become private later
    pub query: String,
    pub file_path: String,
}

impl Config {
    #[allow(dead_code)]
    pub fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }

    #[allow(dead_code)]
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents =
        fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");
    Ok(())
}

