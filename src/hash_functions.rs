use crate::config::Config;




use sha2::{Digest, Sha256};




// appends input count times then hashes 
pub fn stringapphash(config: Config) -> String {
    let mut hasher = Sha256::new();
    let mut inputstr = config.to_hash.to_string();
    inputstr.push_str(&config.to_hash.repeat(config.count as usize));
    hasher.update(&inputstr);
    let result = hasher.finalize();

    format!("{:x}", result)
}





// uses last outputhash as input for next and does this count times
pub fn default_hashoi(config: Config) -> String {

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
pub fn apphasho (config: Config) -> String {
    let mut output = Sha256::new().chain_update(config.to_hash).finalize();
    let mut result = format!("{:x}", output);
    for _ in 0..config.count {
        output = Sha256::new().chain_update(&output).finalize();
        result.push_str(&format!("{:x}", output));
    }
    result
}
