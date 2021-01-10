extern crate hex;
use hex::FromHex;

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
    //TODO: randomize
    let ic_hex = "0123456789ABCDEF";
    return Vec::from_hex(ic_hex).unwrap();
  }

  pub fn draw_and_encrypt_ephemeral_key(&self, inv_code: &Vec<u8>) -> Vec<u8> {
    //TODO:
    return Vec::new();
  }

}

// #[cfg(test)]
// mod tests {
//   use super::*;
// }