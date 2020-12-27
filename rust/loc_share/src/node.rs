type BigInt = i128;

struct Node {
  prv: BigInt,
  pub: BigInt,
  sym: BigInt,
  ric: BigInt,
  //TODO: time of ric
}

impl Node {

  fn generate_ric(&self) -> BigInt {
    //TODO:
    return 1;
  }

  fn generate_public_message(&self, ic: &BigInt) -> BigInt {
    //TODO:
    return 2;
  }

  fn generate_encrypted_secret_message(&self, x2: &BigInt) -> BigInt {
    //TODO:
    return 3;
  }

}
