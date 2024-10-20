use std::{env, collections::HashMap};
use sha2::{Digest, Sha256};


type HashFunction = fn(Config) -> String;


fn main() {
    let args: Vec<String> = env::args().collect();

    match run(args) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run(args: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    if args.len() < 3 {
        return Err("Usage: <program> [flag] <to_hash> <count>".into());
    }
    let flags: Vec<&str> = vec!["-a", "-b"];
    let config: Config;
    if args.len() >= 4 && flags.contains(&args[1].as_str()) {
        config = Config::build(Some(&args[1]), &args[2], &args[3])?;
    } else {
        config = Config::build(None, &args[1], &args[2])?;
    }


    let mut flag_functions: HashMap<&str, HashFunction> = HashMap::new();
    flag_functions.insert("-a", stringapphash);
    flag_functions.insert("-b", apphasho);

    let default_function: HashFunction = default_hashoi;

    let hash_function = config.flag
        .and_then(|f| flag_functions.get(f))
        .unwrap_or(&default_function);

    Ok(hash_function(config))
}






struct Config<'a> {
    flag: Option<&'a str>,
    to_hash: &'a str,
    count: usize,
}





impl<'a> Config<'a> {
    fn build(
        flag: Option<&'a str>, 
        to_hash: &'a str, 
        count_str: &str
        ) -> Result<Config<'a>, String> {

        let flags: Vec<&str> = vec!["-a", "-b"];

        let flag = if let Some(f) = flag {
            if flags.contains(&f) {
                Some(f)
            } else {
                return Err("invalid flag".to_string());
            }
        } else {
            None
        };

        let count = match count_str.parse() {
            Ok(num) => num,
            Err(_) => return Err("second arg must be valid (usize)".to_string()),
        };
        Ok(Config {flag, to_hash, count})
    }
}



// appends input count times then hashes 
fn stringapphash(config: Config) -> String {
    let mut hasher = Sha256::new();
    let mut inputstr = config.to_hash.to_string();
    inputstr.push_str(&config.to_hash.repeat(config.count as usize));
    hasher.update(&inputstr);
    let result = hasher.finalize();

    format!("{:x}", result)
}





// uses last outputhash as input for next and does this count times
fn default_hashoi(config: Config) -> String {

    let mut hasher = Sha256::new();
    hasher.update(config.to_hash);

    for _ in 0..config.count {
        let result = hasher.finalize_reset();
        hasher.update(result);
    }
    let result = hasher.finalize();

    format!("{:x}", result)
}



// appends each hash to outputstring
fn apphasho (config: Config) -> String {
    let mut output = Sha256::new().chain_update(config.to_hash).finalize();
    let mut result = format!("{:x}", output);
    for _ in 0..config.count {
        output = Sha256::new().chain_update(&output).finalize();
        result.push_str(&format!("{:x}", output));
    }
    result
}





























