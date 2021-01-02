extern crate openssl;
use openssl::rsa::{Rsa, Padding};
use openssl::pkey::Private;
use openssl::aes::{AesKey, aes_ige};
use openssl::symm::Mode;
extern crate hex;
use hex::{FromHex, ToHex};

pub type BigInt = i128;

pub struct CryptoNode {
  pub rsa_keys: openssl::rsa::Rsa<Private>,
  pub sym: BigInt,
  pub ric: BigInt,
  pub g: BigInt,
}

impl CryptoNode {

  pub fn generate() -> CryptoNode {
    let keys = Rsa::generate(2048).unwrap();
    return CryptoNode{
      rsa_keys: keys,
      sym: 0,
      ric: 0,
      g: 0,
    }
  }

  pub const fn new(rsa: Rsa<Private>) -> CryptoNode {
    return CryptoNode{
      rsa_keys: rsa,
      sym: 0,
      ric: 0,
      g: 0,
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

  pub fn generate_random_invitation_code(&self) -> BigInt {
    //TODO:
    return 1;
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
    self.g = 7;
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

    //println!("{:?}", buf[0..encrypted_len]);
    let mut decrypted = vec![0; data.len()];
    let decrypted_len = rsa.private_decrypt(&buf[0..encrypted_len], &mut decrypted, Padding::PKCS1).unwrap();

    assert_eq!(decrypted_len, data.len());
  }

  #[test]
  fn test_aes_encrypt_decrypt() {
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