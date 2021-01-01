use crate::udp_node;
use crate::crypto_node;
use crate::connection_process;
use crypto_node::BigInt;
use crypto_node::CryptoNode;
use udp_node::UdpNode;
use connection_process::ConnectionProcess;
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
  pub fn invite_new_user(&mut self) -> BigInt {
    println!("invite_new_user");
    let ric: BigInt = self.crypto.generate_random_inv_code();
    self.crypto.generate_dh_keys();
    self.udp.prepare_broadcast_socket();
    self.send_number(self.crypto.pub_key);
    self.send_number(self.crypto.g);
    return ric;
  }

  // waits for encrypted invitation code and returns it
  pub fn start_connecting_to_existing_node(&self, port: u32) -> ConnectionProcess {
    let enc_pub_key: BigInt = self.receive_broadcast_number();
    let enc_g: BigInt = self.receive_broadcast_number();
    println!("Received encrypted pub_key: {}, g: {}", enc_pub_key, enc_g);
    return ConnectionProcess{
      invitation_code: 0,
      enc_pub_key: enc_pub_key,
      enc_g: enc_g,
    };
  }

  pub fn continue_connecting_to_node(&self, conn_proc: &mut ConnectionProcess, invitation_code: BigInt) {
    conn_proc.invitation_code = invitation_code;
    // todo: decipher enc_pub_key and enc_g with invitation_code
  }

  pub fn new(udp: udp_node::UdpNode, crypto: crypto_node::CryptoNode) -> Node {
    return Node{udp: udp, crypto: crypto};
  }

  fn send_number(&self, num: BigInt) {
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
    static mut new_node: Node = Node{
      udp: UdpNode::new([127,0,0,1], [127,0,0,1]),
      crypto: CryptoNode::new(),
    };

    let uold_node = udp_node::UdpNode::new([127,0,0,1], [127,0,0,1]);
    let cold_node = crypto_node::CryptoNode::new();
    let mut old_node: Node = Node{
        udp: uold_node,
        crypto: cold_node,
    };

    old_node.udp.prepare_broadcast_socket();
    unsafe {
      new_node.udp.prepare_receiving_socket(5555);
      let (mut conn_proc, inv_code) = send_invitation_code(old_node, &new_node);
      new_node.continue_connecting_to_node(&mut conn_proc, inv_code);
    }

    //assert_eq!(old_node.crypto.sym, new_node.crypto.sym);
  }

  fn send_invitation_code(mut old_node: Node, new_node: &'static Node) -> (ConnectionProcess, BigInt) {
    let (sender, receiver) = channel();
    let child_thread = thread::spawn(move || {
      let conn_process = new_node.start_connecting_to_existing_node(DEFAULT_PORT);
      sender.send(conn_process).unwrap();
    });

    let ric = old_node.invite_new_user();
    let conn_process2 = receiver.recv().unwrap();
    child_thread.join().unwrap();
    return (conn_process2, ric);
  }
}