pub mod config;
pub mod hash_functions;
pub mod query;


use std::collections::HashMap;
use config::Config;
use hash_functions::*;


type HashFunction = fn(Config) -> String;



pub fn run(args: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    if args.len() < 3 {
        return Err("Usage: <program> [flag] <to_hash> <count>".into());
    }
    let config: Config;
    config = Config::build(&args)?;
    println!("{:?}", &config);


    let mut flag_functions: HashMap<&str, HashFunction> = HashMap::new();
    flag_functions.insert("-a", stringapphash);
    flag_functions.insert("-b", apphasho);

    let default_function: HashFunction = default_hashoi;

    let hash_function = config.flag
        .and_then(|f| flag_functions.get(f))
        .unwrap_or(&default_function);

    Ok(hash_function(config))
}
