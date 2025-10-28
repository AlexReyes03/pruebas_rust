use anyhow::{Context, Result};
use ed25519_dalek::{PublicKey, SecretKey};
use sha2::{Sha256, Digest};

pub fn encode_stellar_public(key: &PublicKey) -> String {
    let version_byte: u8 = 6 << 3;
    let mut data = vec![version_byte];
    data.extend_from_slice(key.as_bytes());
    
    let checksum = calculate_checksum(&data);
    data.extend_from_slice(&checksum);
    
    let encoded = base32::encode(base32::Alphabet::RFC4648 { padding: false }, &data);
    format!("G{}", encoded)
}

pub fn encode_stellar_secret(key: &SecretKey) -> String {
    let version_byte: u8 = 18 << 3;
    let mut data = vec![version_byte];
    data.extend_from_slice(key.as_bytes());
    
    let checksum = calculate_checksum(&data);
    data.extend_from_slice(&checksum);
    
    let encoded = base32::encode(base32::Alphabet::RFC4648 { padding: false }, &data);
    format!("S{}", encoded)
}

pub fn calculate_checksum(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let first_hash = hasher.finalize();
    
    let mut hasher = Sha256::new();
    hasher.update(&first_hash);
    let second_hash = hasher.finalize();
    
    second_hash[..2].to_vec()
}

pub fn mask_string(s: &str, visible_chars: usize) -> String {
    if s.len() <= visible_chars {
        return "*".repeat(s.len());
    }
    
    let last_chars = &s[s.len() - visible_chars..];
    format!("{}{}",  "*".repeat(s.len() - visible_chars), last_chars)
}

pub fn validate_stellar_address(address: &str) -> Result<bool> {
    if !address.starts_with('G') {
        return Ok(false);
    }
    
    if address.len() != 56 {
        return Ok(false);
    }
    
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_string() {
        assert_eq!(mask_string("1234567890", 4), "******7890");
        assert_eq!(mask_string("123", 4), "***");
    }

    #[test]
    fn test_validate_stellar_address() {
        assert!(validate_stellar_address("GABC...").is_ok());
        assert!(!validate_stellar_address("SABC...").unwrap());
    }
}