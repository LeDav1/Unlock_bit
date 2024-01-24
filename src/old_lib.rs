mod enc;

use std::{error::Error, path::Path};

use crate::enc::{encrypt_aes, decrypt_aes};

// I/O functions and other things that might fail should return a Result<T, E> in their signatures.
// Please follow https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html


// Config struct that will hold the values that are parsed from the command line arguments
pub struct Config {
    pub command: String,
    pub file_path: String,
}

// Config::build function that will handle creating an instance of Config. 
// We return a Result value that will contain a Config instance 
// in the successful case and will return &'static str in the error case.. 
// We’re also going to change the function name from new to build because 
// many programmers expect new functions to never fail
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let command = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { command, file_path })
    }
}

// function named run that will hold all the logic currently in the main function 
// that isn’t involved with setting up configuration or handling errors.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let path = Path::new(config.file_path.as_str());
    assert!(path.exists()); // If assertion is raised, program stops
    
    println!("Welcome to UnlockBit, we will make sure to encrypt your data and (hopefully) retrieve it");
    println!("You specified {} command", config.command);
    println!("Output\n");

    //known values : to change later
    let key = "9311d97f2dd4e73fea8e60d3e1130c4cdc611e47261e498e2de6647bc477d1f7";
    let nonce = "301a35dbfce88096d1fbf0b7";

    match config.command.as_str(){ 
        "encrypt"=> {
            if let Err(err) = encrypt_aes(&config.file_path) {
                eprintln!("Error: {}", err);
            } else {
                println!("File encrypted successfully!");
            }
        }
        "decrypt"=> {
            if let Err(err) = decrypt_aes(&config.file_path, key, nonce) {
                eprintln!("Error: {}", err);
            } else {
                println!("File decrypted successfully!");
            }
        }
        &_ => todo!()
    };

    Ok(())
}
