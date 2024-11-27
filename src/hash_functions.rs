use crate::{HashResult, Flag};
use crate::config::Config;
use sha2::{digest::Digest, Sha256};
use bitcoin::secp256k1::{Secp256k1, SecretKey as SecretKeyBtc, PublicKey as SecpPublicKey};
use bitcoin::{PrivateKey, PublicKey, Network};
use bitcoin::util::address::Address;
use solana_sdk::signer::{keypair::Keypair, Signer};
use ed25519_dalek::{SigningKey, VerifyingKey};
use bs58;

pub fn sha256_to_solana(
    result: sha2::digest::Output<Sha256>,
    count: usize,
) -> HashResult {
    // Create a 64-byte keypair buffer
    let mut keypair_bytes = [0u8; 64];
    // Create a 32-byte array for the secret key
    let mut secret_bytes: [u8; 32] = [0u8; 32];
    secret_bytes.copy_from_slice(result.as_slice());
    
    // Create signing key and derive verifying key
    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let verifying_key = VerifyingKey::from(&signing_key);
    
    // Fill keypair bytes
    keypair_bytes[..32].copy_from_slice(&secret_bytes);
    keypair_bytes[32..].copy_from_slice(verifying_key.as_bytes());

    // Create Solana keypair
    let keypair = match Keypair::from_bytes(&keypair_bytes) {
        Ok(kp) => kp,
        Err(_) => return HashResult::StringResult("Invalid keypair generated.".to_string()),
    };

    let public_key = keypair.pubkey();
    let private_key_b58 = bs58::encode(keypair.to_bytes()).into_string();

    HashResult::KeyResult {
        address: public_key.to_string(),
        private_key: private_key_b58,
        count,
    }
}



//pub fn sha256_to_solana(
//    result: sha2::digest::Output<Sha256>,
//    count: usize,
//) -> HashResult {
//    // Convert the 32-byte hash to a 64-byte keypair buffer
//    let mut keypair_bytes = [0u8; 64];
//    keypair_bytes[..32].copy_from_slice(result.as_slice());
//    // Create a keypair from the bytes
//    let keypair = match Keypair::from_bytes(&keypair_bytes) {
//        Ok(kp) => kp,
//        Err(_) => return HashResult::StringResult("Invalid keypair generated.".to_string()),
//    };
//
//    // Get the public key (address)
//    let public_key = keypair.pubkey();
//
//    // Convert the keypair to bytes and then to base58
//    let private_key_b58 = bs58::encode(keypair.to_bytes()).into_string();
//
////    println!("after keypair");
//    HashResult::KeyResult {
//        address: public_key.to_string(),
//        private_key: private_key_b58,
//        count,
//    }
//}





pub fn sha256_to_btc( result: sha2::digest::Output<Sha256>, count: usize ) -> HashResult {
    let secp = Secp256k1::new();
    let private_key_bytes = result.as_slice();

    let secret_key = match SecretKeyBtc::from_slice(private_key_bytes) {
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
        count,
    }
}








pub fn query_hashoi (config: Config) -> HashResult {
    let query = config.query;
    let maxcount = config.count;
    let mut hasher = Sha256::new();
    hasher.update(config.to_hash.as_bytes());

    for i in 0..maxcount {
        let result = hasher.clone().finalize();
        let mut btc_data: HashResult = HashResult::Temp(0);
        if config.flag == Some(Flag::SB) {
            btc_data = sha256_to_btc(result, i);
        } else if config.flag == Some(Flag::SS) {
            btc_data = sha256_to_solana(result, i);
        } else {
            btc_data = sha256_to_btc(result, i);
        }

            if let HashResult::KeyResult { ref address, ..} = btc_data {
                println!("{}", address);
                let hash_string = format!("{}", address);

                if let Some(q) = query.as_ref() {
                    if hash_string.contains(q) {
                        return btc_data;
                    }
                }
            
                hasher = Sha256::new();
                hasher.update(hash_string.as_bytes());
        }
    }

    HashResult::StringResult(String::from("No Match"))
}





pub fn hashfind_start_end(config: Config) -> HashResult {
    let start = config.hash_start;
    let end = config.hash_end;
    let count = config.count;

    let mut hasher = Sha256::new();
    hasher.update(config.to_hash.as_bytes());

    for i in 0..count {
        let result = hasher.clone().finalize();
        let mut btc_data: HashResult = HashResult::Temp(0);
        
        if config.flag == Some(Flag::SB) {
            btc_data = sha256_to_btc(result, i);
        } else if config.flag == Some(Flag::SS) {
            btc_data = sha256_to_solana(result, i);
        } else {
            btc_data = sha256_to_btc(result, i);
        }

        if let HashResult::KeyResult { ref address, .. } = btc_data {
            println!("{}", address);
            let hash_string = address;

            if let Some(s) = start.as_ref() {
                if let Some(e) = end.as_ref() {
                    if hash_string.starts_with(s) && hash_string.ends_with(e) {
                        return btc_data;
                    }
                }
            }

            hasher = Sha256::new();
            hasher.update(hash_string.as_bytes());
        }
    }

    HashResult::StringResult(String::from("No Match"))
}


//pub fn hashfind_start_end (config: Config) -> HashResult {
//    let start = config.hash_start;
//    let end = config.hash_end;
//    let count = config.count;
//
//    let mut hasher = Sha256::new();
//    hasher.update(config.to_hash.as_bytes());
//
//    for i in 0..count {
//        let result = hasher.clone().finalize();
//        let btc_data = sha256_to_btc(result, i);
//        if let HashResult::KeyResult { ref address, .. } = btc_data {
//            let hash_string = address;
//
//            if let Some(s) = start.as_ref() {
//                if let Some(e) = end.as_ref() {
//                    if hash_string.starts_with(s) && hash_string.ends_with(e) {
//                        return btc_data;
//                        //return HashResult::TupleResult(hash_string, i);
//                    }
//                }
//            }
//
//            hasher = Sha256::new();
//            hasher.update(hash_string.as_bytes());
//        }
//    }
//
//    HashResult::StringResult(String::from("No Match"))
//}




















