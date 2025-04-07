use std::env;
use std::fs;
use std::process;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("In file {}", config.file_path);
    let content = fs::read_to_string(config.file_path).expect("The arguments should be a file path");
    println!("file content, {}", content);
}

struct Config {
    pub file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}
