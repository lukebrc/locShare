extern crate hex;
use hex::FromHex;
use rand::{Rng, random};
use crate::{connection_process, crypto};
use crypto::{aes128_encrypt, aes128_decrypt};
use connection_process::ConnectionProcess;

pub struct CryptoNode {
  pub sym: Vec<u8>,
  pub ric: Vec<u8>,
  pub g: Vec<u8>,
}

impl CryptoNode {

  pub fn generate() -> CryptoNode {
    //let keys = Rsa::generate(2048).unwrap();
    return CryptoNode{
      //rsa_keys: keys,
      sym: Vec::new(),
      ric: Vec::new(),
      g: Vec::new(),
    }
  }

  // pub const fn new(rsa: Rsa<Private>) -> CryptoNode {
  //   return CryptoNode{
  //     rsa_keys: rsa,
  //     sym: 0,
  //     ric: Vec::new(),
  //     g: Vec::new(),
  //   }
  // }

  // pub fn get_pub_key_pem(&self) -> String {
  //   let pub_key: Vec<u8> = self.rsa_keys.public_key_to_pem().unwrap();
  //   return String::from_utf8(pub_key)
  //     .expect("invalid PEM format");
  // }

  // pub fn get_pub_key_der(&self) -> Vec<u8> {
  //   return self.rsa_keys.public_key_to_der().unwrap();
  // }

  // return DER-formed public key encrypted symmetrically with ic (invitation code)
  // pub fn get_encrypted_pub_key_der(&self, ic: &[u8]) -> Vec<u8> {
  //   let pub_key: Vec<u8> = self.rsa_keys.public_key_to_der().unwrap();
  //   return self.aes_encrypt(pub_key, ic);
  // }

  pub fn generate_random_invitation_code(&self) -> Vec<u8> {
    let random_bytes: Vec<u8> = (0..16).map(|_| { rand::random::<u8>() }).collect();
    return random_bytes;
  }

  pub fn draw_ephemeral_key(&self) -> Vec<u8> {
    let random_bytes: Vec<u8> = (0..16).map(|_| { rand::random::<u8>() }).collect();
    return random_bytes;
  }

  pub fn encrypt_ephemeral_key(&self, eph_key: &Vec<u8>, inv_code: &Vec<u8>) -> Vec<u8> {
    return aes128_encrypt(eph_key, inv_code);
  }

  pub fn decrypt_ephmemeral_key(&self, eph_key: &Vec<u8>, inv_code: &Vec<u8>) -> Vec<u8> {
    //TODO: check conn_proc.broadcast_code CRC
    println!("lengts: {}, {}", eph_key.len(), inv_code.len());
    let decrypted = aes128_decrypt(&eph_key, &inv_code);
    return decrypted;
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_draw_and_encrypt_ephemeral_key() {
    let inv_code= b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
    let cnode = CryptoNode::generate();
    let inv_code_vec = inv_code.to_vec();
    let eph_key = cnode.draw_ephemeral_key();
    let enc_eph_key = cnode.encrypt_ephemeral_key(&eph_key, &inv_code.to_vec());
  }
}