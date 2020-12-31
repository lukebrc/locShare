pub type BigInt = i128;

pub struct CryptoNode {
  pub prv: BigInt,
  pub pub_key: BigInt,
  pub sym: BigInt,
  pub ric: BigInt,
  //TODO: time of ric
  pub g: BigInt,
}

impl CryptoNode {

  pub fn generate_random_inv_code(&self) -> BigInt {
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

  pub fn new() -> CryptoNode {
    return CryptoNode{
      prv: 0,
      pub_key: 0,
      sym: 0,
      ric: 0,
      g: 0,
    }
  }

}
