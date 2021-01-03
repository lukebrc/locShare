use crate::crypto_node;
use crate::messages;
use messages::BroadcastCode;

pub struct ConnectionProcess {
  pub broadcast_code: BroadcastCode,
  pub invitation_code: Vec<u8>,
}

impl ConnectionProcess {

  pub fn new(broadcast_code: BroadcastCode) -> ConnectionProcess {
    return ConnectionProcess{
      broadcast_code: broadcast_code,
      invitation_code: Vec::new(),
    };
  }

}