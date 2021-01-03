use crate::crypto_node;
use crypto_node::BigInt;

pub struct ConnectionProcess {
  pub invitation_code: Vec<u8>,
  pub enc_pub_key: Vec<u8>,
  pub enc_g: BigInt,
}

impl ConnectionProcess {

  pub const fn new() -> ConnectionProcess {
    ConnectionProcess{
      invitation_code: Vec::new(),
      enc_pub_key: Vec::new(),
      enc_g: 0,
    }
  }
  // pub fn setInvitationCode(&self, ic: &[u8]) {
  //   //TODO:

  // }

}