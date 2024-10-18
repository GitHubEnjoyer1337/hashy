use std::env;
use sha2::{Digest, Sha256};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "-a" {

        let config = Config::build(&args[2], &args[3]).unwrap();

        let hash_string = hashappend_loop(config.to_hash, config.count);

        println!("{}", hash_string);
    } else if args[1] == "-b" {

        let config = Config::build(&args[2], &args[3]).unwrap();

        let hash_string = app_hash_to_string(config.to_hash, config.count);
        
        println!("{}", hash_string);
    } 
    else {
        let config = Config::build(&args[1], &args[2]).unwrap();

        let hash_string = hashio_loop(config.to_hash, config.count);

        println!("{}", hash_string);
    }
}




struct Config<'a> {
    to_hash: &'a str,
    count: usize,
}





impl<'a> Config<'a> {
    fn build(to_hash: &'a str, count_str: &str) -> Result<Config<'a>, String> {
        let count = match count_str.parse() {
            Ok(num) => num,
            Err(_) => return Err("second arg must be valid (usize)".to_string()),
        };
        Ok(Config {to_hash, count})
    }
}



// appends input count times then hashes 
fn hashappend_loop(input: &str, count: usize) -> String {
    let mut hasher = Sha256::new();
    let mut inputstr = input.to_string();
    inputstr.push_str(&input.repeat(count as usize));
    hasher.update(&inputstr);
    let result = hasher.finalize();

    format!("{:x}", result)
}





// uses last outputhash as input for next and does this count times
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



// appends each hash to outputstring
fn app_hash_to_string (input: &str, count: usize) -> String {
    let mut output = Sha256::new().chain_update(input).finalize();
    let mut result = format!("{:x}", output);
    for _ in 0..count {
        output = Sha256::new().chain_update(&output).finalize();
        result.push_str(&format!("{:x}", output));
    }
    result
}





























