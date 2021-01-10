pub trait Message {
  fn serialize(&self) -> Vec<u8>;
}

pub struct BroadcastCode {
  pub encrypted_msg: Vec<u8>,
  crc: u32,
}

impl BroadcastCode {
  pub fn new(encrypted_msg: &Vec<u8>) -> BroadcastCode {
    let crc: u32 = 0; //TODO:
    return BroadcastCode{
      encrypted_msg: encrypted_msg.clone(),
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