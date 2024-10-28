pub mod config;
pub mod hash_functions;


use std::collections::HashMap;
use config::{Config, Flag};
use hash_functions::*;

#[derive(Debug)]
pub enum HashResult {
    StringResult(String),
    TupleResult(String, usize),
}

type HashFunction = fn(Config) -> HashResult;



pub fn run(args: Vec<String>) -> Result<HashResult, Box<dyn std::error::Error>> {
    if args.len() < 3 {
        return Err("Usage: <program> [flag] <to_hash> <count>".into());
    }
    let config= Config::build(&args)?;
    println!("{:?}", &config);


    let mut flag_functions: HashMap<Flag, HashFunction> = HashMap::new();
    flag_functions.insert(Flag::A, stringapphash);
    flag_functions.insert(Flag::B, apphasho);
    flag_functions.insert(Flag::S, query_hashoi);

    let default_function: HashFunction = default_hashoi;

    let hash_function = config.flag
        .as_ref()
        .and_then(|f| flag_functions.get(f))
        .unwrap_or(&default_function);

    Ok(hash_function(config))
}
