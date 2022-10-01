use crate::udp_node;
use crate::crypto_node;
use crate::connection_process;
use crate::messages;
use crate::crypto;

use crypto_node::CryptoNode;
use udp_node::UdpNode;
use connection_process::ConnectionProcess;
use messages::BroadcastCode;

use std::{thread, u128};
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
  pub fn invite_new_user(&mut self) -> (Vec<u8>, Vec<u8>) {
    println!("invite_new_user");
    let inv_code = self.crypto.generate_random_invitation_code();
    self.udp.prepare_broadcast_socket();
    let eph_key = self.crypto.draw_ephemeral_key();
    let enc_eph_key = self.crypto.encrypt_ephemeral_key(&eph_key, &inv_code);
    let invitation_msg = BroadcastCode::new(&enc_eph_key);
    self.send_message(&invitation_msg);
    return (inv_code, eph_key);
  }

  // waits for encrypted invitation code and returns it
  pub fn start_connecting_to_existing_node(&self, port: u32) -> ConnectionProcess {
    println!("start_connecting_to_existing_node");
    let enc_pub_key: Vec<u8> = self.udp.receive_broadcast_data();
    println!("Received encrypted pub_key: {:?}", enc_pub_key);
    let broadcast_code = BroadcastCode::new(&enc_pub_key);
    return ConnectionProcess::new(broadcast_code);
  }

  pub fn continue_connecting_to_node(&self, encrypted_msg: &Vec<u8>,
                                     invitation_code: &Vec<u8>) -> Vec<u8> {
    println!("continue_connecting_to_node");
    return self.crypto.decrypt_ephmemeral_key(&encrypted_msg, invitation_code);
  }

  fn send_number(&self, num: u128) {
    println!("Sending number: {}", num);
    let buf: String = num.to_string();
    self.udp.broadcast_message(buf.as_bytes(), DEFAULT_PORT);
  }

  fn send_message(&self, msg: &messages::Message) {
    println!("Sending message:");
    let buf = msg.serialize();
    self.udp.broadcast_message(&buf, DEFAULT_PORT);
  }

  fn receive_broadcast_number(&self) -> u128 {
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

    let old_udp_node = udp_node::UdpNode::new([127,0,0,1], [127,0,0,1]);
    let old_crypto_node = crypto_node::CryptoNode::generate();
    let mut old_node: Node = Node{
        udp: old_udp_node,
        crypto: old_crypto_node,
    };

    old_node.udp.prepare_broadcast_socket();

    let receiver = new_node_thread(cnode);
    let (rand_inv_code, eph_code) = old_node.invite_new_user();
    let new_node = receiver.recv().unwrap();
  }

  fn new_node_thread(cnode: CryptoNode) -> std::sync::mpsc::Receiver<Node> {
    let (sender, receiver) = channel();
    let child_thread = thread::spawn(move || {
      let mut new_node: Node = Node::new(
        UdpNode::new([127,0,0,1], [127,0,0,1]),
        cnode,
      );
      new_node.udp.prepare_receiving_socket(5555);

      let mut conn_process = new_node.start_connecting_to_existing_node(DEFAULT_PORT);
      let encrypted_msg = &conn_process.broadcast_code.encrypted_msg;
      //TODO: listen for out-of-band invication_code
      let invitation_code: Vec<u8> = vec![0,0,0];

      conn_process.decrypted_eph_key = new_node.continue_connecting_to_node(encrypted_msg, &invitation_code);
      sender.send(new_node).unwrap();
    });

    return receiver;
  }

}