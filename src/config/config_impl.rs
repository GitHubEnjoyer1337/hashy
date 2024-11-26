use crate::HashResult;
use bitcoin::hashes::hex::ToHex;
use sha2::digest::Output;
use sha2::{digest::Digest, Sha256};
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey as SecpPublicKey};
use bitcoin::{PrivateKey, PublicKey, Network};
use bitcoin::util::address::Address;
use solana_sdk::{
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
};
use bs58;



#[derive(Debug)]
pub struct Config<'a> {
    pub flag: Option<Flag>,
    pub to_hash: &'a str,
    pub count: usize,
    pub query: Option<&'a str>,
    pub hash_start: Option<&'a str>,
    pub hash_end: Option<&'a str>,
}


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Flag {
    A,
    B,
    C,
    S,
    AB,
    BB,
    CB,
    SB,
    AS,
    BS,
    CS,
    SS,
}


impl Flag {
    fn from_str(s: &str) -> Option<Flag> {
        match s {
            "-a" => Some(Flag::A),
            "-b" => Some(Flag::B),
            "-c" => Some(Flag::C),
            "-s" => Some(Flag::S),
            "-ab" => Some(Flag::AB),
            "-bb" => Some(Flag::BB),
            "-cb" => Some(Flag::CB),
            "-sb" => Some(Flag::SB),
            "-as" => Some(Flag::AS),
            "-bs" => Some(Flag::BS),
            "-cs" => Some(Flag::CS),
            "-ss" => Some(Flag::SS),
            _ => None,
        }
    }
}

pub trait HashOperations {
    fn stringapphash(&self) -> (Output<Sha256>, usize);
    fn default_hashoi(&self) -> (Output<Sha256>, usize);
    fn apphasho(&self) -> (Output<Sha256>, usize);
}

pub trait ToHex1 {
    fn to_hex(self) -> HashResult;
    fn to_btc(self) -> HashResult;
    fn to_sol(self) -> HashResult;
}


impl HashOperations for Config<'_> {
    // appends input count times then hashes 
    fn stringapphash(&self) -> (sha2::digest::Output<Sha256>, usize) {
        let mut hasher = Sha256::new();
        let inputstr = self.to_hash.repeat(self.count + 1);
        hasher.update(&inputstr);
        let result = hasher.finalize();
        (result, self.count)
    }

    // uses last outputhash as input for next and does this count times
    fn default_hashoi(&self) -> (sha2::digest::Output<Sha256>, usize) {
    
        let mut hasher = Sha256::new();
        hasher.update(self.to_hash);
    
        for _ in 0..self.count {
            let result = hasher.finalize_reset();
            hasher.update(result);
        }
        let result = hasher.finalize();
        (result, self.count)
    }



    // appends each hash to outputstring
    fn apphasho(&self) -> (sha2::digest::Output<Sha256>, usize) {
        let mut output = Sha256::new().chain_update(self.to_hash).finalize();
        let mut result = Vec::new();
        result.extend_from_slice(&output);
    
    
        for _ in 0..self.count {
            output = Sha256::new().chain_update(&output).finalize();
            result.extend_from_slice(&output);
        }
        let final_output = Sha256::new().chain_update(&result).finalize();

        (final_output, self.count)
    
    }







}

impl ToHex1 for (Output<Sha256>, usize) {
    fn to_hex(self) -> HashResult {
        let (result, num) = self;
        let result1 = format!("{:x}", result);
        HashResult::TupleResult(result1, num)
    }


    fn to_btc(self) -> HashResult {
        let (result, count) = self;
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
            count,
        }
    }


    fn to_sol(self) -> HashResult {
        let (result, count) = self;
        
        // Convert the 32-byte hash to a 64-byte keypair buffer
        let mut keypair_bytes = [0u8; 64];
        keypair_bytes[..32].copy_from_slice(result.as_slice());
    
        // Create a keypair from the bytes
        let keypair = match Keypair::from_bytes(&keypair_bytes) {
            Ok(kp) => kp,
            Err(_) => return HashResult::StringResult("Invalid keypair generated.".to_string()),
        };
    
        // Get the public key (address)
        let public_key = keypair.pubkey();
    
        // Convert the keypair to bytes and then to base58
        let private_key_b58 = bs58::encode(keypair.to_bytes()).into_string();
    
        HashResult::KeyResult {
            address: public_key.to_string(),
            private_key: private_key_b58,
            count,
        }
    }




}
impl<'a> Config<'a> {
    pub fn build(stringvec: &'a [String]) -> Result<Config<'a>, Box<dyn std::error::Error>> {
        if stringvec.len() < 3 {
            return Err("Usage: <program> [flag] <to_hash> <count>".into());
        }

        let (flag, to_hash, count, query, hash_start, hash_end) = if stringvec.len() >= 4 {
            if let Some(flag) = Flag::from_str(&stringvec[1]) {
                match flag {
                    Flag::C => {
                        if stringvec.len() < 6 {
                            return Err("Not enough args for -c flag".into());
                        }
                        (
                            Some(flag),
                            stringvec[2].as_str(),
                            Self::parse_count(&stringvec[3])?,
                            None,
                            Some(stringvec[4].as_str()),
                            Some(stringvec[5].as_str())
                        )
                    },
                    Flag::S => {
                        if stringvec.len() < 5 {
                            return Err("Not enough args for -s flag".into());
                        }
                        (
                            Some(flag),
                            stringvec[3].as_str(),
                            Self::parse_count(&stringvec[4])?,
                            Some(stringvec[2].as_str()),
                            None,
                            None
                        )
                    },
                    _ => (
                            Some(flag),
                            stringvec[2].as_str(),
                            Self::parse_count(&stringvec[3])?,
                            None,
                            None,
                            None
                         )
                    }
                } else {
                    (
                            None,
                            stringvec[1].as_str(),
                            Self::parse_count(&stringvec[2])?,
                            None,
                            None,
                            None
                    )
                }
        } else {
            (
                            None,
                            stringvec[1].as_str(),
                            Self::parse_count(&stringvec[2])?,
                            None,
                            None,
                            None
            )
        };
        Ok(Config { flag, to_hash, count, query, hash_start, hash_end})
    }



    pub fn parse_count(s: &str) -> Result<usize, Box<dyn std::error::Error>> {
        s.parse().map_err(|_| "count must be valid (usize)".into())
    } 



//    pub fn stringapphash(&self) -> (sha2::digest::Output<Sha256>, usize) {
//        let mut hasher = Sha256::new();
//        let inputstr = self.to_hash.repeat(self.count + 1);
//        hasher.update(&inputstr);
//        let result = hasher.finalize();
//        (result, self.count)
//    }
//    pub fn to_hex(result: sha2::digest::Output<Sha256>, num: usize) -> HashResult {
//        let result1 = format!("{:x}", result);
//        HashResult::TupleResult(result1, num)
//    }




    pub fn to_btc( result: sha2::digest::Output<Sha256>, count: usize ) -> HashResult {
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
            count,
        }
    }












}





















