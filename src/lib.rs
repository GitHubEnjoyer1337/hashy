use std::collections::HashMap;
use sha2::{Digest, Sha256};
use config::Config;

pub mod config;

type HashFunction = fn(Config) -> String;



pub fn run(args: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
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





























