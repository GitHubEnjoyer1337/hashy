use std::env;
use testrust::run;


fn main() {
    let args: Vec<String> = env::args().collect();

    match run(args) {
        Ok(result) => match result {
            testrust::HashResult::StringResult(string) => println!("{}", string),
            testrust::HashResult::TupleResult(string, number) => println!("{} {}", string, number),
            testrust::HashResult::KeyResult{ address, private_key } => {
                println!("Bitcoin Address: {}", address);
                println!("Private Key (WIF): {}", private_key);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

