use std::env;
use sha2::{Digest, Sha256};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (to_hash, times) = parse_args(&args);
    let hash_string = hash_loop(to_hash, times);

    println!("the hash is: {}", hash_string);
}

//struct Config {
//    to_hash: String,
//    count: i32,
//}

fn parse_args(args: &[String]) -> (&str, i32) {
    let to_hash = &args[1];
    let times: i32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("second arg must be valid int");
            std::process::exit(1);
        }
    };
//    Config {
//        to_hash,
//        count: times,
//    }
    (to_hash, times)
}

fn hash_loop(input: &str, count: i32) -> String {

    let mut hasher = Sha256::new();
    hasher.update(input);

    for _ in 0..count {
        let result = hasher.finalize_reset();
        hasher.update(result);
    }
    let result = hasher.finalize();

    format!("{:x}", result)
}
