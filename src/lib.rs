//      How to use it:
//
//      fn default_hashoi:
//
//      <program> <to_hash> <count>
//      hashes the <to_hash> and then creates a chain of hashes, where
//      the output of the last hash is used as the input of the next
//      and does this <count> times
//
//      
//
//
//      fn stringapphash:
//
//      <program> -a <to_hash> <count>
//      appends <to_hash> <count> times to itself and then hashes the whole string
//
//      
//
//
//      fn apphasho:
//
//      <program> -b <to_hash> <count>
//      creates a string containing <count> hashes which have been created
//      in a chained manner (output = input) and appended to the string
//      and prints that
//
//
//
//      
//      fn query_hashoi:
//
//      <program> -s <query> <to_hash> <count>
//      cycles through each hash created in a chained manner (output = input)
//      and checks if the <query> is contained, if it is contained it prints
//      the hash and the number of the iteration
//
//
//
//
//
//      fn hashfind_start_end:
//
//      <program> -c <to_hash> <count> <hash_start> <hash_end>
//      cycles through each hash created in a chained manner (output = input)
//      and checks if the the start of the hash matched hash_start and if the end
//      of the hash matched hash_end, if found prints the result and the count for the 
//      generated hash
//




pub mod config;
pub mod hash_functions;


use crate::config::config_impl::*;
use hash_functions::query_hashoi;

#[derive(Debug)]
pub enum HashResult {
    StringResult(String),
    TupleResult(String, usize),
    KeyResult {
        address: String,
        private_key: String,
        count: usize,
    },
    Temp(usize),
}



pub fn run(args: Vec<String>) -> Result<HashResult, Box<dyn std::error::Error>> {
    if args.len() < 3 {
        return Err("Usage: <program> [flag] <to_hash> <count>".into());
    }
    let config= Config::build(&args)?;
    println!("{:?}", &config);

    let hash_result = match config.flag {
        //sha256 flags
        Some(Flag::A) => config.stringapphash().to_hex(),
        Some(Flag::B) => config.apphasho().to_hex(),
        Some(Flag::S) => query_hashoi(config),
        //btc flags
        Some(Flag::AB) => config.stringapphash().to_btc(),
        Some(Flag::BB) => config.apphasho().to_btc(),
        Some(Flag::SB) => query_hashoi(config),
        //sol flags
        Some(Flag::AS) => config.stringapphash().to_sol(),
        Some(Flag::BS) => config.apphasho().to_sol(),
        Some(Flag::SS) => query_hashoi(config),
        _ => config.default_hashoi().to_hex(),
    };
    
    Ok(hash_result)
}









