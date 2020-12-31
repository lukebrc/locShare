use crate::udp_node;
use crate::crypto_node;
use crypto_node::BigInt;
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
  pub fn invite_new_user(&mut self) -> crypto_node::BigInt {
    println!("invite_new_user");
    let ric: crypto_node::BigInt = self.crypto.generate_random_inv_code();
    self.crypto.generate_dh_keys();
    self.udp.prepare_broadcast_socket();
    self.send_number(self.crypto.pub_key);
    self.send_number(self.crypto.g);
    return ric;
  }

  // waits for encrypted invitation code and returns it
  pub fn connect_to_existing_node(&mut self, port: u32) -> (BigInt, BigInt) {
    self.udp.prepare_receiving_socket(port);
    let enc_pub_key: crypto_node::BigInt = self.receive_broadcast_number();
    let enc_g: BigInt = self.receive_broadcast_number();
    println!("Received encrypted pub_key: {}, g: {}", enc_pub_key, enc_g);

    return (enc_pub_key, enc_g);
  }

  pub fn new(udp: udp_node::UdpNode, crypto: crypto_node::CryptoNode) -> Node {
    return Node{udp: udp, crypto: crypto};
  }

  fn send_number(&self, num: crypto_node::BigInt) {
    println!("Sending number: {}", num);
    let buf: String = num.to_string();
    self.udp.broadcast_message(buf.as_bytes(), DEFAULT_PORT);
  }

  fn receive_broadcast_number(&self) -> BigInt {
    let str: String = self.udp.receive_broadcast();
    return str.trim().parse()
      .expect("Invalid number");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_new_node() {
    let unode2 = udp_node::UdpNode::new([127,0,0,1], [127,0,0,1]);
    let cnode2 = crypto_node::CryptoNode::new();
    let new_node: Node = Node{
      udp: unode2,
      crypto: cnode2,
    };

    let unode1 = udp_node::UdpNode::new([127,0,0,1], [127,0,0,1]);
    let cnode1 = crypto_node::CryptoNode::new();
    let node1: Node = Node{
        udp: unode1,
        crypto: cnode1,
    };

    let (ic, enc_pub_key, enc_g) = send_invitation_code(node1, new_node);

    //assert_eq!(node1.crypto.sym, new_node.crypto.sym);
  }

  fn send_invitation_code(mut node1: Node, mut new_node: Node) -> (BigInt, BigInt, BigInt) {
    let (sender, receiver) = channel();
    let child_thread = thread::spawn(move || {
      let encrypted_ric = new_node.connect_to_existing_node(DEFAULT_PORT);
      sender.send(encrypted_ric).unwrap();
    });

    let ric = node1.invite_new_user();
    let (enc_pub_key, enc_g)= receiver.recv().unwrap();
    child_thread.join().unwrap();
    return (ric, enc_pub_key, enc_g);
  }
}