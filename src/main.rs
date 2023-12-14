use std::fs;
use std::io;
use std::env;
use std::process;

struct Config {
    command: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let command = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { command, file_path })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let contents = fs::read_to_string(config.file_path)
        .expect("Something went wrong reading the file");

    println!("Welcome to UnlockBit, we will make sure to encrypt your data and (hopefully) retrieve it");
    println!("You specified {} command", config.command);
    println!("With text:\n{}", contents);
}

