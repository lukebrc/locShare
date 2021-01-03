pub trait Message {
  fn serialize(&self) -> Vec<u8>;
}

pub struct BroadcastCode {
  pub enc_pub_key: Vec<u8>,
  pub enc_dh_group: Vec<u8>,
  crc: u32,
}

impl BroadcastCode {
  pub fn new(enc_pub_key: Vec<u8>, dh_group: Vec<u8>) -> BroadcastCode {
    let crc: u32 = 0; //TODO:
    return BroadcastCode{
      enc_pub_key: enc_pub_key,
      enc_dh_group: dh_group,
      crc: crc,
    }
  }
}

impl Message for BroadcastCode {
  fn serialize(&self) -> Vec<u8> {
    //TODO:
    return Vec::new();
  }

}