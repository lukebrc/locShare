use crate::udp_node;
use crate::crypto_node;

const DEFAULT_PORT: u32 = 5555;

pub struct Node {
  pub udp: udp_node::UdpNode,
  pub crypto: crypto_node::CryptoNode,
}

impl Node {
  pub fn invite_new_user(&self) -> crypto_node::BigInt {
    let ric: crypto_node::BigInt = self.crypto.generate_random_inv_code();
    let buf: String = ric.to_string();
    self.udp.broadcast_message(buf.as_bytes(), DEFAULT_PORT);
    return ric;
  }

  pub fn connect_to_existing_node(&self, port: u32) {
    let ric_str: String = self.udp.receive_broadcast(port);
    //TODO: longer type
    let ric: u32 = ric_str.trim().parse()
      .expect("Invalid random invitation code");
  }

  pub fn new(udp: udp_node::UdpNode, crypto: crypto_node::CryptoNode) -> Node {
    return Node{udp: udp, crypto: crypto};
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_new_node() {
    let unode2 = udp_node::UdpNode::new([127,0,0,1], [127,0,0,1]);
    let cnode2 = crypto_node::CryptoNode{prv: 0, pub_key: 0, sym: 0, ric: 0};
    let new_node: Node = Node{
      udp: unode2,
      crypto: cnode2,
    };
    new_node.connect_to_existing_node(DEFAULT_PORT);

    let unode1 = udp_node::UdpNode::new([127,0,0,1], [127,0,0,1]);
    let cnode1 = crypto_node::CryptoNode{prv: 0, pub_key: 0, sym: 0, ric: 0};
    let node1: Node = Node{
        udp: unode1,
        crypto: cnode1,
    };
    let ric = node1.invite_new_user();

    assert_eq!(node1.crypto.sym, new_node.crypto.sym);
  }
}