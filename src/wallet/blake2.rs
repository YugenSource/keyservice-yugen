use core::str;

use libslug::slugcrypt::internals::digest::blake2::SlugBlake2sHasher;
use subtle_encoding::hex;
use base32ct::Base32Unpadded;
use base58::ToBase58;

// Standards:
// HEX
// BASE58
// Base32

pub struct Standard;

impl Standard {
    pub fn from_bytes(bytes: &[u8]) -> Vec<u8> {
        let hasher = SlugBlake2sHasher::new(224);
        let hash = hasher.hash(bytes);
        return hash
    }
    pub fn from_str(s: &str) -> Vec<u8> {
        let hasher = SlugBlake2sHasher::new(224);
        let hash = hasher.hash(s.as_bytes());
        return hash
    }
    pub fn from_output_hex(bytes: &[u8]) -> Result<String, std::string::FromUtf8Error> {
        let hash = hex::encode(bytes);
        String::from_utf8(hash)?
    }
    pub fn from_output_hex_upper(bytes: &[u8]) -> Result<String,std::string::FromUtf8Error> {
        let hash = hex::encode_upper(bytes);
        String::from_utf8(hash)?
    }
}