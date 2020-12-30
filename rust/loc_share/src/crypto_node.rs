type BigInt = i128;

pub struct CryptoNode {
  pub prv: BigInt,
  pub pub_key: BigInt,
  pub sym: BigInt,
  pub ric: BigInt,
  //TODO: time of ric
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

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_new_node() {
    let node = CryptoNode{prv: 0, pub_key: 0, sym: 0, ric: 0};
    let new_node = CryptoNode{prv: 0, pub_key: 0, sym: 0, ric: 0};
    let ric = node.generate_random_inv_code();

    assert_eq!(node.sym, new_node.sym);
  }
}