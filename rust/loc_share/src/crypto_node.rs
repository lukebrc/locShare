extern crate openssl;
extern crate hex;
use openssl::rsa::{Rsa, Padding};
use openssl::pkey::Private;
use openssl::aes::{AesKey, aes_ige};
use openssl::symm::Mode;
use openssl::symm::{encrypt, decrypt, Cipher};
use hex::{FromHex, ToHex};

pub type BigInt = i128;

pub struct CryptoNode {
  pub rsa_keys: openssl::rsa::Rsa<Private>,
  pub sym: BigInt,
  pub ric: Vec<u8>,
  pub g: Vec<u8>,
}

impl CryptoNode {

  pub fn generate() -> CryptoNode {
    let keys = Rsa::generate(2048).unwrap();
    return CryptoNode{
      rsa_keys: keys,
      sym: 0,
      ric: Vec::new(),
      g: Vec::new(),
    }
  }

  pub const fn new(rsa: Rsa<Private>) -> CryptoNode {
    return CryptoNode{
      rsa_keys: rsa,
      sym: 0,
      ric: Vec::new(),
      g: Vec::new(),
    }
  }

  pub fn get_pub_key_pem(&self) -> String {
    let pub_key: Vec<u8> = self.rsa_keys.public_key_to_pem().unwrap();
    return String::from_utf8(pub_key)
      .expect("invalid PEM format");
  }

  pub fn get_pub_key_der(&self) -> Vec<u8> {
    return self.rsa_keys.public_key_to_der().unwrap();
  }

  // return DER-formed public key encrypted symmetrically with ic (invitation code)
  pub fn get_encrypted_pub_key_der(&self, ic: &[u8]) -> Vec<u8> {
    let pub_key: Vec<u8> = self.rsa_keys.public_key_to_der().unwrap();
    return self.aes_encrypt(pub_key, ic);
  }

  pub fn generate_random_invitation_code(&self) -> Vec<u8> {
    //TODO: randomize
    let ic_hex = "0123456789ABCDEF";
    return Vec::from_hex(ic_hex).unwrap();
  }

  pub fn generate_public_message(&self, ic: &BigInt) -> BigInt {
    //TODO:
    return 2;
  }

  pub fn generate_encrypted_secret_message(&self, x2: &BigInt) -> BigInt {
    //TODO:
    return 3;
  }

  pub fn generate_dh_keys(&mut self) {
    //TODO:
    self.g = Vec::new();
  }

  pub fn aes_encrypt(&self, input: Vec<u8>, key: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_cbc(); //todo: cipher dependent of key size
    let encrypted = encrypt(cipher, key, Some(key), &input).unwrap(); //TODO: currently iv=key, change it
    return encrypted;
  }

  pub fn aes_decrypt(&self, input: Vec<u8>, key: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_cbc(); //todo: cipher dependent of key size
    return decrypt(cipher, key, Some(key), &input).unwrap(); //TODO: currently iv=key, change it
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rsa_encrypt_decrypt() {
    let rsa = Rsa::generate(2048).unwrap();
    let data = b"my_test data";
    let mut buf = vec![0; rsa.size() as usize];
    let encrypted_len = rsa.public_encrypt(data, &mut buf, Padding::PKCS1).unwrap();

    let mut decrypted = vec![0; encrypted_len];
    let decrypted_len = rsa.private_decrypt(&buf[0..encrypted_len], &mut decrypted, Padding::PKCS1).unwrap();

    assert_eq!(decrypted[0..data.len()].to_vec(), data);
  }

  #[test]
  fn test_get_encrypted_pub_key() {

    let mut cnode = CryptoNode::generate();
    let ic = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
    let iv = ic;
    let enc_pub_key = cnode.get_encrypted_pub_key_der(ic);

    let decrypted = cnode.aes_decrypt(enc_pub_key, ic);
    println!("unencrypted ({}), {:?}", decrypted.len(), decrypted);
    let pub_key = cnode.get_pub_key_der();
    assert_eq!(pub_key, &decrypted[..]);
  }

  #[test]
  fn test_aes_encrypt_decrypt() {
    let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
    let iv =  b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

    let input = b"hello world";
    println!("input: {:?}", input);
    let cipher = Cipher::aes_128_cbc();

    let encrypted = encrypt(cipher, key, Some(iv), input).unwrap();
    println!("encrypted: {:?}", encrypted);

    let decrypted = decrypt(cipher, key, Some(iv), &encrypted).unwrap(); 
    println!("unencrypted ({}), {:?}", decrypted.len(), decrypted);
    assert_eq!(input, &decrypted[..]);
  }

  #[test]
  fn test_aes_encrypt_decrypt2() {
    let key_hex = "12345678901234561234567890123456";
    let key = Vec::from_hex(key_hex).unwrap();
    let mut iv  = *b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
    let mut iv2 = iv.clone();
    let aes_enc_key = AesKey::new_encrypt(&key).unwrap();
    let aes_dec_key = AesKey::new_decrypt(&key).unwrap();

    let input = b"hello!!!hello!!!hello!!!hello!!!";
    let mut output = [0u8; 32];
    aes_ige(input, &mut output, &aes_enc_key, &mut iv, Mode::Encrypt);

    let mut output2 = [0u8; 32];
    aes_ige(&output, &mut output2, &aes_dec_key, &mut iv2, Mode::Decrypt);

    assert_eq!(output2.to_vec(), input.to_vec());
  }

}