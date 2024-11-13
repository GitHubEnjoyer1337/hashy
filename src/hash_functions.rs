use crate::HashResult;
use crate::config::Config;
use sha2::{digest::Digest, Sha256};
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey as SecpPublicKey};
use bitcoin::{PrivateKey, PublicKey, Network};
use bitcoin::util::address::Address;




// appends input count times then hashes 
pub fn stringapphash(config: Config) -> HashResult {
    let mut hasher = Sha256::new();
    let inputstr = config.to_hash.repeat(config.count + 1);
    hasher.update(&inputstr);
    let result = hasher.finalize();

    let secp = Secp256k1::new();
    let private_key_bytes = result.as_slice();

    let secret_key = match SecretKey::from_slice(private_key_bytes) {
        Ok(sk) => sk,
        Err(_) => return HashResult::StringResult("Invalid private key generated.".to_string()),
    };

    let secp_public_key = SecpPublicKey::from_secret_key(&secp, &secret_key);

    let private_key = PrivateKey {
        compressed: true,
        network: Network::Bitcoin,
        inner: secret_key,
    };

    let public_key = PublicKey {
        compressed: true,
        inner: secp_public_key,
    };

    let address = Address::p2pkh(&public_key, Network::Bitcoin);

    let private_key_wif = private_key.to_wif();

    HashResult::KeyResult {
        address: address.to_string(),
        private_key: private_key_wif,
    }
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




pub fn hashfind_start_end (config: Config) -> HashResult {
    let start = config.hash_start;
    let end = config.hash_end;
    let count = config.count;

    let mut hasher = Sha256::new();
    hasher.update(config.to_hash.as_bytes());

    for i in 0..count {
        let result = hasher.clone().finalize();
        let hash_string = format!("{:x}", result);

        if let Some(s) = start.as_ref() {
            if let Some(e) = end.as_ref() {
                if hash_string.starts_with(s) && hash_string.ends_with(e) {
                    return HashResult::TupleResult(hash_string, i);
                }
            }
        }

        hasher = Sha256::new();
        hasher.update(hash_string.as_bytes());
    }

    HashResult::StringResult(String::from("No Match"))
}




















