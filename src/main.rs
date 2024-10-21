use std::env;
use testrust::run;
use testrust::config::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    match run(args) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    let str: &str = "test";
    let count: &str = "32";

    let mexstruct = Config::build(None, str, count);

    println!("{:?}", mexstruct);
}

