
use std::str;

const SOLANA_ADDRESS_LENGTH_BYTES: usize = 32;

pub enum Encoding {
    Base58,
    Base64,
}

pub fn validate(address: &str) -> Result<Encoding, &str> {
    let decoded_base58 = bs58::decode(address).into_vec();
    let decoded_base64 = base64::decode(address);

    let is_base58 = decoded_base58.is_ok();
    let is_base64 = decoded_base64.is_ok();

    if is_base58 && decoded_base58.unwrap().len() == SOLANA_ADDRESS_LENGTH_BYTES {
        return Ok(Encoding::Base58);
    }
    if is_base64 && decoded_base64.unwrap().len() == SOLANA_ADDRESS_LENGTH_BYTES {
        return Ok(Encoding::Base64);
    }
    if is_base58 || is_base64 {
        return Err("Invalid length");
    }
    return Err("Invalid address");
}