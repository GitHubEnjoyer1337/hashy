//      How to use it:
//
//      <program> <to_hash> <count>
//      hashes the <to_hash> and then creates a chain of hashes, where
//      the output of the last hash is used as the input of the next
//      and does this <count> times
//
//      
//
//
//      <program> -a <to_hash> <count>
//      appends <to_hash> <count> times to itself and then hashes the whole string
//
//      
//
//
//      <program> -b <to_hash> <count>
//      creates a string containing <count> hashes which have been created
//      in a chained manner (output = input) and appended to the string
//      and prints that
//
//
//
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






use crate::config::Config;
use crate::HashResult;




use sha2::{Digest, Sha256};




// appends input count times then hashes 
pub fn stringapphash(config: Config) -> HashResult {
    let mut hasher = Sha256::new();
    let mut inputstr = config.to_hash.to_string();
    inputstr.push_str(&config.to_hash.repeat(config.count as usize));
    hasher.update(&inputstr);
    let result = hasher.finalize();

    let formattted_result = format!("{:x}", result);
    HashResult::StringResult(formattted_result)
}





// uses last outputhash as input for next and does this count times
pub fn default_hashoi(config: Config) -> HashResult {

    let mut hasher = Sha256::new();
    hasher.update(config.to_hash);

    for _ in 0..config.count {
        let result = hasher.finalize_reset();
        hasher.update(result);
    }
    let result = hasher.finalize();

    let formattted_result = format!("{:x}", result);
    HashResult::StringResult(formattted_result)
}



// appends each hash to outputstring
pub fn apphasho (config: Config) -> HashResult {
    let mut output = Sha256::new().chain_update(config.to_hash).finalize();
    let mut result = format!("{:x}", output);
    for _ in 0..config.count {
        output = Sha256::new().chain_update(&output).finalize();
        result.push_str(&format!("{:x}", output));
    }
    HashResult::StringResult(result)
}






pub fn query_hashoi (config: Config) -> HashResult {
    let query = config.query;
    let maxcount = config.count;
    let mut hasher = Sha256::new();
    hasher.update(config.to_hash.as_bytes());

    for i in 0..maxcount {
        let result = hasher.clone().finalize();
        let hash_string = format!("{:x}", result);

        if let Some(q) = query.as_ref() {
            if hash_string.contains(q) {
                return HashResult::TupleResult(hash_string, i);
            }
        }
        
        hasher = Sha256::new();
        hasher.update(hash_string.as_bytes());
    }

    HashResult::StringResult(String::from("No Match"))
}
























