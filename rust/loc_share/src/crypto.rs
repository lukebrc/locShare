extern crate openssl;
//use openssl::rsa::{Rsa, Padding};
//use openssl::pkey::Private;
use openssl::aes::{AesKey, aes_ige};
use openssl::symm::Mode;
use openssl::symm::{encrypt, decrypt, Cipher};
extern crate hex;
use hex::{FromHex, ToHex};
  
pub fn aes128_encrypt(input: &Vec<u8>, key: &[u8]) -> Vec<u8> {
  let cipher = Cipher::aes_128_cbc(); //todo: cipher dependent of key size
  let encrypted = encrypt(cipher, key, Some(key), input).unwrap(); //TODO: currently iv=key, change it
  return encrypted;
}

pub fn aes128_decrypt(input: &Vec<u8>, key: &[u8]) -> Vec<u8> {
  let cipher = Cipher::aes_128_cbc(); //todo: cipher dependent of key size
  return decrypt(cipher, key, Some(key), &input).unwrap(); //TODO: currently iv=key, change it
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