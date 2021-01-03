use crate::udp_node;
use crate::crypto_node;
use crate::connection_process;
use crypto_node::BigInt;
use crypto_node::CryptoNode;
use udp_node::UdpNode;
use connection_process::ConnectionProcess;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::Mutex;


const DEFAULT_PORT: u32 = 5555;

pub struct Node {
  pub udp: udp_node::UdpNode,
  pub crypto: crypto_node::CryptoNode,
}

impl Node {

  pub fn new(udp: udp_node::UdpNode, crypto: crypto_node::CryptoNode) -> Node {
    return Node{udp: udp, crypto: crypto};
  }

  // sends encrypted invitation code over broadcast
  // and returns unencrypted invitation code
  pub fn invite_new_user(&mut self) -> Vec<u8> {
    println!("invite_new_user");
    let ric = self.crypto.generate_random_invitation_code();
    self.crypto.generate_dh_keys();
    self.udp.prepare_broadcast_socket();
    let enc_pub_key = self.crypto.get_encrypted_pub_key_der(&ric);
    self.send_message(enc_pub_key);
    self.send_number(self.crypto.g);
    return ric;
  }

  // waits for encrypted invitation code and returns it
  pub fn start_connecting_to_existing_node(&self, port: u32) -> ConnectionProcess {
    println!("start_connecting_to_existing_node");
    let enc_pub_key: Vec<u8> = self.udp.receive_broadcast_data();
    //TODO: enc_g - change type
    let enc_g: BigInt = self.receive_broadcast_number();
    println!("Received encrypted pub_key: {:?}, g: {}", enc_pub_key, enc_g);
    return ConnectionProcess{
      invitation_code: [0;1].to_vec(),
      enc_pub_key: enc_pub_key,
      enc_g: enc_g,
    };
  }

  pub fn continue_connecting_to_node(&self, conn_proc: &mut ConnectionProcess, invitation_code: Vec<u8>) {
    println!("continue_connecting_to_node");
    conn_proc.invitation_code = invitation_code;
    // todo: decipher enc_pub_key and enc_g with invitation_code
  }

  fn send_number(&self, num: BigInt) {
    println!("Sending number: {}", num);
    let buf: String = num.to_string();
    self.udp.broadcast_message(buf.as_bytes(), DEFAULT_PORT);
  }

  fn send_message(&self, msg: Vec<u8>) {
    println!("Sending message: {:?}", msg);
    //TODO:
    //let buf: String = msg.to_string();
    //self.udp.broadcast_message(buf.as_bytes(), DEFAULT_PORT);
  }

  fn receive_broadcast_number(&self) -> BigInt {
    let str: String = self.udp.receive_broadcast_str();
    return str.trim().parse()
      .expect("Invalid number");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_new_node() {
    let cnode: CryptoNode = CryptoNode::generate();
    let mut new_node: Node = Node::new(
      UdpNode::new([127,0,0,1], [127,0,0,1]),
      cnode,
    );

    let uold_node = udp_node::UdpNode::new([127,0,0,1], [127,0,0,1]);
    let cold_node = crypto_node::CryptoNode::generate();
    let mut old_node: Node = Node{
        udp: uold_node,
        crypto: cold_node,
    };

    old_node.udp.prepare_broadcast_socket();
    new_node.udp.prepare_receiving_socket(5555);

    let mut conn_process2 = ConnectionProcess::new();
    let mut ric: Vec<u8> = [0;1].to_vec();

    let (sender, receiver) = channel();
    let mutex = Mutex::new(new_node);
    let counter = std::sync::Arc::new(mutex);
    {
      println!("counter.clone");
      let counter = counter.clone();
      let child_thread = thread::spawn(move || {
        let node_ref = counter.lock().unwrap();
        let conn_process = node_ref.start_connecting_to_existing_node(DEFAULT_PORT);
        sender.send(conn_process).unwrap();
      });
      ric = old_node.invite_new_user();
      conn_process2 = receiver.recv().unwrap();
      println!("Waiting for thread join");
      child_thread.join().unwrap();
    }

    let node_ref2 = counter.lock().unwrap();
    node_ref2.continue_connecting_to_node(&mut conn_process2, ric);
  }

}