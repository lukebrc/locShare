extern crate openssl;
use openssl::aes::AesKey;
use openssl::symm::{encrypt, decrypt, Cipher};
use openssl::{hash::MessageDigest, pkcs5::pbkdf2_hmac};
extern crate hex;
use rand::{Rng, random};
use std::io::{Result, Error, ErrorKind};
  
pub const CIPHER_LEN: usize = 32;
const HMAC_ITER: usize = 1024;
pub type EResult<T> = std::result::Result<T, Box<dyn std::error::Error> >;

pub fn aes128_encrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
  let cipher = Cipher::aes_128_cbc(); //todo: cipher dependent of key size
  let encrypted = encrypt(cipher, key, Some(key), input).unwrap(); //TODO: currently iv=key, change it
  return encrypted;
}

pub fn aes128_decrypt(input: &Vec<u8>, key: &[u8]) -> Vec<u8> {
  let cipher = Cipher::aes_128_cbc(); //todo: cipher dependent of key size
  return decrypt(cipher, key, Some(key), &input).unwrap(); //TODO: currently iv=key, change it
}

pub fn encrypt_sym_key(pin: &str, sym_key: &[u8]) -> Result<[u8; CIPHER_LEN + 8]> {
  let iv: [u8; 8] = [7; 8];
  let mut key = [0; CIPHER_LEN];
  pbkdf2_hmac(pin.as_bytes(), &iv, HMAC_ITER, MessageDigest::sha256(), &mut key)?;
  let ak: AesKey = AesKey::new_encrypt(&key).unwrap();
  let mut enc_buf = [0; CIPHER_LEN + 8];
  openssl::aes::wrap_key(&ak, Some(iv), &mut enc_buf, &sym_key)
    .or(Err(Error::new(ErrorKind::InvalidData, "Could not wrap key")))?;
  Ok(enc_buf)
}

pub fn decrypt_sym_key(pin: &str, sym_key: &[u8]) -> Result<[u8; 32]> {
  let iv: [u8; 8] = [7; 8];
  let mut key = [0; CIPHER_LEN];
  pbkdf2_hmac(pin.as_bytes(), &iv, HMAC_ITER, MessageDigest::sha256(), &mut key)?;

  let ak: AesKey = AesKey::new_decrypt(&key).unwrap();
  let mut dec_buf = [0; CIPHER_LEN];
  openssl::aes::unwrap_key(&ak, Some(iv), &mut dec_buf, sym_key)
    .or(Err(Error::new(ErrorKind::InvalidData, "Could not decipher sym key")))?;
  Ok(dec_buf)
}

pub fn compute_mac(eph_key: &[u8]) -> Vec<u8> {
  let iv: [u8; 8] = [7; 8];
  let mut mac = [0u8; CIPHER_LEN];
  pbkdf2_hmac(&eph_key, &iv, HMAC_ITER, MessageDigest::sha256(), &mut mac).unwrap();
  mac.to_vec()
}

pub fn encrypt_msg(msg: &Vec<u8>, key: &[u8]) -> EResult<Vec<u8>> {
  let key_hash = openssl::hash::hash(MessageDigest::sha256(), &key).unwrap();
  let enc = openssl::symm::encrypt(Cipher::aes_256_cbc(), &key_hash, None, msg);
  match enc {
    Ok(e) => Ok(e),
    Err(msg) => Err(msg.to_string().into())
  }
}

pub fn decrypt_msg(msg: &Vec<u8>, key: &[u8]) -> std::result::Result<Vec<u8>, Error> {
  let key_hash = openssl::hash::hash(MessageDigest::sha256(), &key).unwrap();
  let dec = openssl::symm::decrypt(Cipher::aes_256_cbc(), &key_hash, None, msg);
  match dec {
    Ok(e) => Ok(e),
    Err(msg) => Err(Error::new(ErrorKind::Other, msg.to_string()))
  }
}

pub fn random_bytes(n: u16) -> Vec<u8> {
  (0..n).map(|_| { random::<u8>() }).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

//   #[test]
//   fn test_rsa_encrypt_decrypt() {
//     let rsa = Rsa::generate(2048).unwrap();
//     let data = b"my_test data";
//     let mut buf = vec![0; rsa.size() as usize];
//     let encrypted_len = rsa.public_encrypt(data, &mut buf, Padding::PKCS1).unwrap();

//     let mut decrypted = vec![0; encrypted_len];
//     let decrypted_len = rsa.private_decrypt(&buf[0..encrypted_len], &mut decrypted, Padding::PKCS1).unwrap();

//     assert_eq!(decrypted[0..data.len()].to_vec(), data);
//   }

//   #[test]
//   fn test_get_encrypted_pub_key() {

//     let mut cnode = CryptoNode::generate();
//     let ic = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
//     let iv = ic;
//     let enc_pub_key = cnode.get_encrypted_pub_key_der(ic);

//     let decrypted = cnode.aes_decrypt(enc_pub_key, ic);
//     println!("unencrypted ({}), {:?}", decrypted.len(), decrypted);
//     let pub_key = cnode.get_pub_key_der();
//     assert_eq!(pub_key, &decrypted[..]);
//   }

  #[test]
  fn test_aes128_encrypt_decrypt() {
    let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
    //let iv =  b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

    let input = b"hello world";
    println!("input: {:?}", input);

    let encrypted = aes128_encrypt(&input.to_vec(), key);
    println!("encrypted: {:?}", encrypted);

    let decrypted = aes128_decrypt(&encrypted, key); 
    println!("unencrypted ({}), {:?}", decrypted.len(), decrypted);
    assert_eq!(input, &decrypted[..]);
  }

  #[test]
  fn test_aes_encrypt_decrypt2() {
    let key_hex = "12345678901234561234567890123456";
    let key = hex::decode(key_hex).unwrap();
    let mut iv  = *b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
    let mut iv2 = iv.clone();
    let aes_enc_key = AesKey::new_encrypt(&key).unwrap();
    let aes_dec_key = AesKey::new_decrypt(&key).unwrap();
  }
}