use crate::udp_node;
use crate::crypto_node;
use std::thread;
use std::sync::mpsc::channel;

const DEFAULT_PORT: u32 = 5555;

pub struct Node {
  pub udp: udp_node::UdpNode,
  pub crypto: crypto_node::CryptoNode,
}

impl Node {

  // sends encrypted invitation code over broadcast
  // and returns unencrypted invitation code
  pub fn invite_new_user(&self) -> crypto_node::BigInt {
    let ric: crypto_node::BigInt = self.crypto.generate_random_inv_code();
    let buf: String = ric.to_string();
    self.udp.broadcast_message(buf.as_bytes(), DEFAULT_PORT);
    return ric;
  }

  // waits for encrypted invitation code and returns it
  pub fn connect_to_existing_node(&self, port: u32) -> crypto_node::BigInt {
    let ric_str: String = self.udp.receive_broadcast(port);
    //TODO: longer type
    let ric: crypto_node::BigInt = ric_str.trim().parse()
      .expect("Invalid random invitation code");
    return ric;
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
    let mut new_node: Node = Node{
      udp: unode2,
      crypto: cnode2,
    };

    let unode1 = udp_node::UdpNode::new([127,0,0,1], [127,0,0,1]);
    let cnode1 = crypto_node::CryptoNode{prv: 0, pub_key: 0, sym: 0, ric: 0};
    let node1: Node = Node{
        udp: unode1,
        crypto: cnode1,
    };

    let (ic, encrypted_ic) = send_invitation_code(node1, new_node);

    //assert_eq!(node1.crypto.sym, new_node.crypto.sym);
  }

  fn send_invitation_code(node1: Node, new_node: Node) -> (crypto_node::BigInt, crypto_node::BigInt) {
    let (sender, receiver) = channel();
    let child_thread = thread::spawn(move || {
      let encrypted_ric = new_node.connect_to_existing_node(DEFAULT_PORT);
      sender.send(encrypted_ric).unwrap();
    });

    let ric = node1.invite_new_user();
    let received_encrypted_ic = receiver.recv().unwrap();
    child_thread.join();
    return (ric, received_encrypted_ic);
  }
}