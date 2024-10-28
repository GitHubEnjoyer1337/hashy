use std::env;
use testrust::run;


fn main() {
    let args: Vec<String> = env::args().collect();

    match run(args) {
        Ok(result) => println!("{:?}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

