use crypto_node::BigInt;

pub struct ConnectionProcess {
  pub invitation_code: [u8];
  pub enc_pub_key: BigInt,
  pub enc_g: BigInt,
}

impl ConnectionProcess {

  fn setInvitationCode(&self, ic: &[u8]) {
    //TODO:

  }

}