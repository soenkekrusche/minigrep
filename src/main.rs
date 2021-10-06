use std::{env, process};
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config)
}

fn run(config: Config) {
    let contents = fs::read_to_string(&config.filename)
        .expect(format!("Something went wrong with reading file {}", config.filename).as_str());
    println!("With text:\n{}", contents)
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        //TODO: use more efficient way to fix ownership issue than clone. Check chapter 13 rustbook
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}