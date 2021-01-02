use crate::crypto_node;
use crypto_node::BigInt;

pub struct ConnectionProcess {
  pub invitation_code: BigInt,
  pub enc_pub_key: BigInt,
  pub enc_g: BigInt,
}

impl ConnectionProcess {

  pub const fn new() -> ConnectionProcess {
    ConnectionProcess{
      invitation_code: 0,
      enc_pub_key: 0,
      enc_g: 0,
    }
  }
  // pub fn setInvitationCode(&self, ic: &[u8]) {
  //   //TODO:

  // }

}