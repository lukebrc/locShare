extern crate hex;
extern crate dirs;
use std::{fs::File, io::{Read, Write}};
use openssl::{hash::MessageDigest, pkcs5::pbkdf2_hmac};

use openssl::aes::AesKey;
use protobuf::{Message, SpecialFields};
use rand::{Rng, random};
use crate::{connection_process, crypto};
use crypto::{aes128_encrypt, aes128_decrypt};
use std::io::{Result, Error, ErrorKind};
use std::path::PathBuf;
use crate::proto::pb::Config;

const CIPHER_LEN: usize = 32;
const HMAC_ITER: usize = 1024;

pub struct CryptoNode {
  pub sym_key: Vec<u8>,
  pub invitation_code: Vec<u8>,
  pub eph: Vec<u8>,
  //pub g: Vec<u8>,
}

impl CryptoNode {

  pub fn has_config() -> bool {
    match Self::load_config_from_disc() {
      Ok(_) => true,
      Err(_) => false
    }
  }

  pub fn load_from_disc(pin: &str) -> Result<CryptoNode> {
    let config = Self::load_config_from_disc()?;
    let dec_buf = Self::decrypt_sym_key(pin, &config.sym_key)?;

    Ok(CryptoNode {
      sym_key: dec_buf.to_vec(),
      invitation_code: Vec::new(),
      eph: Vec::new()
    })
  }

  pub fn create_new(pin: &str) -> Result<CryptoNode> {
    let mut sym_key = [0; CIPHER_LEN];
    openssl::rand::rand_bytes(&mut sym_key).unwrap();
    let enc_sym_key = Self::encrypt_sym_key(pin, &sym_key)?;
    let node = CryptoNode {
       sym_key: enc_sym_key.to_vec(),
       invitation_code: Vec::new(),
       eph: Vec::new()
    };
    Self::save_to_disc(&node)?;
    Ok(node)
  }

  fn load_config_from_disc() -> Result<Config> {
    let loc_share_dir  = Self::create_config_dir_if_needed()?;
    let config_path = loc_share_dir.as_path().join("config.pb");
    let mut config_file = File::options().read(true).open(config_path)?;
    let mut buf: Vec<u8> = Vec::new();
    config_file.read_to_end(&mut buf)?;
    Config::parse_from_bytes(&buf)
      .or(Err(Self::io_error("Could not load config file")))
  }

  fn save_to_disc(node: &CryptoNode) -> Result<()> {
    let mut config_file = Self::create_or_open_config()?;
    let conf = Config {
      sym_key: node.sym_key.clone(),
      generated: 0,
      special_fields: SpecialFields::default()
    };
    let mut v: Vec<u8> = Vec::new();
    conf.write_to_vec(&mut v)?;
    println!("Writing {} bytes to config file", v.len());
    config_file.write(&v)?;
    config_file.flush()?;
    Ok(())
  }

  fn create_or_open_config() -> Result<File> {
    let loc_share_dir  = Self::create_config_dir_if_needed()?;
    let config_path = loc_share_dir.as_path().join("config.pb");
    if config_path.exists() {
      File::options().write(true).open(config_path)
        .or(Err(Self::io_error("Could not open config path")))
    }
    else {
      File::create(config_path)
        .or(Err(Self::io_error("could not create config path")))
    }
  }

  fn create_config_dir_if_needed() -> Result<PathBuf> {
    let loc_share_dir = match dirs::home_dir() {
      Some(d) => d.as_path().join(".loc_share"),
      None => return Err(Error::new(ErrorKind::NotFound, "Could not get home dir"))
    };
    let loc_share_dir_str = loc_share_dir.to_str().unwrap();
    if ! std::fs::exists(&loc_share_dir)? {
      println!("Creating directory {}", loc_share_dir_str);
      std::fs::create_dir(&loc_share_dir)
        .or(Err(Error::new(ErrorKind::Other, "Could not create loc_share directory")))?;
    }
    Ok(loc_share_dir)
  }

  fn io_error(msg: &str) -> Error {
    Error::new(ErrorKind::NotFound, msg)
  }

  fn encrypt_sym_key(pin: &str, sym_key: &[u8]) -> Result<[u8; CIPHER_LEN + 8]> {
    let iv: [u8; 8] = [7; 8];
    let mut key = [0; CIPHER_LEN];
    pbkdf2_hmac(pin.as_bytes(), &iv, HMAC_ITER, MessageDigest::sha256(), &mut key)?;
    let ak: AesKey = AesKey::new_encrypt(&key).unwrap();
    let mut enc_buf = [0; CIPHER_LEN + 8];
    openssl::aes::wrap_key(&ak, Some(iv), &mut enc_buf, &sym_key)
      .or(Err(Error::new(ErrorKind::InvalidData, "Could not wrap key")))?;
    Ok(enc_buf)
  }

  fn decrypt_sym_key(pin: &str, sym_key: &[u8]) -> Result<[u8; 32]> {
    let iv: [u8; 8] = [7; 8];
    let mut key = [0; CIPHER_LEN];
    pbkdf2_hmac(pin.as_bytes(), &iv, HMAC_ITER, MessageDigest::sha256(), &mut key)?;

    let ak: AesKey = AesKey::new_decrypt(&key).unwrap();
    let mut dec_buf = [0; CIPHER_LEN];
    openssl::aes::unwrap_key(&ak, Some(iv), &mut dec_buf, sym_key)
      .or(Err(Error::new(ErrorKind::InvalidData, "Could not decipher sym key")))?;
    Ok(dec_buf)
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

  pub fn generate_random_invitation_code() -> Vec<u8> {
    return CryptoNode::random_bytes(16);
  }

  pub fn draw_ephemeral_key() -> Vec<u8> {
    return CryptoNode::random_bytes(16);
  }

  pub fn random_bytes(n: u16) -> Vec<u8> {
    return (0..n).map(|_| { random::<u8>() }).collect();
  }

  pub fn encrypt_ephemeral_key(eph_key: &Vec<u8>, inv_code: &Vec<u8>) -> Vec<u8> {
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
    let cnode = CryptoNode::new();
    let inv_code_vec = inv_code.to_vec();
    let eph_key = CryptoNode::draw_ephemeral_key();
    let enc_eph_key = CryptoNode::encrypt_ephemeral_key(&eph_key, &inv_code.to_vec());
  }
}