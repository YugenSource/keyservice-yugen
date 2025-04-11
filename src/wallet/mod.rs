#![no_std]

/// Wallet KeyService Standards
pub mod standards;

pub trait KeyService {
    fn generate_key(&self) -> String;
    fn generate_key_bip39(&self) -> String;
    fn generate_key_securerand(&self) -> String;
    
    // Sign and Verify
    fn sign_message(&self, message: &str) -> String;
    fn verify_signature(&self, message: &str, signature: &str) -> bool;
    
    // KeyService Address
    fn keyservice_addr(&self, standard: u8) -> String;
}

pub trait KeyServiceEncrypt {
    fn encrypt_message(&self, message: &str) -> String;
    fn decrypt_message(&self, encrypted_message: &str) -> String;
}

/// Keypairs:
/// 
/// General
/// - Ed25519
/// - ECDSA
/// - Secp256k1
/// 
/// More Advanced
/// - Ed448
/// - P521
/// 
/// Specialized
/// - RSA
/// - BLS
/// - Schnorr
/// 
/// PQC
/// - SPHINCS+
/// - Falcon
/// - Dilithium
/// 
/// Hybrid
/// 1. SPHINCS+ with Ed25519
/// 


/// KeyService Address Standard:
/// 00: From Bytes Using BLAKE2s-224
/// 01: From Bytes Using SHA3-224
/// 
/// 08: From Bytes Using BLAKE3 (256 bits)

pub struct KeSerWallet {

}