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
  pub udp: UdpNode,
  pub crypto: CryptoNode,
}

impl Node {

  pub fn new(udp: UdpNode, crypto: CryptoNode) -> Node {
    return Node{udp, crypto };
  }

  pub fn invite_new_user(&mut self) {
    println!("invite_new_user - waiting for broadcast data");
    let enc_eph = self.udp.receive_broadcast_data();
    println!("Received encrypted eph {:?}", enc_eph);
  }

  // waits for encrypted invitation code and returns it
  pub fn start_connecting_to_existing_node(&mut self, inv_code: &String) {
    println!("start_connecting_to_existing_node");
    self.crypto.invitation_code = inv_code.clone();
    self.udp.prepare_broadcast_socket();
    // self.crypto.eph = CryptoNode::draw_ephemeral_key();
    // let encrypted_eph = self.crypto.encrypt_ephemeral_key(inv_code);
    // // let broadcast_code = BroadcastCode::new(&enc_pub_key);
    // println!("broadcasting message of {} bytes", self.crypto.eph.len());
    // self.udp.broadcast_message(&encrypted_eph, DEFAULT_PORT);
    //println!("Received encrypted pub_key: {:?}", enc_pub_key);
    //TODO: check broadcast CRC code
    // return broadcast_code.encrypted_msg;
  }

  // pub fn continue_connecting_to_node(&mut self, enc_eph_key: &Vec<u8>, invitation_code: &String) -> Vec<u8> {
  //   println!("continue_connecting_to_node");
  //   self.crypto.invitation_code = invitation_code.clone();
  //   return self.crypto.decrypt_ephmemeral_key(&enc_eph_key, invitation_code);
  // }

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

  fn receive_broadcast_message(&self) -> Vec<u8> {
    return self.udp.receive_broadcast_data();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_new_node() {
    let sym_key= b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
    let inv_code= "12345678".to_string();
    let eph_key = CryptoNode::draw_ephemeral_key();
    let cnode = CryptoNode {
      sym_key: sym_key.to_vec(),
      invitation_code: inv_code.clone(),
      eph: eph_key.clone(),
      port: 0
    };

    let old_udp_node = UdpNode::new([127,0,0,1], [127,0,0,1]);

    let old_crypto_node = CryptoNode {
      sym_key: sym_key.to_vec(),
      invitation_code: inv_code.clone(),
      eph: eph_key.clone(),
      port: 0
    };

    let mut old_node: Node = Node{
        udp: old_udp_node,
        crypto: old_crypto_node,
    };

    //old_node.udp.prepare_broadcast_socket();
    let invitation_code = CryptoNode::generate_random_invitation_code();
    println!("Generated random invitation code {:?}", invitation_code);
    old_node.crypto.invitation_code = invitation_code.clone();
    old_node.udp.prepare_receiving_socket(DEFAULT_PORT);

    let receiver = new_node_thread(cnode, &invitation_code);

    old_node.invite_new_user();
    println!("Waiting for new node end");
    let new_node = receiver.recv().unwrap();
  }

  fn new_node_thread(cnode: CryptoNode, invitation_code: &String) -> std::sync::mpsc::Receiver<Node> {
    let (sender, receiver) = channel();
    let inv_code_copy = invitation_code.clone();
    let child_thread = thread::spawn(move || {
      let mut new_node: Node = Node::new(
        UdpNode::new([127,0,0,1], [127,0,0,1]),
        cnode,
      );
      //new_node.udp.prepare_receiving_socket(DEFAULT_PORT);

      // new_node.start_connecting_to_existing_node(&inv_code_copy);

      // let enc_eph_key = new_node.receive_broadcast_message();
      // println!("Received enc_eph_key {:?}", enc_eph_key);
      // //conn_process.decrypted_eph_key = new_node.continue_connecting_to_node(&mut conn_process);
      // let decrypted_eph_key = new_node.continue_connecting_to_node(&enc_eph_key, &inv_code_copy);
      // sender.send(new_node).unwrap();
    });

    return receiver;
  }

}