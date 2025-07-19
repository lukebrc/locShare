extern crate hex;
extern crate dirs;
use std::fs::File;
use std::io::{Read, Write};
use protobuf::{Message, SpecialFields};
use crate::{connection_process, crypto, udp_node};
use std::io::{Result, Error, ErrorKind};
use std::path::PathBuf;
use crate::proto::pb::{Config, ConnectRequest};


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

  pub fn load_from_disc(config_dir: &PathBuf, pin: &str, inv_code: String) -> Result<CryptoNode> {
    let config = Self::load_config_from_disc(config_dir)?;
    let dec_buf = crypto::decrypt_sym_key(pin, &config.sym_key)?;

    Ok(CryptoNode {
      sym_key: dec_buf.to_vec(),
      invitation_code: inv_code,
      eph: Vec::new(),
      port: 5522,
    })
  }

  pub fn create_new(config_dir: &PathBuf, pin: &str, inv_code: String) -> Result<CryptoNode> {
    let mut sym_key = [0; crypto::CIPHER_LEN];
    openssl::rand::rand_bytes(&mut sym_key).unwrap();
    let enc_sym_key = crypto::encrypt_sym_key(pin, &sym_key)?;
    let node = CryptoNode {
       sym_key: enc_sym_key.to_vec(),
       invitation_code: inv_code,
       eph: Vec::new(),
       port: 5522
    };
    Self::save_to_disc(config_dir, &node)?;
    Ok(node)
  }

  pub fn listen(self, unode: &mut udp_node::UdpNode) -> std::result::Result<(), Box<dyn std::error::Error> > {
    unode.prepare_receiving_socket(5522);
    let enc_connect_msg = unode.receive_broadcast_data();
    let inv_code = hex::decode(self.invitation_code)?;
    let connect_msg = crypto::decrypt_msg(&enc_connect_msg, &inv_code)?;
    let request = ConnectRequest::parse_from_bytes(&connect_msg)?;
    let mac = crypto::compute_mac(&request.eph_key);

    if request.inv_code_mac != mac {
      println!("Invitation code does not match");
    }
    Ok(())
  }

  pub fn connect(self, unode: &mut udp_node::UdpNode, inv_code_hex: String) -> std::result::Result<(), Box<dyn std::error::Error> > {
    unode.prepare_broadcast_socket();

    let enc_con_req = Self::create_encrypted_connect_request(inv_code_hex)?;
    unode.broadcast_message(&enc_con_req, self.port)?;
    let mut buf  = [0u8; 1024];
    let answer = unode.receive_data(&mut buf)?;
    Ok(())
  }

  pub fn create_encrypted_connect_request(inv_code_hex: String) -> crypto::EResult< Vec<u8> > {
    let eph_key = Self::draw_ephemeral_key();
    let con_req = Self::create_connect_request(eph_key);
    Self::encrypt_connect_request(&con_req, inv_code_hex)
  }

  pub fn create_connect_request(eph_key: Vec<u8>) -> ConnectRequest {
    let mac = crypto::compute_mac(&eph_key);
    ConnectRequest {
      eph_key: eph_key.to_vec(),
      inv_code_mac: mac.to_vec(),
      special_fields: SpecialFields::default()
    }
  }

  pub fn encrypt_connect_request(msg: &ConnectRequest, inv_code_hex: String) -> crypto::EResult<Vec<u8>> {
    let mut v: Vec<u8> = Vec::new();
    msg.write_to_vec(&mut v)?;
    let k = hex::decode(inv_code_hex).unwrap();
    crypto::encrypt_msg(&v, &k)
  }

  pub fn draw_ephemeral_key() -> Vec<u8> {
    let mut eph_key= [0; crypto::CIPHER_LEN];
    openssl::rand::rand_bytes(&mut eph_key).unwrap();
    eph_key.to_vec()
  }

  pub fn generate_random_invitation_code() -> String {
    let bytes = crypto::random_bytes(8);
    hex::encode(bytes)
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
      generated: 0, //todo: time
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
    let inv_code= "12345678".to_string();
    let eph_key = CryptoNode::draw_ephemeral_key();
    let cnode = CryptoNode {
      sym_key: sym_key.to_vec(),
      invitation_code: inv_code.clone(),
      eph: eph_key,
      port: 0
    };
    let encrypted_req = CryptoNode::create_encrypted_connect_request(inv_code);
  }
}
