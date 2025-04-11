use core::str;
use core::str::Utf8Error;

use libslug::slugcrypt::internals::digest::blake2::SlugBlake2sHasher;
use subtle_encoding::hex;
use base32ct::Base32Unpadded;
use base58::ToBase58;

use fixedstr::str64;


// Standards:
// HEX
// BASE58
// Base32

pub struct KeserStandard;

impl KeserStandard {
    pub fn from_bytes(bytes: &[u8]) -> [u8;28] {
        let mut fixed = [0u8;28];
        let hasher = SlugBlake2sHasher::new(28);
        let hash = hasher.hash(bytes);

        if hash.len() == 28 {
            fixed.copy_from_slice(&hash);
        } else {
            panic!("Hash length mismatch");
        }
        return fixed
    }
    pub fn from_output_hex(bytes: &[u8]) -> fixedstr::str64 {
        let hash = hex::encode(bytes);
        let output = core::str::from_utf8(&hash).unwrap();
        return fixedstr::str64::from(output)
    }
    pub fn from_output_hex_upper(bytes: &[u8]) -> fixedstr::str64 {
        let hash = hex::encode_upper(bytes);
        let output = core::str::from_utf8(&hash).unwrap();
        return fixedstr::str64::from(output)
    }
}

#[test]
fn create_keser_standard() {
    let bytes = b"Hello, World!";
    let keser = KeserStandard::from_bytes(bytes);
    let hex_output = KeserStandard::from_output_hex_upper(&keser);

    println!("Keser Standard: {}", hex_output);
}