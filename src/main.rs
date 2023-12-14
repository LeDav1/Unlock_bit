use std::env;
use std::process;

use unlock_bit::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Returns command line arguments in a config structure.
    // Uses unwrap_or_else, which is defined on Result<T, E> by the standard library. 
    // Using unwrap_or_else allows us to define some custom, non-panic! error handling. 
    // If the Result is an Ok value, this method returns the inner value Ok. 
    // However, if the value is an Err value unwrap_or_else will pass the inner value of the Err, 
    // which in this case is the static string "not enough arguments"
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = unlock_bit::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
        
    }

}

