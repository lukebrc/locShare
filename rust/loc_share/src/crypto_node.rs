extern crate hex;
extern crate dirs;
use std::fs::File;
use std::io::{Read, Write};
use openssl::symm::Cipher;
use openssl::{hash::MessageDigest, pkcs5::pbkdf2_hmac};
use openssl::cipher::Cipher;

use openssl::aes::AesKey;
use protobuf::{Message, SpecialFields};
use rand::{Rng, random};
use crate::{connection_process, crypto, udp_node};
use crypto::{aes128_encrypt, aes128_decrypt};
use std::io::{Result, Error, ErrorKind};
use std::path::PathBuf;
use crate::proto::pb::{Config, ConnectRequest};

const CIPHER_LEN: usize = 32;
const HMAC_ITER: usize = 1024;

pub struct CryptoNode {
  pub sym_key: Vec<u8>,
  pub invitation_code: String,
  pub eph: Vec<u8>,
  pub port: u32,
  //pub g: Vec<u8>,
}

impl CryptoNode {

  pub fn has_config(config_dir: &PathBuf) -> bool {
    match Self::load_config_from_disc(config_dir) {
      Ok(_) => true,
      Err(_) => false
    }
  }

  pub fn load_from_disc(config_dir: &PathBuf, pin: &str) -> Result<CryptoNode> {
    let config = Self::load_config_from_disc(config_dir)?;
    let dec_buf = Self::decrypt_sym_key(pin, &config.sym_key)?;

    Ok(CryptoNode {
      sym_key: dec_buf.to_vec(),
      invitation_code: String::new(),
      eph: Vec::new(),
      port: 5522,
    })
  }

  pub fn create_new(config_dir: &PathBuf, pin: &str) -> Result<CryptoNode> {
    let mut sym_key = [0; CIPHER_LEN];
    openssl::rand::rand_bytes(&mut sym_key).unwrap();
    let enc_sym_key = Self::encrypt_sym_key(pin, &sym_key)?;
    let node = CryptoNode {
       sym_key: enc_sym_key.to_vec(),
       invitation_code: String::new(),
       eph: Vec::new(),
       port: 5522
    };
    Self::save_to_disc(config_dir, &node)?;
    Ok(node)
  }

  pub fn listen(self, unode: &mut udp_node::UdpNode) -> Result<()> {
    let iv: [u8; 8] = [7; 8];
    unode.prepare_receiving_socket(5522);
    let enc_connect_msg = unode.receive_broadcast_data();
    let connect_msg = openssl::symm::decrypt(Cipher::aes_256_cbc(), &self.invitation_code.as_bytes(), None, &enc_connect_msg)?;
    let request = ConnectRequest::parse_from_bytes(&connect_msg)?;
    if request.inv_code != self.invitation_code {
      println!("Invitation code does not match");
    }
    Ok(())
  }

  pub fn connect(self, unode: &mut udp_node::UdpNode, inv_code: String) -> Result<()> {
    let iv: [u8; 8] = [7; 8];
    unode.prepare_broadcast_socket();

    let mut eph_key= [0; CIPHER_LEN];
    openssl::rand::rand_bytes(&mut eph_key).unwrap();

    let msg = ConnectRequest {
      eph_key: eph_key.to_vec(),
      inv_code: inv_code.clone(),
      special_fields: SpecialFields::default()
    };
    let mut v: Vec<u8> = Vec::new();
    msg.write_to_vec(&mut v)?;
    let enc_msg= openssl::symm::encrypt(Cipher::aes_256_cbc(), inv_code.as_bytes(), None, &v)?;
    unode.broadcast_message(&enc_msg, self.port)?;
    let mut buf  = [0u8; 1024];
    let answer = unode.receive_data(&mut buf)?;
    Ok(())
  }

  pub fn generate_random_invitation_code() -> Vec<u8> {
    return CryptoNode::random_bytes(16);
  }

  pub fn draw_ephemeral_key() -> Vec<u8> {
    return CryptoNode::random_bytes(16);
  }

  pub fn random_bytes(n: u16) -> Vec<u8> {
    return (0..n).map(|_| { random::<u8>() }).collect();
  }

  pub fn encrypt_ephemeral_key(&self, inv_code: &Vec<u8>) -> Vec<u8> {
    return aes128_encrypt(&self.eph, inv_code);
  }

  pub fn decrypt_ephmemeral_key(&self, eph_key: &Vec<u8>, inv_code: &Vec<u8>) -> Vec<u8> {
    //TODO: check conn_proc.broadcast_code CRC
    println!("lengths: {}, {}", eph_key.len(), inv_code.len());
    let decrypted = aes128_decrypt(&eph_key, &inv_code);
    return decrypted;
  }

  fn load_config_from_disc(config_dir: &PathBuf) -> Result<Config> {
    let config_path = config_dir.as_path().join("config.pb");
    let mut config_file = File::options().read(true).open(config_path)?;
    let mut buf: Vec<u8> = Vec::new();
    config_file.read_to_end(&mut buf)?;
    Config::parse_from_bytes(&buf)
      .or(Err(Self::io_error("Could not load config file")))
  }

  fn save_to_disc(config_dir: &PathBuf, node: &CryptoNode) -> Result<()> {
    let mut config_file = Self::create_or_open_config(config_dir)?;
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

  fn create_or_open_config(config_dir: &PathBuf) -> Result<File> {
    let config_path = config_dir.as_path().join("config.pb");
    if config_path.exists() {
      File::options().write(true).open(config_path)
        .or(Err(Self::io_error("Could not open config path")))
    }
    else {
      File::create(config_path)
        .or(Err(Self::io_error("could not create config path")))
    }
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

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_draw_and_encrypt_ephemeral_key() {
    let sym_key= b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
    let inv_code= b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
    let eph_key = CryptoNode::draw_ephemeral_key();
    let inv_code_vec = inv_code.to_vec();
    let cnode = CryptoNode {
      sym_key: sym_key.to_vec(),
      invitation_code: inv_code_vec,
      eph: eph_key,
    };
    let enc_eph_key = cnode.encrypt_ephemeral_key(&inv_code.to_vec());
  }
}
