use core::panic;
use std::env;
use sha2::{Digest, Sha256};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let hash_string = hashio_loop(config.to_hash, config.count);

    println!("the hash is: {}", hash_string);
}

struct Config<'a> {
    to_hash: &'a str,
    count: usize,
}

impl<'a> Config<'a> {
    fn new(args: &'a Vec<String>) -> Result<Config<'a>, &'static str> {
        if args.len() != 3 {
            return Err("Usage: <program> <string> <count>");
        }
        let to_hash = args[1].as_str();
        let count = 
            match args[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("second arg must be valid int (usize)");
                    std::process::exit(1);
                }
            };
        Ok(Config { to_hash, count })
    }
}
// just made this loop in prep for interactive functionality
fn hashappend_loop(input: &str, count: usize) -> String {
    let mut hasher = Sha256::new();
    let mut inputstr = input.to_string();
    inputstr.push_str(&input.repeat(count as usize));
    hasher.update(&inputstr);
    let result = hasher.finalize();

    format!("{:x}", result)
}

fn hashio_loop(input: &str, count: usize) -> String {

    let mut hasher = Sha256::new();
    hasher.update(input);

    for _ in 0..count {
        let result = hasher.finalize_reset();
        hasher.update(result);
    }
    let result = hasher.finalize();

    format!("{:x}", result)
}
